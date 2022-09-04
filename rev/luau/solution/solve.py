enc = [62, 85, 25, 84, 47, 56, 118, 71, 109, 0, 90, 71, 115, 9, 30, 58, 32, 101, 40, 20, 66, 111, 3, 92, 119, 22, 90, 11, 119, 35, 61, 102, 102, 115, 87, 89, 34, 34]
key = list(b"CakeCTF 2022")

for i in range(len(enc)):
    enc[i] ^= key[i % len(key)]

for i in range(len(enc)-1, -1, -1):
    for j in range(len(enc)-1, i, -1):
        enc[i], enc[j] = enc[j], enc[i]

flag = ''
for c in enc:
    flag += chr(c)
print(flag)
