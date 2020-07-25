n = int(input())
a_lst = list(map(int, input().split(" ")))

have_money = 1000
have_stack = 0

next_index = 1  # 高くなる波のスタートのindex
# 最初
# 高くなる波が来る前の中で一番安い日に買う
most_cheap = a_lst[0]
for i in range(1, n):
    if a_lst[i] > most_cheap:
        have_stack = have_money // most_cheap
        have_money = have_money % most_cheap
        break
    if i >= n:
        next_index += 1
        break
    most_cheap = a_lst[i]
    next_index += 1

# した二つを繰り返す
while next_index < n:
    # 高くなる波の中で一番高いところで売る
    most_expensive = a_lst[next_index]
    while next_index < n:
        if a_lst[next_index] < most_expensive or next_index == n - 1:
            have_money += have_stack * most_expensive
            have_stack = 0
            break
        most_expensive = a_lst[next_index]
        next_index += 1
    if next_index >= n - 1:
        break

    most_cheap = a_lst[next_index]
    while next_index < n:
        if a_lst[next_index] > most_cheap:
            have_stack = have_money // most_cheap
            have_money = have_money % most_cheap
            break
        most_cheap = a_lst[next_index]
        next_index += 1

print(have_money)


# 安くなる波の中で一番安い時に買う

