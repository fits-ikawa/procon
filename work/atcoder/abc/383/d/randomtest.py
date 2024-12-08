from random import randint


def divisors(n):
    low = []
    high = []

    for i in range(1, int(n**0.5) + 1):
        if n % i == 0:
            low.append(i)
            if i != n // i:
                high.append(n // i)

    return low + high[::-1]


N = randint(1, 10000)

ans = 0

for i in range(N + 1):
    if len(divisors(i)) == 9:
        ans += 1

print(N)
print()
print(ans)
