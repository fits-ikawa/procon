from random import randint
import itertools

N = 10
K = randint(1, N)
D = randint(1, 100)
A = [randint(0, 100) for _ in range(N)]

ans = None

for c in itertools.combinations(range(N), K):
    total = 0
    for ci in c:
        total += A[ci]
    if total % D == 0:
        if ans is None:
            ans = total
        else:
            ans = max(ans, total)

print(N, K, D)
print(" ".join(map(str, A)))
print()
if ans is None:
    print(-1)
else:
    print(ans)
