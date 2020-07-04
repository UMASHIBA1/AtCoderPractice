command = input()
first = 0
for i in command:
	if (i == "+"):
		first+=1
	elif(i == "-"):
		first -= 1

print(first)