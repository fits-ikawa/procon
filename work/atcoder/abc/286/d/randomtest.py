from random import randint, choices, random, randrange, sample, choice

N = 4  # 固定
X = randint(1, 10001)
A = sample(range(1, 101), N)
B = [randint(1, 51) for _ in range(N)]

ans = False

for i in range(B[0] + 1):
    for j in range(B[1] + 1):
        for k in range(B[2] + 1):
            for l in range(B[3] + 1):
                if A[0] * i + A[1] * j + A[2] * k + A[3] * l == X:
                    ans = True

print(N, X)
print("\n".join(map(lambda p: f"{p[0]} {p[1]}", zip(A, B))))
print()
print("Yes" if ans else "No")
