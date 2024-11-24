from random import randint

N = 16

a = [randint(-100, 100) for _ in range(N)]
b = [randint(-100, 100) for _ in range(N)]

ans = max(a) + max(b)

print(N)
print(" ".join(map(str, a)))
print(" ".join(map(str, b)))
print()
print(ans)
