# from math import floor, ceil
# # 入力を受け取る
# n = input()
# a_lst = list(map(int, input().split(" ")))


# row_lst = []
# # 各値からindexの値を抜いたものを取る
# # 1 0 0 1 0
# for (i, a) in enumerate(a_lst):
#     row_lst.append(a - (i + 1))

# # 上の合計を取る
# sum_row = sum(row_lst)

# # 上の合計を値の数で割る
# div_row = sum_row / len(row_lst)

# # 値の数で割ったものの切り上げ切り下げの値の内より悲しさが小さくなるものを選ぶ
# # print(div_row)
# # print(round(div_row))
# b_floor = floor(div_row)
# b_ceil = ceil(div_row)

# floor_result = 0
# for (i, a) in enumerate(a_lst):
#     floor_result += abs(a - (b_floor + i + 1))

# ceil_result = 0
# for (i, a) in enumerate(a_lst):
#     ceil_result += abs(a - (b_ceil + i + 1))

# print(min(floor_result, ceil_result))


# 正解

from statistics import median

n = input()
a_lst = list(map(int, input().split(" ")))

each_b = []
for (i, a) in enumerate(a_lst):
    each_b.append(a - (i + 1))

# どうやら中央値を求めれば良いらしかった
good_b = median(each_b)

total = 0
for (i, a) in enumerate(a_lst):
    total += abs(a - (good_b + i + 1))

print(int(total))
