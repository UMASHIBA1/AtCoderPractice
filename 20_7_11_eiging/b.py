n = int(input())
a_lst = list(map(int, input().split(" ")))

count = 0

for (i, a_i) in enumerate(a_lst):
    if (i + 1) % 2 == 1 and a_i % 2 == 1:
        count += 1

print(count)
