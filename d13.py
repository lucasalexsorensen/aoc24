import re

import numpy as np

with open("data/d13.txt") as f:
    chunks = f.read().split("\n\n")

costs = np.array([3, 1])


def chunk_cost(chunk: str, p2=False):
    nums = [int(n) for n in re.findall(r"(\d+)", chunk)]
    A = np.array(nums[:4], np.uint).reshape((2, 2))
    B = np.array(nums[4:6], np.uint)
    if p2:
        B += 10000000000000
    x = np.linalg.solve(A.T, B).round().astype(int)

    if not (x @ A == B).all():
        return 0
    if not p2 and any(x > 100):
        return 0
    return int(costs @ x)


print("p1", sum(chunk_cost(c) for c in chunks))
print("p2", sum(chunk_cost(c, p2=True) for c in chunks))
