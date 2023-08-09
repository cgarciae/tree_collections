import itertools
import random
import tree_collections as tc


def random_float():
  return random.random()


def random_string():
  return str(random.random())


def random_int():
  return random.randint(0, 100_000_000)


class TestTreeCollections:

  def test_basic(self):
    tree = tc.PyBTreeMap()
    tree.insert(1, "one")
    tree.insert(-1, "minus one")
    tree.insert(2, "two")
    tree.insert(-3, "minus three")

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
    tree = tc.PyBTreeMap()
    tree.insert(1, "one")
    tree.insert(-1, "minus one")
    tree.insert(2, "two")
    tree.insert(-3, "minus three")
    tree.insert(5.2, "five point two")

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
      tree = tc.PyBTreeMap()
      for _ in range(N):
        tree.insert(key_fn(), value_fn())

      print("\n\n========================================")
      print(key_fn.__name__, value_fn.__name__)
      print("========================================")
      print(list(tree.items()))

      assert len(tree) == N
      assert len(list(tree.keys())) == N
      assert len(list(tree.values())) == N
      assert len(list(tree.items())) == N
