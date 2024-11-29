from random import randint

N = randint(2, 100)

A = [randint(1, 1000) for _ in range(N)]

ans = 0

for i in range(N - 1):
    for j in range(i + 1, N):
        if (A[i] - A[j]) % 200 == 0:
            ans += 1

print(N)
print(" ".join(map(str, A)))
print()
print(ans)
