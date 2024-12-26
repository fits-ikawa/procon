from random import randint, choices, random, randrange, sample, choice
import itertools

N = randint(1, 5)
K = randint(1, N)
P = randint(1, 10000)
A = [randint(1, 2000) for _ in range(N)]

ans = 0

for c in itertools.combinations(range(N), K):
    total = 0
    for ci in c:
        total += A[ci]
    if total < P:
        ans += 1

print(N, K, P)
print(" ".join(map(str, A)))
print()
print(ans)
