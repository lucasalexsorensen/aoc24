from copy import deepcopy

with open("data/d9.txt") as f:
    line = f.read().strip()

series = []
is_file = True
for i, char in enumerate(line):
    c = int(char)
    part = ([i // 2] * c) if is_file else (["."] * c)
    series.extend(part)
    is_file = not is_file

original_series = deepcopy(series)

i = 0
j = len(series) - 1
while i < j:
    if series[i] != ".":
        i += 1
        continue
    if series[j] == ".":
        j -= 1
        continue
    series[i], series[j] = series[j], series[i]


print("p1", sum(s * i for i, s in enumerate(series[:i])))

parts: list[tuple[str, int]] = []
for i, char in enumerate(line):
    is_file = i % 2 == 0
    part = str(i // 2) if is_file else ".", int(char)
    parts.append(part)

for j in range(len(parts))[::-1]:
    for i in range(j):
        j_char, occupied_size = parts[j]
        i_char, empty_size = parts[i]

        if occupied_size > empty_size:
            continue
        if i_char != ".":
            continue
        if j_char == ".":
            continue

        parts[j] = ".", occupied_size
        parts[i] = ".", empty_size - occupied_size
        parts.insert(i, (j_char, occupied_size))

checksum = []
for part in parts:
    c = int(part[0]) if part[0].isdigit() else 0
    checksum.extend([c] * part[1])

print("p2", sum(i * p for i, p in enumerate(checksum)))
