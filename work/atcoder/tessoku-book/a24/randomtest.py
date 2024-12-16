from random import randint, choices, random, randrange, sample, choice

N = randint(1, 20)
A = [randint(1, 20) for _ in range(N)]

dp = [0] * N
dp[0] = 1

for i in range(1, N):
    dp[i] = 1
    for j in range(0, i):
        if A[j] < A[i]:
            dp[i] = max(dp[i], dp[j] + 1)

print(N)
print(" ".join(map(str, A)))
print()
print(max(dp))
