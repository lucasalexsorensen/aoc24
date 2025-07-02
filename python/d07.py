from itertools import product
from operator import add, mul
from typing import Callable

with open("../data/d07.txt") as f:
    lines = f.readlines()


def is_line_valid(line: str, oplist: list[Callable[[int, int], int]]) -> int:
    goal, rest = line.strip().split(": ")
    goal = int(goal)
    terms = [int(t) for t in rest.split()]
    for ops in product(oplist, repeat=len(terms) - 1):
        result = terms[0]
        for op, term in zip(ops, terms[1:]):
            result = op(result, term)
        if result == goal:
            return goal
    return 0


print("p1", sum(is_line_valid(line, [add, mul]) for line in lines))


def concat(a: int, b: int) -> int:
    return int(str(a) + str(b))


print("p2", sum(is_line_valid(line, [add, mul, concat]) for line in lines))
