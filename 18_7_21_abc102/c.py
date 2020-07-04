n = int(input())
a_lst = list(map(int, input().split(" ")))


m = 1
for i in a_lst:
	m *= i
m -=1



print(sum([m % i for i in a_lst]))