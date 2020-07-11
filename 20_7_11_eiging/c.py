n = int(input())
x = list(input())

# int化
def to_int(binary, n):
    result = 0
    for i in range(n):
        result += int(binary[(n - i - 1)]) * (2 ** i)
    return result


def popcount_by_int(n):
    count = 0
    while n >= 1:
        n = n / 2
        count += 1
    return count


def fn(n):
    count = 0
    while n != 0:
        p_count = popcount_by_int(n)
        count += 1

        n = n % p_count
    return count


for i in range(n):
    binary = x.copy()
    binary[i] = int(x[i]) ^ 1
    int_v = to_int(binary, n)
    print(fn(int_v))

