from tree_collections.tree_collections import PyBTreeSet
import typing as tp

K = tp.TypeVar("K")
B = tp.TypeVar("B")
_T_co = tp.TypeVar("_T_co", covariant=True)

tp.Sequence.index


class TreeSet(tp.MutableSet[K]):
  if tp.TYPE_CHECKING:
    _tree: PyBTreeSet[K]

  def __init__(self, __input: tp.Optional[tp.Iterable[K]] = None, /):
    if __input is not None:
      self._tree = PyBTreeSet(__input)
    else:
      self._tree = PyBTreeSet()

  def __iter__(self) -> tp.Iterator[K]:
    return iter(self._tree.iter())

  def add(self, value: K) -> None:
    self._tree.insert(value)

  def discard(self, value: K) -> None:
    self._tree.remove(value)

  # Mixin methods
  def clear(self) -> None:
    self._tree.clear()

  def pop(self) -> K:
    raise NotImplementedError

  def remove(self, value: K) -> None:
    raise NotImplementedError

  def __len__(self) -> int:
    return self._tree.len()

  def __contains__(self, __x: object) -> bool:
    return self._tree.contains(__x)

  def __ior__(self, it: tp.AbstractSet[K]) -> tp.Self:
    raise NotImplementedError  # type: ignore[override,misc]

  def __iand__(self, it: tp.AbstractSet[tp.Any]) -> tp.Self:
    raise NotImplementedError

  def __ixor__(self, it: tp.AbstractSet[K]) -> tp.Self:
    raise NotImplementedError  # type: ignore[override,misc]

  def __isub__(self, it: tp.AbstractSet[tp.Any]) -> tp.Self:
    raise NotImplementedError

  def _hash(self) -> int:
    raise NotImplementedError

  # Mixin methods
  def __le__(self, other: tp.AbstractSet[tp.Any]) -> bool:
    raise NotImplementedError

  def __lt__(self, other: tp.AbstractSet[tp.Any]) -> bool:
    raise NotImplementedError

  def __gt__(self, other: tp.AbstractSet[tp.Any]) -> bool:
    raise NotImplementedError

  def __ge__(self, other: tp.AbstractSet[tp.Any]) -> bool:
    raise NotImplementedError

  def __and__(self, other: tp.AbstractSet[B]) -> tp.AbstractSet[tp.Union[B, K]]:
    raise NotImplementedError

  def __or__(self, other: tp.AbstractSet[B]) -> tp.AbstractSet[tp.Union[B, K]]:
    raise NotImplementedError

  def __sub__(self, other: tp.AbstractSet[K]) -> tp.AbstractSet[K]:
    raise NotImplementedError

  def __xor__(self, other: tp.AbstractSet[B]) -> tp.AbstractSet[tp.Union[B, K]]:
    raise NotImplementedError

  def __eq__(self, other: object) -> bool:
    raise NotImplementedError

  def isdisjoint(self, other: tp.Iterable[tp.Any]) -> bool:
    raise NotImplementedError
