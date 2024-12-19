from functools import cache
from itertools import combinations, pairwise, permutations

with open("data/d19.txt") as f:
    chunks = f.read().split("\n\n")

components = set(chunks[0].split(", "))
towels = [line.strip() for line in chunks[1].splitlines()]


@cache
def can_be_made(s: str) -> bool:
    if s == "":
        return True
    return any(can_be_made(s[len(c) :]) for c in components if s.startswith(c))


print("p1", sum(can_be_made(t) for t in towels))


@cache
def how_many_ways_to_make(s: str) -> int:
    if len(s) == 0:
        return 1

    return sum(
        how_many_ways_to_make(s[len(c) :]) for c in components if s.startswith(c)
    )


print("p2", sum(how_many_ways_to_make(t) for t in towels))
