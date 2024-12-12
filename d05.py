from collections import defaultdict
from functools import cmp_to_key

with open("data/d5.txt") as f:
    chunks = f.read().split("\n\n")

rules = defaultdict(set)
for line in chunks[0].splitlines():
    a, b = line.split("|")
    rules[a].add(b)

updates = chunks[1].splitlines()


def is_update_valid(line: list[str]):
    for update_index, update in enumerate(line):
        other_indices_if_present = {
            line.index(other) for other in rules[update] if other in line
        }

        if any(other_index < update_index for other_index in other_indices_if_present):
            return False

    return True


valid_series = (s for s in (line.split(",") for line in updates) if is_update_valid(s))
print(
    "p1",
    sum(int(s[len(s) // 2]) for s in valid_series),
)


def compare(a: str, b: str) -> int:
    return -1 if b in rules[a] else 1


fixed_series = (
    sorted(series, key=cmp_to_key((compare)))
    for series in (line.split(",") for line in updates)
    if not is_update_valid(series)
)

print(
    "p2",
    sum(int(s[len(s) // 2]) for s in fixed_series),
)
