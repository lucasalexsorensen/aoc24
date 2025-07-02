import numpy as np

with open("../data/d02.txt") as f:
    rows = [np.array(line.split(), dtype=int) for line in f.readlines()]


def is_row_valid(row: np.ndarray) -> bool:
    diff = np.diff(row)
    is_monotone = sum(diff > 0) == len(diff) or sum(diff < 0) == len(diff)
    is_max_diff_valid = max(abs(diff)) <= 3
    return is_monotone and is_max_diff_valid


print("p1", sum(is_row_valid(row) for row in rows))


def is_row_valid_p2(row: np.ndarray) -> bool:
    if is_row_valid(row):
        return True
    return any(is_row_valid(np.delete(row, [x])) for x in np.arange(len(row)))


print("p2", sum(is_row_valid_p2(row) for row in rows))
