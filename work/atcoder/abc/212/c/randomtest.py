from random import randint

N = randint(1, 10)
M = randint(1, 20)

A = [randint(1, 1000) for _ in range(N)]
B = [randint(1, 1000) for _ in range(M)]

ans = float("inf")

for a in A:
    for b in B:
        ans = min(ans, abs(a - b))

print(N)
print(M)
print(" ".join(map(str, A)))
print(" ".join(map(str, B)))
print()
print(ans)
