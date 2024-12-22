from itertools import product

import numba as nb
import numpy as np
from tqdm.auto import tqdm

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


# yoinked from stackoverflow
@nb.jit
def find_kmp(seq, subseq):
    n = len(seq)
    m = len(subseq)
    # : compute offsets
    offsets = [0] * m
    j = 1
    k = 0
    while j < m:
        if subseq[j] == subseq[k]:
            k += 1
            offsets[j] = k
            j += 1
        else:
            if k != 0:
                k = offsets[k - 1]
            else:
                offsets[j] = 0
                j += 1
    # : find matches
    i = j = 0
    while i < n:
        if seq[i] == subseq[j]:
            i += 1
            j += 1
        if j == m:
            yield i - j
            j = offsets[j - 1]
        elif i < n and seq[i] != subseq[j]:
            if j != 0:
                j = offsets[j - 1]
            else:
                i += 1


highscore = 0
changes_to_try = list(
    product(range(-10, 10), range(-10, 10), range(-10, 10), range(-10, 10))
)
for changes in tqdm(changes_to_try):
    result = 0
    for s, diff in zip(series, diffs):
        idx = next(find_kmp(diff, changes), None)
        if idx is None:
            continue
        result += s[idx + 3]
    highscore = max(highscore, result)
print("p2", highscore)
