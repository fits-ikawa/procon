def lmap(fn, seq):
    return list(map(fn, seq))


def solve():
    inf = float("inf")

    N = int(input())
    S = [lmap(int, list(input())) for _ in range(N)]
    ans = inf

    for target in range(10):
        t = 0
        rolling = set(range(N))

        while True:
            stop = False
            for reel in range(N):
                if reel in rolling and S[reel][t % 10] == target:
                    stop = True
                    break
            if stop:
                rolling.remove(reel)
            t += 1

            if len(rolling) == 0:
                break

        ans = min(ans, t - 1)

    print(ans)


solve()
