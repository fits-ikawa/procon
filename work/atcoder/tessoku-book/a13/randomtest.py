from random import randint

N = randint(1, 50)
K = randint(1, 10000)
A = [randint(1, 10000) for _ in range(N + 1)]

A.sort()

ans = 0

for i in range(N - 1):
    for j in range(i + 1, N):
        if A[j] - A[i] <= K:
            ans += 1

print(N, K)
print(" ".join(map(str, A)))
print()
print(ans)
