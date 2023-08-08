import tree_collections as tc


class TestTreeCollections:

  def test_basic(self):
    tree = tc.PyBTreeMap()
    tree.insert(1, "one")
    tree.insert(-1, "minus one")
    tree.insert(2, "two")
    tree.insert(-3, "minus three")

    assert tree.keys() == [-3, -1, 1, 2]
    assert tree.values() == ["minus three", "minus one", "one", "two"]
    # assert tree.items() == [
    #     (-3, "minus three"),
    #     (-1, "minus one"),
    #     (1, "one"),
    #     (2, "two"),
    # ]

    iterator = iter(tree)
    iterator
