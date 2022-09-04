from ptrlib import Socket
from hashlib import sha256
from Crypto.Util.number import long_to_bytes, inverse

def thues_lemma(a, mod):
    R = Zmod(mod)
    A = matrix([
        [1,   a],
        [0, mod]
    ])
    A = A.LLL()
    return int(abs(A[0][0])), int(abs(A[0][1]))

def h(s: bytes) -> int:
    return int(sha256(s).hexdigest(), 16)


g = 2
def sign(m: bytes):
    z = h(m)
    k = h(long_to_bytes(x + z))
    r = h(long_to_bytes(pow(g, k, p)))
    s = (z + x*r) * inverse(k, q) % q
    return r, s

q = 139595134938137125662213161156181357366667733392586047467709957620975239424132898952897224429799258317678109670496340581564934129688935033567814222358970953132902736791312678038626149091324686081666262178316573026988062772862825383991902447196467669508878604109723523126621328465807542441829202048500549865003
p = 2*q + 1

sock = Socket("localhost", 9999)
m = b"the quick brown fox"

sock.sendlineafter("m = ", m)
# r = int(sock.recvlineafter("r = "))
s = int(sock.recvlineafter("s = "))

t, k = thues_lemma(s, q)
if k > t:
    t, k = k, t
r = h(long_to_bytes(pow(g, k, p)))
x = (t - h(m)) * inverse(r, q) % q

m2 = b"hirake goma"
r2, s2 = sign(m2)

sock.sendlineafter("m = ", m2)
sock.sendlineafter("r = ", str(r2))
sock.sendlineafter("s = ", str(s2))

print(sock.recvline())
