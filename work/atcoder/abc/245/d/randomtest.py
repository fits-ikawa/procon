from random import randint, choices, random, randrange, sample, choice

N = randint(1, 10)
M = randint(1, 10)

A = [randint(-100, 100) for _ in range(N)]
A.append(randint(1, 100) * (1 if randint(0, 1) == 0 else -1))  # 最高次数は 0 でない

B = [randint(-100, 100) for _ in range(M)]
B.append(randint(1, 100) * (1 if randint(0, 1) == 0 else -1))  # 最高次数は 0 でない

C = [0] * (N + M + 1)

for i in range(N + 1):
    for j in range(M + 1):
        C[i + j] += A[i] * B[j]

print(N, M)
print(" ".join(map(str, A)))
print(" ".join(map(str, C)))
print()
print(" ".join(map(str, B)))
