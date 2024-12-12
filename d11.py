from collections import defaultdict
from copy import deepcopy

from tqdm.auto import tqdm

with open("data/d11.txt") as f:
    line = f.read().strip()


def part_one():
    stones = line.split(" ")
    for _ in range(25):
        i = 0
        while i < len(stones):
            if stones[i] == "0":
                stones[i] = "1"
            elif len(stones[i]) % 2 == 0:
                mid = len(stones[i]) // 2
                left, right = stones[i][:mid], stones[i][mid:]
                left, right = left.lstrip("0") or "0", right.lstrip("0") or "0"
                stones[i] = right
                stones.insert(i, left)
                i += 1  # extra bump to account for the additional element
            else:
                stones[i] = str(int(stones[i]) * 2024)
            i += 1
    return len(stones)


# print("p1", part_one())


def part_two():
    # the overall positions of the stones doesn't matter
    # so we can resort to using a dict for storing the stones instead
    stones = defaultdict(int)
    for s in line.split():
        stones[s] += 1

    for _ in range(75):
        new_stones = deepcopy(stones)

        for stone, count in stones.items():
            if count == 0:
                continue
            if stone == "0":
                new_stones[stone] -= count
                new_stones["1"] += count
            elif len(stone) % 2 == 0:
                new_stones[stone] -= count
                mid = len(stone) // 2
                left, right = stone[:mid], stone[mid:]
                left, right = left.lstrip("0") or "0", right.lstrip("0") or "0"
                new_stones[left] += count
                new_stones[right] += count
            else:
                new_stones[stone] -= count
                new_stones[str(int(stone) * 2024)] += count

        stones = new_stones

    return sum(stones.values())


print("p2", part_two())
