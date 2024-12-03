from random import randint, choice
from string import ascii_lowercase

N = randint(2, 50)
S = [choice(ascii_lowercase) for _ in range(N)]

ts = set()

for i in range(N - 1):
    for j in range(i + 1, N):
        T = S.copy()
        T[i] = S[j]
        T[j] = S[i]
        ts.add("".join(T))


print("".join(S))
print()
print(len(ts))
