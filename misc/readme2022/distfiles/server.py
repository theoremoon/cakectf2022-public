import os

try:
    f = open("/flag.txt", "r")
except:
    print("[-] Flag not found. If this message shows up")
    print("    on the remote server, please report to amdin.")

if __name__ == '__main__':
    filepath = input("filepath: ")
    if filepath.startswith("/"):
        exit("[-] Filepath must not start with '/'")
    elif '..' in filepath:
        exit("[-] Filepath must not contain '..'")

    filepath = os.path.expanduser(filepath)
    try:
        print(open(filepath, "r").read())
    except:
        exit("[-] Could not open file")
