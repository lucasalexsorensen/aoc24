import networkx as nx

LEFT, RIGHT, UP, DOWN = -1j, 1j, -1 + 0j, 1 + 0j
dirs = {LEFT, RIGHT, UP, DOWN}

G = nx.DiGraph()

with open("data/d16.txt") as f:
    grid = {
        r + c * 1j: char
        for r, row in enumerate(f.readlines())
        for c, char in enumerate(row.strip())
    }

start_pos = next(k for k, v in grid.items() if v == "S")
end_pos = next(k for k, v in grid.items() if v == "E")

for p, char in grid.items():
    if char == "#":
        continue
    for dir in dirs:
        G.add_edge((p, dir), (p, dir * 1j), weight=1000)
        G.add_edge((p, dir), (p, dir * (-1j)), weight=1000)
        if p + dir in grid and grid[p + dir] != "#":
            G.add_edge((p, dir), (p + dir, dir), weight=1)

shortest_path_lens = {
    (nx.shortest_path_length(G, (start_pos, RIGHT), (end_pos, d), "weight"), d)
    for d in dirs
}

shortest_path_len, shortest_path_dir = min(shortest_path_lens)
print("p1", shortest_path_len)

p = set()
for path in nx.all_shortest_paths(
    G, (start_pos, RIGHT), (end_pos, shortest_path_dir), "weight"
):
    for pos, _ in path:
        p.add(pos)

print("p2", len(p))
