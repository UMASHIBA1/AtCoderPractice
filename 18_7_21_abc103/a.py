a1,a2,a3 = map(int,input().split(" "))


a12 = abs(a1 - a2)
a13 = abs(a1 - a3)
a23 = abs(a2 - a3)

max_num = max(a12,a13, a23)

print(a12 + a13 + a23 - max_num)

