# w: 列数
# h: 行数
h, w, k = list(map(int,input().split(" ")))

# w * h
matrix = []

total_black = 0

for i in range(h):
	one_line = input()
	lst = []
	for j in one_line:
		lst.append()
		if(j == "#"):
			total_black += 1
	matrix.append(lst)

must_decrease_num = total_black - k

result_count = 0

for i in range(2 ** h):
