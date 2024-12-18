import re
from collections import deque

with open("data/d15.txt") as f:
    chunks = f.read().split("\n\n")


grid = {
    complex(r, c): char
    for r, line in enumerate(chunks[0].splitlines())
    for c, char in enumerate(line.strip())
}

pos = next(k for k, v in grid.items() if v == "@")
grid[pos] = "."

dirs = {"^": -1 + 0j, "v": 1 + 0j, "<": -1j, ">": 1j}

moves = "".join(chunks[1].splitlines())
for move in moves:
    dir = dirs[move]

    def scan_until_wall():
        scanner_pos = pos
        while True:
            scanner_pos += dir
            if grid[scanner_pos] == "#":
                break
            yield grid[scanner_pos]

    scanner = enumerate(scan_until_wall())

    first_empty_idx = next((i for i, c in scanner if c == "."), None)
    if first_empty_idx is None:
        continue

    # we need to make it such that first_empty_idx == 0
    # e.g. ['O', '.', 'O', '.'] => ['.', 'O', 'O', '.']
    if first_empty_idx > 0:
        grid[pos + dir] = "."
        grid[pos + dir * (first_empty_idx + 1)] = "O"

    pos += dir

print("p1", sum(int(100 * k.real + k.imag) for k, v in grid.items() if v == "O"))

bgrid = {}
for r, line in enumerate(chunks[0].splitlines()):
    for c, char in enumerate(line.strip()):
        if char == "@":
            c1, c2 = ".."
            pos = complex(r, c * 2)
        elif char == "O":
            c1, c2 = "[]"
        else:
            c1, c2 = char, char

        bgrid[complex(r, c * 2)] = c1
        bgrid[complex(r, c * 2 + 1)] = c2


R, C = len(chunks[0].splitlines()), 2 * len(chunks[0].splitlines()[0].strip())

for move in moves:
    dir = dirs[move]
    ##############            ##############
    ##......##..##            ##......##..##
    ##..........##  move ^    ##...xxxx...##
    ##...[][]...##    =>      ##...<><>...##
    ##....[][]..##            ##....<>[]..##
    ##.....@....##            ##.....@....##
    ##############            ##############

    # instead of scanning in a straight line, we should scan in some BFS fashion
    Q = deque([pos + dir])
    seen = set()
    stones: set[tuple[complex, complex]] = set()

    doable = True
    while Q:
        p = Q.popleft()
        scanned_char = bgrid[p]
        if p in seen:
            continue
        seen.add(p)

        if scanned_char == "#":
            doable = False
            break
        if scanned_char == ".":
            continue
        if scanned_char == "]":
            Q.append(p + dirs["<"])
            stones.add((p + dirs["<"], p))
        elif scanned_char == "[":
            Q.append(p + dirs[">"])
            stones.add((p, p + dirs[">"]))
        Q.append(p + dir)

    if not doable:
        continue

    new_stones = {(a + dir, b + dir) for a, b in stones}

    # a bit crude, but just wipe the grid at every previous stone location, and put them at the new locations
    for a, b in stones:
        bgrid[a] = "."
        bgrid[b] = "."
    for a, b in new_stones:
        bgrid[a] = "["
        bgrid[b] = "]"

    pos += dir

print("p1", sum(int(100 * k.real + k.imag) for k, v in bgrid.items() if v == "["))
