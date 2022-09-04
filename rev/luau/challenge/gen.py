flag = list(b"CakeCTF{w4n1w4n1_p4n1c_uh0uh0_g0ll1r4}")
key = list(b"CakeCTF 2022")

for i in range(len(flag)):
    for j in range(i+1, len(flag)):
        flag[i], flag[j] = flag[j], flag[i]

for i in range(len(flag)):
    flag[i] ^= key[i % len(key)]

print(flag)
