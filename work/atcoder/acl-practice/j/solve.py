def lmap(fn, seq):
    return list(map(fn, seq))


def tmap(fn, seq):
    return tuple(map(fn, seq))


def solve():
    N, Q = map(int, input().split())
    A = lmap(int, input().split())
    qs = [tmap(int, input().split()) for _ in range(Q)]

    seg = SegmentTree(N, max, lambda: -1)
    seg.from_seq(A)

    for t, a, b in qs:
        if t == 1:
            x, v = a - 1, b
            seg.set(x, v)

        elif t == 2:
            l, r = a - 1, b
            print(seg.prod(l, r))

        else:
            x, v = a - 1, b
            print(seg.max_right(x, lambda sm: sm < v) + 1)


class SegmentTree:
    def __init__(self, n, f, unit):
        self._n = n
        self._f = f
        self._unit = unit
        self._log = (n - 1).bit_length()
        self._size = 1 << self._log
        self._data = [unit()] * (self._size << 1)

    def from_seq(self, seq):
        for i, x in enumerate(seq, self._size):
            self._data[i] = x

        for i in range(self._size - 1, 0, -1):
            self._data[i] = self._f(self._data[i << 1], self._data[i << 1 | 1])

    def set(self, i, x):
        i += self._size
        self._data[i] = x

        while i > 1:
            i >>= 1
            self._data[i] = self._f(self._data[i << 1], self._data[i << 1 | 1])

    def prod(self, l, r):
        l += self._size
        r += self._size
        vl = self._unit()
        vr = self._unit()

        while l < r:
            if l & 1:
                vl = self._f(vl, self._data[l])
                l += 1
            if r & 1:
                r -= 1
                vr = self._f(self._data[r], vr)
            l >>= 1
            r >>= 1

        return self._f(vl, vr)

    def max_right(self, l, f):
        assert 0 <= l <= self._n
        assert f(self._unit())

        if l == self._n:
            return self._n

        l += self._size
        sm = self._unit()

        while True:
            while l % 2 == 0:
                l >>= 1

            if not f(self._f(sm, self._data[l])):
                while l < self._size:
                    l <<= 1
                    if f(self._f(sm, self._data[l])):
                        sm = self._f(sm, self._data[l])
                        l += 1
                return l - self._size

            sm = self._f(sm, self._data[l])
            l += 1
            if l & -l == l:
                break

        return self._n

    def min_left(self, r, f):
        # 省略
        pass


solve()
