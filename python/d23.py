import networkx as nx

G = nx.Graph()
with open("../data/d23.txt") as f:
    for line in f.readlines():
        G.add_edge(*line.strip().split("-"))

result = 0
for grp in nx.enumerate_all_cliques(G):
    if len(grp) != 3:
        continue
    if not any(c.startswith("t") for c in grp):
        continue
    result += 1

print("p1", result)

print(
    "p2",
    ",".join(sorted(sorted(nx.enumerate_all_cliques(G), key=len, reverse=True)[0])),
)
