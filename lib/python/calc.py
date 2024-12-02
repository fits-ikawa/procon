import math


def fact(n: int) -> int:
    return math.factorial(n)


def comb(n: int, r: int) -> int:
    assert n >= r, "n must be greater than or equal to r"
    r = min(r, n - r)
    result = 1
    for i in range(r):
        result = result * (n - i) // (i + 1)
    return result


def repcomb(n: int, r: int) -> int:
    return comb(n + r - 1, r)


def perm(n: int, r: int) -> int:
    assert n >= r, "n must be greater than or equal to r"
    return math.prod(range(n - r + 1, n + 1))


def repperm(n: int, r: int) -> int:
    return n**r
