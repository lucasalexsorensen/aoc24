from collections import defaultdict
from itertools import combinations

with open('data/d8.txt') as f:
    grid = defaultdict(str) | {
        #(r,c): char
        (r+c*1j): char
        for r,line in enumerate(f.readlines())
        for c,char in enumerate(line.strip())
    }

antenna_positions = defaultdict(list)
for pos,char in grid.items():
    if char == '.':
        continue
    antenna_positions[char].append(pos)

antinodes = set()
for positions in antenna_positions.values():
    for a,b in combinations(positions, 2):
        diff = b-a
        for candidate in [a-diff, a+2*diff]:
            if candidate in grid:
                antinodes.add(candidate)
print('p1', len(antinodes))


antinodes = set()
for positions in antenna_positions.values():
    for a,b in combinations(positions, 2):
        diff = b-a
        for dir in [True, False]:
            candidate = a if dir else b
            while True:
                if dir:
                    candidate += diff
                else:
                    candidate -= diff
                if candidate not in grid:
                    break
                antinodes.add(candidate)
print('p2', len(antinodes))
