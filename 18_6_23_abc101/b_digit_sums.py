n_str = input()
n = int(n_str)


total = 0
for i in n_str:
	total += int(i)

if(n % total == 0):
	print("Yes")
else:
	print("No")