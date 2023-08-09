"""
Compare sortedcollections vs tree_collections
"""

import base64
import time
import random
import numpy as np
import sortedcollections
import tree_collections
import itertools
import secrets

N = 10_000


def random_float():
  return random.random()


def random_string():
  num_bytes = 10  # Number of random bytes you want to generate
  random_bytes = secrets.token_bytes(num_bytes)
  return base64.b64encode(random_bytes).decode("utf-8")


# print(random_string())
# exit()


def random_int():
  return random.randint(0, 100_000_000)


combinations = list(
    itertools.product(
        [
            random_float,
            random_int,
            random_string,
        ],
        [
            random_float,
            random_int,
            random_string,
        ],
    )
)


def test_sortedcollections(key_fn, value_fn):
  t0 = time.time()
  sorted_dict = sortedcollections.SortedDict()
  for _ in range(N):
    sorted_dict[key_fn()] = value_fn()

  return time.time() - t0


def test_tree_collections(key_fn, value_fn):
  t0 = time.time()
  tree = tree_collections.TreeDict()
  for _ in range(N):
    tree[key_fn()] = value_fn()

  return time.time() - t0


ts = {
    test_sortedcollections: [],
    test_tree_collections: [],
}

for key_fn, value_fn in combinations:
  for test_fn in [test_sortedcollections, test_tree_collections]:
    t = test_fn(key_fn, value_fn)
    ts[test_fn].append((key_fn, value_fn, t))
    print(
        f"[{test_fn.__name__}]({key_fn.__name__}, {value_fn.__name__}): {t:.3f} seconds"
    )

for test_fn in [test_sortedcollections, test_tree_collections]:
  t = sum(t for _, _, t in ts[test_fn])
  print(f"{test_fn.__name__}: {t:.3f} seconds")

# create a double bar chart comparing the two
import matplotlib.pyplot as plt

fig, ax = plt.subplots()

times_tree_collections = [t for _, _, t in ts[test_tree_collections]]
times_sortedcollections = [t for _, _, t in ts[test_sortedcollections]]
# remove 'random_' prefix
labels = [
    f"({key_fn.__name__[7:]}, {value_fn.__name__[7:]})"
    for key_fn, value_fn, _ in ts[test_tree_collections]
]

x = np.arange(len(labels))  # the label locations
width = 0.35  # the width of the bars

rects1 = ax.bar(
    x - width / 2,
    times_tree_collections,
    width,
    label="BTreeMap",
)
rects2 = ax.bar(
    x + width / 2,
    times_sortedcollections,
    width,
    label="SortedDict",
)
ax.set_ylabel("Time (seconds)")
ax.set_title(f"Insertion time {N = } elements")
ax.set_xticks(x)
ax.set_xticklabels(labels)
ax.legend()

plt.show()
