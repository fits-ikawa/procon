from random import randint, choices, random, randrange, sample, choice

N = randint(1, 10)
M = randint(1, 10)

MAX = 100

A = [randint(1, MAX) for _ in range(N)]
B = [randint(1, MAX) for _ in range(M)]

ans = 1

while True:
    seller = len(list(filter(lambda x: ans >= x, A)))
    buyer = len(list(filter(lambda x: ans <= x, B)))

    if seller >= buyer:
        break

    ans += 1

print(N, M)
print(" ".join(map(str, A)))
print(" ".join(map(str, B)))
print()
print(ans)
