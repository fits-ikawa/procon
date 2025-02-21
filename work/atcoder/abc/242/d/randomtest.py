from random import randint, choices, random, randrange, sample, choice

Q = 1
S = "CBA"

table = {"A": "BC", "B": "CA", "C": "AB"}


def rec(s, t):
    if t == 0:
        return s

    return rec("".join([table[c] for c in s]), t - 1)


qs = []
ans = []

for _ in range(Q):
    t = randint(0, 10)
    expand = rec(S, t)
    k = randint(1, len(expand))

    qs.append((t, k))
    ans.append(expand[k - 1])

print(S)
print(Q)
for t, k in qs:
    print(f"{t} {k}")

print()
print("\n".join(ans))

# for s in S:
#     print(s)

# print("".join(["AB", "CD"]))
