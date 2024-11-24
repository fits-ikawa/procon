from random import shuffle

N = 2
S = []

for _ in range(N):
    reel = list(range(10))
    shuffle(reel)
    S.append(reel)


def solve(N, S):
    inf = float("inf")
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


print(N)
for reel in S:
    print("".join(map(str, reel)))
print()
solve(N, S)
