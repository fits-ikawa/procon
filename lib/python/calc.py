from math import log10, log2, factorial, comb, perm, gcd, lcm


def fact(n: int) -> int:
    return factorial(n)


# def comb(n: int, r: int) -> int:
#     assert n >= r, "n must be greater than or equal to r"
#     r = min(r, n - r)
#     result = 1
#     for i in range(r):
#         result = result * (n - i) // (i + 1)
#     return result


def repcomb(n: int, r: int) -> int:
    return comb(n + r - 1, r)


# def perm(n: int, r: int) -> int:
#     assert n >= r, "n must be greater than or equal to r"
#     return prod(range(n - r + 1, n + 1))


def repperm(n: int, r: int) -> int:
    return n**r


def recon(n, modulus):
    n = int(n)
    v = (modulus, 0)
    w = (n, 1)

    while w[0] * w[0] * 2 > modulus:
        q = v[0] // w[0]
        z = (v[0] - q * w[0], v[1] - q * w[1])
        v = w
        w = z

    if w[1] < 0:
        w = (-w[0], -w[1])

    return w


def recon_998244353(n):
    return recon(n, 998244353)


def recon_1000000007(n):
    return recon(n, 1000000007)
