
def calc_splited_total(s_str: str, index_list):
	total = 0
	if(len(index_list) == 1):
		value = index_list[0]
		total += int(s_str[0:value+1])
		total += int(s_str[value:])
	else:			
		for (i, value) in enumerate(index_list):
			# 最初のindexの場合
			if(i == 0):
				total += int(s_str[0:value])
			# 最後のindexの場合
			if(i == len(index_list) - 1):
				total += int(s_str[value])
				total += int(s_str[value + 1:])
			else:
				next_value = index_list[i + 1]
				total += int(s_str[value:next_value])
	return total


s_str = input()

len_s = len(s_str)

bit = 0
n = len_s
# strを区切ってそれらをintにしてそれを+する方法でやる

total = 0
while(bit < 1 << len_s):
	index_list = []
	for i in range(len_s):
		if(bit & (1 << i)):
			index_list.append(i)
	print(index_list)
	
	if(len(index_list) == 0):
		print(int(s_str))
		total += int(s_str)
	else:
		print(int(calc_splited_total(s_str, index_list)))
		total += calc_splited_total(s_str, index_list)

	bit+=1

print(total)