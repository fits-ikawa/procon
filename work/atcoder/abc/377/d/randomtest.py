from random import randint

N = 3
M = 10
L = [randint(1, M) for _ in range(N)]
R = [randint(L[i], M) for i in range(N)]

# N = 6
# M = 20
# L = [8, 14, 11, 5, 4, 1]
# R = [12, 20, 13, 19, 11, 6]

ans = 0

for i in range(1, M + 1):
    for j in range(i, M + 1):
        include = False
        for k in range(N):
            include = include or (i <= L[k] and R[k] <= j)
        if not include:
            ans += 1

print(N, M)
for i in range(N):
    print(L[i], R[i])
print()
print(ans)
