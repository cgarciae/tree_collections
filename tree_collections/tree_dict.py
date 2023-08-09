from tree_collections.tree_collections import PyBTreeMap
import typing as tp

K = tp.TypeVar("K")
V = tp.TypeVar("V")
T = tp.TypeVar("T")


class SupportsKeysAndGetItem(tp.Protocol, tp.Generic[K, V]):

  def keys(self) -> tp.Iterable[K]:
    raise NotImplemented

  def __getitem__(self, __key: K) -> V:
    raise NotImplemented


class TreeDict(tp.MutableMapping[K, V]):
  if tp.TYPE_CHECKING:
    _tree: PyBTreeMap[K, V]

  def __init__(
      self, other: tp.Union[tp.Mapping[K, V], tp.Iterable[tp.Tuple[K, V]], None] = None
  ):
    if other is not None:
      self._tree = PyBTreeMap(other)
    else:
      self._tree = PyBTreeMap()

  def __getitem__(self, key: K) -> V:
    return self._tree[key]

  def __setitem__(self, key: K, value: V) -> None:
    self._tree[key] = value

  def __delitem__(self, key: K) -> None:
    del self._tree[key]

  def __iter__(self) -> tp.Iterator[K]:
    return iter(self._tree)

  def __len__(self) -> int:
    return len(self._tree)

  def __contains__(self, key: K) -> bool:
    return key in self._tree

  def __getattr__(self, name: str) -> tp.Any:
    return getattr(self._tree, name)

  def __setitem__(self, __key: K, __value: V) -> None:
    self._tree[__key] = __value

  def __delitem__(self, __key: K) -> None:
    raise NotImplemented

  def clear(self) -> None:
    raise NotImplemented

  @tp.overload
  def pop(self, __key: K) -> V:
    raise NotImplemented

  @tp.overload
  def pop(self, __key: K, default: tp.Union[V, T]) -> tp.Union[V, T]:
    raise NotImplemented

  def popitem(self) -> tuple[K, V]:
    raise NotImplemented

  def setdefault(self, __key: K, __default: V) -> V:
    raise NotImplemented

  @tp.overload
  def update(self, __m: SupportsKeysAndGetItem[K, V], **kwargs: V) -> None:
    ...

  @tp.overload
  def update(self, __m: tp.Iterable[tuple[K, V]], **kwargs: V) -> None:
    ...

  @tp.overload
  def update(self, **kwargs: V) -> None:
    ...

  def update(self, *args, **kwargs: V) -> None:
    raise NotImplemented

  def __getitem__(self, __key: K) -> V:
    return self._tree[__key]

  # Mixin methods
  @tp.overload
  def get(self, __key: K) -> V:
    ...

  @tp.overload
  def get(self, __key: K, default: tp.Union[V, T]) -> tp.Union[V, T]:
    ...

  def get(self, __key: K, default: tp.Union[V, T] = None) -> tp.Union[V, T]:
    return self._tree.get(__key, default)

  def items(self) -> tp.ItemsView[K, V]:
    return self._tree.items()

  def keys(self) -> tp.KeysView[K]:
    return self._tree.keys()

  def values(self) -> tp.ValuesView[V]:
    return self._tree.values()

  def __contains__(self, __key: object) -> bool:
    return __key in self._tree

  def __eq__(self, __other: object) -> bool:
    return self._tree == __other


tp.MutableMapping.update
