# this approach works backwards from the goal, and continues only with the options that are possible

from typing import Literal

with open("../data/d07.txt") as f:
    lines = f.readlines()

# example:
# valid(v=123456, terms=[2, 617, 56])
# |-ADD-> valid(123400, [2, 617])
# |       |-ADD-> valid(122783, [2]) => ❌ 122783 != 2
# |       |-MUL-> valid(200, [2]) => ❌ 200 != 2
# |       |-CAT-> 🚫 impossible, since 123400 does not end in 56
# |-MUL-> 🚫 impossible, since 123400 % 617 != 0
# |-CAT-> valid(1234, [2, 617])
# |       |-ADD-> valid(1222, [2]) => ❌ 1222 != 2
# |       |-MUL-> valid(2, [2]) => ✅
# |       |-CAT-> valid(1234, [2]) => ❌ 1234 != 2


type Operator = Literal["add", "mul", "concat"]


def valid(v: int, terms: list[int], operators: list[Operator]) -> bool:
    #  print(f"valid({v}, {terms}, {operators})")
    if len(terms) == 1:
        return v == terms[0]

    next_terms, t = terms[:-1], terms[-1]

    def handle_op(op: Operator) -> bool:
        if op == "add":
            if v < t:
                return False
            return valid(v - t, next_terms, operators)
        elif op == "mul":
            if v % t != 0:
                return False
            return valid(v // t, next_terms, operators)
        elif op == "concat":
            q, r = divmod(v, 10 ** len(str(t)))
            if r != t:
                return False
            return valid(q, next_terms, operators)
        else:
            raise ValueError(f"invalid operator: {op}")

    return any(handle_op(op) for op in operators)


def is_line_valid(line: str, operators: list[Operator]) -> int:
    goal, rest = line.strip().split(": ")
    goal = int(goal)
    terms = [int(t) for t in rest.split()]

    if valid(goal, terms, operators):
        return goal
    return 0


print("p1", sum(is_line_valid(line, ["add", "mul"]) for line in lines))
print("p2", sum(is_line_valid(line, ["add", "mul", "concat"]) for line in lines))
