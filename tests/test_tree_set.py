import tree_collections as tc


class TestTreeSet:

  def test_basic(self):
    tset = tc.TreeSet(range(10, 0, -1))

    assert list(tset) == list(range(1, 11))
