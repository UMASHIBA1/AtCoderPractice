# n_str  = input()
# n = int(n_str)
# k = n

# sn = 0
# for i in n_str:
# 	sn += int(i)

# n_div_sn = n / sn

# start = 1
# count = 0
# while (count <= k):
# 	sn_start = 0
# 	for i in str(start):
# 		sn_start += int(i)
# 	start_div_sn_start = start / sn_start
# 	print(start, sn_start, start_div_sn_start)
# 	if(start_div_sn_start >= n_div_sn):
# 		count+=1
# 		print(start)
# 	start+=1

# coding: utf-8
# Your code here!


# あってた人の答え
k = int(input())
count = 0
ans = []
minN = 10**15
 
for i in reversed(range(13)):
    minN = 10**15
    for j in reversed(range(1,10**3)):
        num = str(j)+"9"*i
        n = sum([int(k) for k in num])
        n = int(num)/n
        if(minN > n or n==1):
            minN = n
            ans.append(int(num))
 
for i in list(sorted(list(set(ans))))[:k]:
    print(i)