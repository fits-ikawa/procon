def lmap(fn, seq):
    return list(map(fn, seq))


def tmap(fn, seq):
    return tuple(map(fn, seq))


def solve():
    N, Q = map(int, input().split())
    A = lmap(int, input().split())
    qs = [tmap(int, input().split()) for _ in range(Q)]

    bit = FenwickTree(N)

    for i in range(N):
        bit.add(i, A[i])

    for t, a, b in qs:
        if t == 0:
            p, x = a, b
            bit.add(p, x)
        else:
            l, r = a, b
            print(bit.sum(l, r))


class FenwickTree:
    def __init__(self, n):
        self._n = n
        self._data = [0] * n

    def add(self, p, x):
        assert 0 <= p < self._n
        p += 1
        while p <= self._n:
            self._data[p - 1] += x
            p += p & -p

    def _sum(self, r):
        # 右半開区間で考えるので r はそのまま 1-indexed での r になる。
        # よって r += 1 しない
        s = 0
        while r > 0:
            s += self._data[r - 1]
            r -= r & -r
        return s

    def sum(self, l, r):
        assert 0 <= l <= r <= self._n
        return self._sum(r) - self._sum(l)


solve()
