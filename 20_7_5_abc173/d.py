import heapq
n = int(input())
a_lst = list(map(int, input().split(" ")))

a_lst.sort(reverse=True)


result = 0
heap = []
heapq.heappush(heap,-a_lst[0])

for i in a_lst[1:]:
	now_max = heapq.heappop(heap)
	result += now_max
	heapq.heappush(heap, -i)
	heapq.heappush(heap, -i)

print(-result)