from tree_collections.tree_collections import PyBTreeSeq
import typing as tp

K = tp.TypeVar("K")
B = tp.TypeVar("B")
_T_co = tp.TypeVar("_T_co", covariant=True)

tp.Sequence.index


class TreeSeq(tp.Generic[K]):
    if tp.TYPE_CHECKING:
        _tree: PyBTreeSeq[K]

    def __init__(self, __input: tp.Optional[tp.Iterable[K]] = None, /):
        if __input is not None:
            self._tree = PyBTreeSeq(__input)
        else:
            self._tree = PyBTreeSeq()

    def __iter__(self) -> tp.Iterator[K]:
        return iter(self._tree.iter())

    def __getitem__(self, idx: int) -> K:
        output = self._tree.nth(idx)
        if output is None:
            raise IndexError(idx)
        return output
