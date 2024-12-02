from copy import deepcopy

with open("data/d2.txt") as f:
    all_series = [[int(s) for s in line.split()] for line in f.readlines()]


def is_series_valid(x: list[int]) -> bool:
    diff = [x[i] - x[i + 1] for i in range(len(x) - 1)]
    n_pos = sum(d > 0 for d in diff)
    n_neg = sum(d < 0 for d in diff)
    is_monotone = n_pos == len(diff) or n_neg == len(diff)
    is_max_diff_valid = max(abs(d) for d in diff) <= 3
    return is_monotone and is_max_diff_valid


print("p1", sum(is_series_valid(series) for series in all_series))


def is_series_valid_p2(x: list[int]) -> bool:
    if is_series_valid(x):
        return True

    removed = lambda i: x[:i] + x[(i + 1) :]

    return any(is_series_valid(removed(i)) for i in range(len(x)))


print("p2", sum(is_series_valid_p2(series) for series in all_series))
