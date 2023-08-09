import itertools
import random
import time

import pytest
import tree_collections as tc


def random_float():
  return random.random()


def random_string():
  return str(random.random())


def random_int():
  return random.randint(0, 100_000_000)


class TestTreeCollections:

  def test_basic(self):
    tree = tc.TreeDict({})
    tree[1] = "one"
    tree[-1] = "minus one"
    tree[2] = "two"
    tree[-3] = "minus three"

    assert list(tree) == [-3, -1, 1, 2]
    assert list(tree.keys()) == [-3, -1, 1, 2]
    assert list(tree.values()) == ["minus three", "minus one", "one", "two"]
    assert list(tree.items()) == [
        (-3, "minus three"),
        (-1, "minus one"),
        (1, "one"),
        (2, "two"),
    ]

  def test_mutiple_types(self):
    tree = tc.TreeDict({
        1: "one",
        -1: "minus one",
        2: "two",
        -3: "minus three",
        5.2: "five point two",
    })

    assert list(tree) == [-3, -1, 1, 2, 5.2]
    assert list(tree.keys()) == [-3, -1, 1, 2, 5.2]
    assert list(tree.values()) == [
        "minus three",
        "minus one",
        "one",
        "two",
        "five point two",
    ]
    assert list(tree.items()) == [
        (-3, "minus three"),
        (-1, "minus one"),
        (1, "one"),
        (2, "two"),
        (5.2, "five point two"),
    ]

  def test_combinations(self):
    combinations = itertools.product(
        [random_float, random_string, random_int],
        [random_float, random_string, random_int],
    )

    N = 10

    for key_fn, value_fn in combinations:
      tree = tc.TreeDict((key_fn(), value_fn()) for _ in range(N))

      print("\n\n========================================")
      print(key_fn.__name__, value_fn.__name__)
      print("========================================")
      print(list(tree.items()))

      assert len(tree) == N
      assert len(list(tree.keys())) == N
      assert len(list(tree.values())) == N
      assert len(list(tree.items())) == N

  def test_constructor(self):
    tree = tc.TreeDict({
        3: "three",
        1: "one",
        2: "two",
    })
    assert list(tree) == [1, 2, 3]

  def test_nth(self):
    tree = tc.TreeDict({
        3: "three",
        1: "one",
        2: "two",
    })
    assert tree.nth(0) == (1, "one")
    assert tree.nth(1) == (2, "two")
    assert tree.nth(2) == (3, "three")
    # test negative index
    assert tree.nth(-1) == (3, "three")
    assert tree.nth(-2) == (2, "two")
    assert tree.nth(-3) == (1, "one")
    # test out of bounds
    with pytest.raises(IndexError):
      tree.nth(3)

    with pytest.raises(IndexError):
      tree.nth(-4)

  def test_iter_after_drop(self):
    tree = tc.TreeDict({-i: i for i in range(10)})

    items = tree.items()
    del tree

    print()
    for k, v in items:
      time.sleep(0.2)
      print(k, v)
