from collections import defaultdict
from copy import deepcopy
from itertools import product

from tqdm.auto import tqdm

with open("data/d6.txt") as f:
    lines = [line.strip() for line in f.readlines()]

R, C = len(lines), len(lines[0])

grid = defaultdict(str) | {(r, c): lines[r][c] for r in range(R) for c in range(C)}

pos = next(k for k, v in grid.items() if v == "^")
dir = (-1, 0)


def rotate_dir_by_90(direction: tuple[int, int]) -> tuple[int, int]:
    x, y = direction
    return (y, -x)


seen = set()
while True:
    seen.add(pos)
    r, c = pos
    dr, dc = dir

    next_pos = (r + dr, c + dc)
    next_tile = grid[next_pos]
    if next_tile == "#":
        dir = rotate_dir_by_90(dir)
    elif next_tile in {".", "^"}:
        pos = next_pos
    else:
        break

print("p1", len(seen))

# reuse from p1
cycles = 0
for obstacle_pos in tqdm(seen):
    modified_grid = deepcopy(grid)

    pos = next(k for k, v in modified_grid.items() if v == "^")
    dir = (-1, 0)

    if modified_grid[obstacle_pos] not in {".", "^"}:
        continue
    modified_grid[obstacle_pos] = "#"

    seen_with_dir = set()
    while True:
        if (pos, dir) in seen_with_dir:
            cycles += 1
            break
        seen_with_dir.add((pos, dir))
        r, c = pos
        dr, dc = dir

        next_pos = (r + dr, c + dc)
        next_tile = modified_grid[next_pos]
        if next_tile == "#":
            dir = rotate_dir_by_90(dir)
        elif next_tile in {".", "^"}:
            pos = next_pos
        else:
            break

print("p2", cycles)
