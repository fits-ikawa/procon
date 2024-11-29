from random import randint, choice, shuffle

# エラーケースを出せなかった緩いテスト

ans = randint(5, 20)

A = list(range(ans))
shuffle(A)

pick = randint(1, ans // 2)

A = A[pick:]
trade = []

for _ in range(pick * 2):
    if randint(0, 1) == 0:
        trade.append(choice(A))
    else:
        trade.append(randint(ans + 2, ans * 10))

A = A + trade
shuffle(A)

print(len(A))
print(" ".join(map(lambda x: str(x + 1), A)))
print()
print(ans)
