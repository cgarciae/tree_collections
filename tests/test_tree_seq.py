import tree_collections as tc


class TestTreeSet:

  def test_basic(self):
    nums = [1, 2, 1, 2, 3, 1, 0, -1, 1]
    tset = tc.TreeSeq(nums)

    assert list(tset) == [-1, 0, 1, 1, 1, 1, 2, 2, 3]

    assert tset[0] == -1
    assert tset[1] == 0
    assert tset[2] == 1
    assert tset[3] == 1
    assert tset[4] == 1
    assert tset[5] == 1
    assert tset[6] == 2
    assert tset[7] == 2
    assert tset[8] == 3
    # negative indices
    assert tset[-1] == 3
    assert tset[-2] == 2
    assert tset[-3] == 2
    assert tset[-4] == 1
    assert tset[-5] == 1
    assert tset[-6] == 1
    assert tset[-7] == 1
    assert tset[-8] == 0
    assert tset[-9] == -1
