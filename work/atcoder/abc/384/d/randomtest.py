from random import randint

N = randint(1, 5)
A_MAX = 20
A = [randint(1, A_MAX) for _ in range(N)]
REP = 3

sum_a = sum(A)

S = randint(1, sum_a * REP)

AREP = A * (REP + 1)
ans = False

for i in range(len(AREP)):
    for j in range(i, len(AREP)):
        sum = 0
        for k in range(i, j + 1):
            sum += AREP[k]
        if sum == S:
            ans = True

print(N, S)
print(" ".join(map(str, A)))
print()
print("Yes" if ans else "No")
