l, r, d = list(map(int, input().split(" ")))

now = l
count = 0
while now <= r:
    if now % d == 0:
        count += 1
    now += 1
print(count)
