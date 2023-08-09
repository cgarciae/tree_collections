from tree_collections.tree_collections import PyTreeDict
import typing as tp

K = tp.TypeVar("K")
B = tp.TypeVar("B")
_T_co = tp.TypeVar("_T_co", covariant=True)


class TreeSet(tp.MutableSet[K]):
  if tp.TYPE_CHECKING:
    _tree: PyTreeDict[K, None]

  def __init__(self, __input: tp.Optional[tp.Iterable[K]] = None, /):
    if __input is not None:
      self._tree = PyTreeDict((key, None) for key in __input)
    else:
      self._tree = PyTreeDict()

  def __iter__(self) -> tp.Iterator[K]:
    return iter(self._tree.keys())

  def add(self, value: K) -> None:
    self._tree[value] = None

  def discard(self, value: K) -> None:
    del self._tree[value]

  # Mixin methods
  def clear(self) -> None:
    self._tree.clear()

  def pop(self) -> K:
    return self._tree.popitem()[0]

  def remove(self, value: K) -> None:
    raise NotImplementedError

  def __len__(self) -> int:
    return len(self._tree)

  def __contains__(self, __x: object) -> bool:
    return __x in self._tree

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
