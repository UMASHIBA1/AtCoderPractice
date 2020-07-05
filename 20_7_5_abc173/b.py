n = int(input())
s_list = []
for i in range(n):
	s_list.append(input())

ac = 0
wa = 0
tle = 0
re = 0

for i in s_list:
	if(i == "AC"):
		ac+=1
	elif(i == "WA"):
		wa += 1
	elif(i == "TLE"):
		tle += 1
	elif(i == "RE"):
		re += 1

print("AC x " + str(ac))
print("WA x " + str(wa))
print("TLE x " + str(tle))
print("RE x " + str(re))