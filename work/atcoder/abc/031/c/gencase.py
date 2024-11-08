from random import randint

N = 50
A = [randint(-50, 50) for _ in range(N)]

print(N)
print(" ".join(map(str, A)))
