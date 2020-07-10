s = input()
t = input()

s_len = len(s)

is_yes = False

rotate_s = s
for i in range(s_len):
	if(rotate_s == t):
		is_yes = True
		break
	rotate_s = rotate_s[-1] + rotate_s[:s_len - 1]
if(is_yes):
	print("Yes")
else:
	print("No")