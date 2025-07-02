import numpy as np

x = np.loadtxt("d01.txt").astype(int)
left, right = np.sort(x.T, axis=1)
print("p1", abs(left - right).sum())
print("p2", sum(l * (l == right).sum() for l in left))
