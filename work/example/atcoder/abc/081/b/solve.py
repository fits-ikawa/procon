def lmap(fn, seq):
    return list(map(fn, seq))


def solve():
    N = int(input())
    A = lmap(int, input().split())

    count = 0

    while all(map(lambda x: x % 2 == 0, A)):
        A = lmap(lambda x: x >> 1, A)
        count += 1

    print(count)


solve()
