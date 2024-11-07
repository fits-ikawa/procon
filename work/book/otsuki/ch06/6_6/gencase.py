from random import randint
import itertools

N = 8

# 重複しないランダムな整数列 A, B
a = [randint(0, 100) for _ in range(N)]
while len(set(a)) != N:
    a = [randint(0, 100) for _ in range(N)]

b = [randint(0, 100) for _ in range(N)]
while len(set(b)) != N:
    b = [randint(0, 100) for _ in range(N)]

print(N)
print(" ".join(map(str, a)))
print(" ".join(map(str, b)))

# 全てのペア和を出力するので、答えが存在するよう適当に K を決める
sums = sorted([i[0] + i[1] for i in itertools.product(a, b)])
print(sums)
