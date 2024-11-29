from random import choice
import itertools

N = 25

chars = list("chokudaix")
S = [choice(chars) for _ in range(N)]

ans = 0

for c in itertools.combinations(S, 8):
    if "".join(c) == "chokudai":
        ans += 1

print("".join(S))
print()
print(ans)

assert ans == 0  # 部分列が作れる S を見つけたら停止
