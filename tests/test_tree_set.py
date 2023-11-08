import tree_collections as tc


class TestTreeSet:

  def test_basic(self):
    tset = tc.TreeSet(range(10, 0, -1))
    assert list(tset) == list(range(1, 11))

  def test_basic2(self):
    tset = tc.TreeSet([8, 0, 2])
    assert list(tset) == [0, 2, 8]
