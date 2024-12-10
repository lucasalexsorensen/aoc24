from collections import defaultdict, deque

with open("data/d10.txt") as f:
    grid = defaultdict(lambda: -1) | {
        (r + c * 1j): int(char) if char != "." else -1
        for r, line in enumerate(f.readlines())
        for c, char in enumerate(line.strip())
    }

directions = [1j, -1j, 1, -1]

sources = {pos for pos, val in grid.items() if val == 0}


def get_bfs(do_seen_check: bool):
    def bfs(source: complex) -> int:
        Q = deque([source])
        result = 0
        seen = set()
        while Q:
            pos = Q.popleft()
            if pos in seen:
                continue
            if do_seen_check:
                seen.add(pos)
            if grid[pos] == 9:
                result += 1
            for dir in directions:
                if grid[(pos + dir)] == grid[pos] + 1:
                    Q.append((pos + dir))
        return result

    return bfs


p1_bfs = get_bfs(do_seen_check=True)
print("p1", sum(p1_bfs(source) for source in sources))

p2_bfs = get_bfs(do_seen_check=False)
print("p2", sum(p2_bfs(source) for source in sources))
