n,k = list(map(int, input().split(" ")))
a_lst = list(map(int,input().split(" ")))


plus_lst = list(filter(lambda num: num >= 0 , a_lst))
minus_lst = list(filter(lambda num: num < 0 , a_lst))
plus_lst.sort(reverse=True)
minus_lst.sort(reverse=True)


# +,-どちらもk以上ある
	# max(abs+,abs-)で大きい方をk-1回目までとる, k-1回目で+だったら+, -だったら-
# +: kない, -: kある
	# +の数が残り1になるまでabsでやる,最後に+を使って調整する
# +: kある, -: kない
	# -の数が1になるまでabsでやる, 最後に-を使って調整
# +: kない, -: kない
	# どちらかが残り1になるまでやる,最後に調整
# -のみ
	# absでできるだけ小さいものを使う
print(minus_lst)

# 0を抜いたらk個なかった場合、絶対どっかで0を掛けるので0
# 全部-の場合
if(len(plus_lst) == 0):
	total = 1
	for i in range(k):
		total *=minus_lst[i]
	print(total % (10**9 + 7))
elif(len(minus_lst) == 0):
	total = 1
	for i in range(k):
		total *= plus_lst[i]
	print(total % (10**9 + 7))
else:
	total = 1
	for i in range(k):
		if(i == k - 1):
			if(total > 0):
				total *= plus_lst.pop(0)
			else:
				total *= minus_lst.pop(len(minus_lst)-1)
		else:
			# plus残り1の場合
			if(len(plus_lst) == 1):
				total *= minus_lst.pop(len(minus_lst)-1)
			# minus残り1の場合
			elif(len(minus_lst) == 1):
				total *= plus_lst.pop(0)					
			# +-どっちも十分あった場合
			else:
				plus_max = plus_lst[0]
				minus_max = minus_lst[0]
				if(abs(plus_max) > abs(minus_max)):
					plus_lst.pop(0)
					total *= plus_max
				else:
					minus_lst.pop(len(minus_lst)-1)
					total *= minus_max
	print(total % (10**9 + 7))
