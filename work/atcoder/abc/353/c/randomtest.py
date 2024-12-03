from random import randint

N = 5
A = [randint(1, 10**8) for _ in range(N)]
# N = 5
# A = [1, 3, 99999999, 99999994, 1000000]

ans = 0

for i in range(N - 1):
    for j in range(i + 1, N):
        ans += (A[i] + A[j]) % 10**8

print(N)
print(" ".join(map(str, A)))
print()
print(ans)
