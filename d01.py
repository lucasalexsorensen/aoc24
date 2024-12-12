from collections import Counter

with open("d1.txt") as f:
    lines = f.readlines()

left, right = list(zip(*((int(x) for x in line.strip().split()) for line in lines)))

left, right = sorted(left), sorted(right)
print("p1", sum(abs(l - r) for l, r in zip(left, right)))
c_right = Counter(right)
print("p2", sum(k * c_right[k] for k in left))
