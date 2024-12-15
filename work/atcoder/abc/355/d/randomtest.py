from random import randint, choices, random, randrange, sample, choice

N = randint(2, 10)
R_MAX = 100
L = [randint(0, R_MAX - 1) for _ in range(N)]
R = [l + randint(1, R_MAX - l) for l in L]

ans = 0
for i in range(N - 1):
    for j in range(i + 1, N):
        if L[i] <= R[j] and L[j] <= R[i]:
            ans += 1

print(N)
print("\n".join([" ".join(map(str, lr)) for lr in zip(L, R)]))
print()
print(ans)
