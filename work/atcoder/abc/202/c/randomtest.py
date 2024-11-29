from random import randint

N = randint(1, 50)

A = [randint(1, N) for _ in range(N)]
B = [randint(1, N) for _ in range(N)]
C = [randint(1, N) for _ in range(N)]

ans = 0

for i in range(N):
    for j in range(N):
        if A[i] == B[C[j] - 1]:
            ans += 1

print(N)
print(" ".join(map(str, A)))
print(" ".join(map(str, B)))
print(" ".join(map(str, C)))
print()
print(ans)
