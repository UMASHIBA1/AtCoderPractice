# n,m = list(map(int,input().split(" ")))

# ab_lst = []

# for i in range(m):
# 	ab = list(map(int, input().split(" ")))
# 	ab_lst.append(ab)


# cut_distance = []

# for a,b in ab_lst:
# 	have_add = True
# 	for cut in cut_distance:
# 		is_break = False
# 		if(a > cut[0] and a < cut[1]):
# 			cut[0] = a
# 			have_add = False
# 			is_break = True
# 		if(b < cut[1] and b > cut[0]):
# 			cut[1] = b
# 			have_add = False
# 			is_break = True
# 		if(is_break):
# 			break
# 	if(have_add):
# 		cut_distance.append([a,b])
	
# print(len(cut_distance))

# 正解者の回答
def resolve():
    import sys
    from operator import itemgetter
    
    readline = sys.stdin.readline    # 1行だけ文字列にする
 
    N, M = map(int, readline().split())
    AB = [list(map(int, readline().split())) for _ in [0] * M]
    AB.sort(key=itemgetter(1))
    islands = -float('inf')
    ans = 0
    for left, right in AB:
        if islands <= left:
            ans += 1
            islands = right
    print(ans)
    
 
if __name__ == "__main__":
    resolve()