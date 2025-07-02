import heapq
import random

R, C = 71, 71
incoming = []
with open("../data/d18.txt") as f:
    for line in f.readlines():
        x, y = line.strip().split(",")
        incoming.append((int(y), int(x)))


def heuristic(u: tuple):
    return abs(R - u[0]) + abs(C - u[1])


dirs = {(1, 0), (-1, 0), (0, 1), (0, -1)}


end: tuple[int, int] = (R - 1, C - 1)


def a_star(n: int):
    occupied = set(incoming[:n])
    pos: tuple[int, int] = (0, 0)
    Q: list[tuple[float, tuple[int, int]]] = [(0.0, pos)]
    seen = set()
    dist: dict[tuple[int, int], int] = {pos: 0}
    while Q:
        _, u = heapq.heappop(Q)
        if u in seen:
            continue

        for dir in dirs:
            v = (u[0] + dir[0], u[1] + dir[1])
            if not 0 <= v[0] < R or not 0 <= v[1] < C:
                continue

            if v in occupied:
                continue

            alt = dist.get(u, 0) + 1

            if alt < dist.get(v, float("inf")):
                dist[v] = alt
                heapq.heappush(Q, (alt + heuristic(v), v))

        seen.add(u)
    return dist


dist = a_star(1024)
print("p1", dist[end])

lower, upper = 1024, len(incoming)
while True:
    mid = (lower + upper) // 2

    pre_has_path = end in a_star(mid - 1)
    post_has_path = end in a_star(mid)

    if pre_has_path and not post_has_path:
        last_valid = incoming[mid - 1]
        break
    elif pre_has_path and post_has_path:
        lower = mid
    elif not pre_has_path and not post_has_path:
        upper = mid
print("p2", f"{last_valid[1]},{last_valid[0]}")
