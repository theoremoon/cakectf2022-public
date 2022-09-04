from Crypto.Util.number import inverse

key, c = open("output.txt").read().strip().split("\n")

s, t, n = eval(key)
c = eval(c)

flag = []
for (c1, c2) in c:
    for m in range(0x20, 0x7f):
        rs = c1 * inverse(m, n) % n
        rst = pow(rs, t, n)
        c2s = pow(c2, s, n)

        ms = c2s * inverse(rst, n) % n
        if pow(m, s, n) == ms:
            flag.append(m)
            break
    else:
        raise ValueError("XP")
    print(bytes(flag))


        


