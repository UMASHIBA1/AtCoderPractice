n = input()
a_lst = list(map(int, input().split(" ")))

a_lst.sort()

print(abs(a_lst[0] - a_lst[-1]))
