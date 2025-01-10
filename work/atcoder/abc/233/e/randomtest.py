from random import randint

N = randint(1, 1000)
X = randint(1, 10**N)

ans = 0

for k in range(len(str(X))):
    ans += X // 10**k

print(X)
print()
print(ans)
