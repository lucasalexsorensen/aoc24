import re

with open("data/d3.txt") as f:
    line = "".join(f.readlines())

print("p1", sum(int(a) * int(b) for a, b in re.findall(r"mul\((\d+),(\d+)\)", line)))

enabled = True
result = 0
for match in re.findall(r"mul\((\d+),(\d+)\)|(do)\(\)|(don't)\(\)", line):
    a, b, do, dont = match
    if do:
        enabled = True
    elif dont:
        enabled = False
    else:
        if not enabled:
            continue
        result += int(a) * int(b)


print("p2", result)
