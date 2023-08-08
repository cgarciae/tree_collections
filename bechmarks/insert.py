"""
Compare sortedcollections vs tree_collections
"""

import time
import random
import sortedcollections
import tree_collections

N = 1_000_000

t0 = time.time()
sorted_dict = sortedcollections.SortedDict()
for _ in range(N):
  sorted_dict[random.random()] = random.random()

t = time.time() - t0
print(f"sortedcollections.SortedDict: {t:.3f} seconds")

t0 = time.time()
tree = tree_collections.PyBTreeMap()

for _ in range(N):
  tree.insert(random.random(), random.random())

t = time.time() - t0
print(f"tree_collections.PyBTreeMap: {t:.3f} seconds")
