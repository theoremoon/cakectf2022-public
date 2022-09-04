from Crypto.Util.number import inverse

with open("output.txt") as f:
    n = int(f.readline().strip().split(" = ")[1])
    a = int(f.readline().strip().split(" = ")[1])
    b = int(f.readline().strip().split(" = ")[1])
    c = int(f.readline().strip().split(" = ")[1])

m = inverse(c * inverse(a*b, n) % n, n)
print(bytes.fromhex(hex(m)[2:]))
