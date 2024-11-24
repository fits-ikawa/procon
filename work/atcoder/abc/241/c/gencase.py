N = 1000

print(N)
for _ in range(N):
    print("".join(["."] * N))  # No
    # print("".join([".#"] * (N // 2)))  # Yes
