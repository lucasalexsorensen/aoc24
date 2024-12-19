import math
import re
from collections import defaultdict

import numpy as np

W, H = 101, 103

with open("data/d14.txt") as f:
    bots = [[int(x) for x in re.findall(r"-?\d+", line)] for line in f.readlines()]


def simulate(t: int):
    return np.array([((x + vx * t) % W, (y + vy * t) % H) for x, y, vx, vy in bots])


quadrants = defaultdict(int)
for x, y in simulate(100):
    if x == W // 2 or y == H // 2:
        continue
    quadrants[x < W // 2, y < H // 2] += 1


print("p1", math.prod(quadrants.values()))

T = np.arange(10_000)
y = [simulate(t).var() for t in T]
best_t = np.argmin(y)

print("p2", best_t)
