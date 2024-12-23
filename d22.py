from collections import defaultdict

import numpy as np

with open("data/d22.txt") as f:
    nums = [int(line) for line in f.readlines()]


def mix(n: int, p: int) -> int:
    return n ^ p


def prune(n: int) -> int:
    mask = (1 << 24) - 1  # 2**24 = 16777216
    return n & mask


def predict_next(n: int):
    n = prune(mix(n, n * 64))
    n = prune(mix(n, n // 32))
    n = prune(mix(n, n * 2048))
    return n


result = 0
for num in nums:
    for _ in range(2000):
        num = predict_next(num)
    result += num

print("p1", result)

series = np.zeros((len(nums), 2000))
diffs = np.zeros((len(nums), 2000))
for i, num in enumerate(nums):
    for j in range(2000):
        next_num = predict_next(num)
        diffs[i, j] = next_num % 10 - num % 10
        series[i, j] = next_num % 10
        num = next_num


c = defaultdict(int)
for diff, s in zip(diffs, series):
    seen = set()
    for i in range(4, 2000):
        window = tuple(diff[i - 4 : i])
        if window in seen:
            continue
        c[window] += int(s[i - 1])
        seen.add(window)
print("p2", max(c.values()))
