from random import randint

A = randint(-100, 100)
B = randint(-100, 100)
C = randint(1, 10)

ac = A**C
bc = B**C

if ac < bc:
    ans = "<"
elif ac == bc:
    ans = "="
elif ac > bc:
    ans = ">"

print(A, B, C)
print()
print(ans)
