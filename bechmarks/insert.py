"""
Compare sortedcollections vs tree_collections
"""

import time
import random
import sortedcollections
import tree_collections
import itertools


def random_float():
  return random.random()


def random_string():
  return str(random.random())


def random_int():
  return random.randint(0, 100_000_000)


combinations = list(
    itertools.product(
        [
            random_float,
        ],  # random_string, random_int],
        [
            random_float,
        ],  # random_string, random_int],
    )
)


N = 1_000_000


def test_sortedcollections(key_fn, value_fn):
  t0 = time.time()
  sorted_dict = sortedcollections.SortedDict()
  for _ in range(N):
    sorted_dict[key_fn()] = value_fn()

  return time.time() - t0


def test_tree_collections(key_fn, value_fn):
  t0 = time.time()
  tree = tree_collections.PyBTreeMap()
  for _ in range(N):
    tree.insert(key_fn(), value_fn())

  return time.time() - t0


for test_fn in [test_sortedcollections, test_tree_collections]:
  ts = []
  for key_fn, value_fn in combinations:
    t = test_sortedcollections(key_fn, value_fn)
    ts.append(t)
    print(
        f"[{test_fn.__name__}]({key_fn.__name__}, {value_fn.__name__}): {t:.3f} seconds"
    )

  t = sum(ts)
  print(f"{test_fn.__name__}: {t:.3f} seconds")
