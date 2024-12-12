from itertools import product

with open("data/d4.txt") as f:
    grid = [list(line.strip()) for line in f.readlines()]

directions = set(product([-1, 0, 1], [-1, 0, 1])) - {(0, 0)}

series = dict(zip('XMA', 'MAS'))

def dfs(r: int, c: int, prev_char = 'X', prev_dir = (0,0)) -> int:
    if not 0 <= r < len(grid) or not 0 <= c < len(grid[0]):
        return 0
    if grid[r][c] != prev_char:
        return 0
    if prev_char == "S":
        return 1

    return sum(
        dfs(r + dr, c + dc, series[prev_char], (dr,dc))
        for (dr, dc) in directions
        if (prev_dir == (0,0)) or (prev_dir == (dr,dc))
    )

print("p1", sum(
    dfs(r,c)
    for r in range(len(grid))
    for c in range(len(grid[0]))
))

def is_valid_cross(r: int, c: int) -> bool:
    if grid[r][c] != 'A':
        return False

    left_diagonal = {(-1,-1), (1,1)}
    right_diagonal = {(1,-1), (-1, 1)}

    for dirs in (
        left_diagonal, right_diagonal
    ):
        diagonal_chars = {
            grid[r+dr][c+dc]
            for dr,dc in dirs
            if 0 <= (r+dr) < len(grid) and 0 <= (c+dc) < len(grid[0])
        }
        if not diagonal_chars == {'S', 'M'}:
            return False
    return True

print("p2", sum(
    is_valid_cross(r,c)
    for r in range(len(grid))
    for c in range(len(grid[0]))
))
