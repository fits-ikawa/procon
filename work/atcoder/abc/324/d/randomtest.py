from random import randint
import itertools

N = 4
A = [randint(0, 9) for _ in range(N)]

sq = set()
i = 0

while True:
    if i*i > 10**N:
        break
    sq.add(i*i)
    i += 1

done = set()
ans = 0

for p in itertools.permutations(A, N):
    num = int("".join(map(str, p)))
    if num not in done and num in sq:
        ans += 1
    done.add(num)

print(N)
print("".join(map(str, A)))
print()
print(ans)
