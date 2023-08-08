import tree_collections as tc


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
