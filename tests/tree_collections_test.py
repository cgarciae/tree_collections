import tree_collections as tc


class TestTreeCollections:

  def test_basic(self):
    assert "3" == tc.sum_as_string(1, 2)

  def test_pybtreemap(self):
    tree = tc.PyBTreeMap()
    tree
