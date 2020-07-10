# import heapq

# n = int(input())
# a_lst = list(map(int, input().split(" ")))

# max_a = max(a_lst)

# # 最小のモノを取り出す為に使う
# heap = []

# for (i, a) in enumerate(a_lst):
#     heapq.heappush(heap, (a, i))

# while len(a_lst) > 4:
#     # 最小の値を取り出す
#     (min_a, index) = heapq.heappop(heap)
#     print("len", len(a_lst))
#     print("index", index)
#     # 前後のindexの値のうち小さい方をmin_aに足す
#     previous_a = a_lst[index - 1] if index > 0 else max_a + 1
#     next_a = a_lst[index + 1] if index < len(a_lst) - 1 else max_a + 1
#     neighbor_min_index = None
#     neighbor_min_a = None
#     if previous_a < next_a:
#         neighbor_min_index = index - 1
#         neighbor_min_a = previous_a
#     else:
#         neighbor_min_index = index + 1
#         neighbor_min_a = next_a
#     min_a += neighbor_min_a
#     a_lst.pop(index)
#     a_lst.insert(index, min_a)
#     # heapに入れる
#     heapq.heappush(heap, (min_a, index))
#     a_lst.pop(neighbor_min_index)

# (min_a, _) = heapq.heappop(heap)
# print(abs(max_a - min_a))


def calc_min_max(sum_of_list: int, num_list):
    half_sum = sum_of_list / 2
    # 前半を計算
    total = num_list[0]
    previous_total = num_list[0]
    index = 0
    while total < half_sum:
        previous_total = total
        total += num_list[index]
        index += 1
    if abs(half_sum - previous_total) < abs(half_sum - total):
        after_half = sum(num_list[i:])
        return {
            "min": min(previous_total, after_half),
            "max": max(previous_total, after_half),
        }
    else:
        after_half = sum(num_list[i + 1 :])
        return {"min": min(total, after_half), "max": max(total, after_half)}

    # return {"min": previous_total, "max": total}


n = int(input())
a_lst = list(map(int, input().split(" ")))

sum_a = sum(a_lst)

most_min_abs = sum_a + 1

for i in range(2, n - 1):
    left = a_lst[:i]
    right = a_lst[i:n]
    print(left, right)
    # 左側の小さい合計、大きい合計をとる
    sum_left = sum(left)
    left_min_max = calc_min_max(sum_left, left)

    # 右側の小さい合計、大きい合計をとる
    sum_right = sum(right)
    right_min_max = calc_min_max(sum_right, right)

    min_in_left_right = min(left_min_max["min"], right_min_max["min"])
    max_in_left_right = max(left_min_max["max"], right_min_max["max"])
    print("sum_left", sum_left, "left_min_max", left_min_max)
    print("sum_right", sum_right, "left_min_max", right_min_max)
    print(max_in_left_right)

    this_abs = abs(max_in_left_right - min_in_left_right)

    if this_abs < most_min_abs:
        most_min_abs = this_abs

print(most_min_abs)
