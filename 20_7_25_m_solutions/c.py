n, k = list(map(int, input().split(" ")))

a_lst = list(map(int, input().split(" ")))

for i in range(k, n):
    if a_lst[i] > a_lst[i - k]:
        print("Yes")
    else:
        print("No")
