from scapy.all import *

keycodes = [
    "RESERVED", "ESC",
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
    "MINUS", "EQUAL", "BACKSPACE", "TAB",
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P",
    "LEFTBRACE", "RIGHTBRACE", "ENTER", "LEFTCTRL",
    "A", "S", "D", "F", "G", "H", "J", "K", "L",
    "SEMICOLON", "APOSTROPHE", "GRAVE", "LEFTSHIFT", "BACKSLASH",
    "Z", "X", "C", "V", "B", "N", "M",
    "COMMA", "DOT", "SLASH", "RIGHTSHIFT", "KPASTERISK",
    "LEFTALT", "SPACE", "CAPSLOCK",
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10",
    "NUMLOCK", "SCROLLLOCK"
]

packets = rdpcap("../distfiles/evidence.pcapng")

log = ""
shift = False
for packet in packets:
    if TCP in packet and packet[TCP].dport == 6379 and Raw in packet[TCP]:
        load = packet[TCP][Raw].load
        if b'$17' in load:
            code = int.from_bytes(load[-5:-3], 'little')
            value = load[-3]
            if code < len(keycodes):
                if value == 0:
                    if "SHIFT" in keycodes[code]:
                        shift = False

                elif value == 1:
                    if "SHIFT" in keycodes[code]:
                        shift = True
                    else:
                        c = keycodes[code]
                        if len(c) == 1:
                            log += c if shift else c.lower()
                        else:
                            log += "<" + c + ">"

                elif value == 2:
                    log += "[" + c + "]"

            else:
                if value == 1:
                    if code == 89:
                        log += "_"
                    else:
                        log += "<?>"

print(log)
