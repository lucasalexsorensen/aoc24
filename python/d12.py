import sys
from collections import defaultdict, deque

with open("../data/d12.txt") as f:
    grid = {
        r + c * 1j: char
        for r, line in enumerate(f.readlines())
        for c, char in enumerate(line.strip())
    }

UP, DOWN, LEFT, RIGHT = -1, 1, -1j, 1j
dirs = {UP, DOWN, LEFT, RIGHT}


# bfs floodfill type shit
regions = []
visited = set()
while len(visited) < len(grid):
    start = (set(grid.keys()) - visited).pop()

    Q = deque([start])
    region = set()
    while Q:
        pos = Q.popleft()
        if pos in region or pos in visited:
            continue
        region.add(pos)
        visited.add(pos)
        for dir in dirs:
            new_pos = pos + dir
            if new_pos in grid and grid[pos + dir] == grid[start]:
                Q.append(pos + dir)
    regions.append((grid[start], region))


def perimeter(region: set[complex]) -> int:
    p = 0
    for pos in region:
        for dir in dirs:
            if pos + dir in region:
                continue
            p += 1
    return p


print("p1", sum(len(region) * perimeter(region) for _, region in regions))


def n_sides(region: set[complex]) -> int:
    corners = 0
    # just hardcode the corner cases lol
    for pos in region:
        # .Z
        # ..
        corners += int(pos + LEFT not in region and pos + DOWN not in region)

        # ..
        # .Z
        corners += int(pos + LEFT not in region and pos + UP not in region)

        # Z.
        # ..
        corners += int(pos + RIGHT not in region and pos + DOWN not in region)

        # ..
        # Z.
        corners += int(pos + RIGHT not in region and pos + UP not in region)

        # XZ
        # .X
        corners += int(
            pos + LEFT in region
            and pos + DOWN in region
            and pos + DOWN + LEFT not in region
        )

        # ZX
        # X.
        corners += int(
            pos + RIGHT in region
            and pos + DOWN in region
            and pos + DOWN + RIGHT not in region
        )

        # X.
        # ZX
        corners += int(
            pos + RIGHT in region
            and pos + UP in region
            and pos + UP + RIGHT not in region
        )

        # .X
        # XZ
        corners += int(
            pos + LEFT in region
            and pos + UP in region
            and pos + UP + LEFT not in region
        )

    return corners


r = 0
for symbol, region in regions:
    r += len(region) * n_sides(region)
print("p2", sum(len(region) * n_sides(region) for _, region in regions))
