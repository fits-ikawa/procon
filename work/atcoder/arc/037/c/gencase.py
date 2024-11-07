from random import randint
import itertools

N = 1000
K = randint(1, N**2)

A = [randint(1, 10000) for _ in range(N)]
B = [randint(1, 10000) for _ in range(N)]

mult = [ai * bi for ai, bi in itertools.product(A, B)]
mult.sort()

print(N, K)
print(" ".join(map(str, A)))
print(" ".join(map(str, B)))
print()
print(mult[K - 1])
