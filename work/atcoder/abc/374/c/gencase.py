from random import randint

N = 20
K = [randint(1, 100) for _ in range(N)]

print(N)
print(" ".join(map(str, K)))
