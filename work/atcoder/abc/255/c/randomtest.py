from random import randint

N = 10
A = randint(-100, 100)
# D = randint(-100, 100)
D = randint(5, 100)

# diff = randint(-(abs(D) // 2), abs(D) // 2)
diff = D - 1

xs = [A + D * i for i in range(N)]
# X = choice(xs) + diff
X = xs[-1] + diff

print(X, A, D, N)
print()
print(abs(diff))
