from random import randint

N = randint(1, 50)
K = randint(1, 1000)
A = [randint(1, 200) for _ in range(N)]

ans = 0

for i in range(N):
    for j in range(i, N):
        sum = 0
        for k in range(i, j + 1):
            sum += A[k]
        if sum <= K:
            ans += 1

print(N, K)
print(" ".join(map(str, A)))
print()
print(ans)
