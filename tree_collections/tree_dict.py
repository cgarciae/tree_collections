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


class Missing:
  pass


MISSING = Missing()


class TreeDict(tp.MutableMapping[K, V]):
  if tp.TYPE_CHECKING:
    _tree: PyBTreeMap[K, V]

  def __init__(
      self,
      other: tp.Union[tp.Mapping[K, V], tp.Iterable[tp.Tuple[K, V]], None] = None,
  ):
    if other is not None:
      self._tree = PyBTreeMap(other)
    else:
      self._tree = PyBTreeMap()

  def __getitem__(self, key: K) -> V:
    value = self._tree.get(key)
    if value is None:
      raise KeyError
    return value

  def __setitem__(self, key: K, value: V) -> None:
    self._tree.insert(key, value)

  def __delitem__(self, key: K) -> None:
    result = self._tree.remove(key)
    if result is None:
      raise KeyError

  def __iter__(self) -> tp.Iterator[K]:
    return iter(self._tree.keys())

  def __len__(self) -> int:
    return self._tree.len()

  def __contains__(self, key: object) -> bool:
    return self._tree.contains_key(key)

  def clear(self) -> None:
    self._tree.clear()

  @tp.overload
  def pop(self, __key: K) -> V:
    ...

  @tp.overload
  def pop(self, __key: K, default: tp.Union[V, T]) -> tp.Union[V, T]:
    ...

  def pop(self, __key: K, default: tp.Union[V, T] = None) -> tp.Union[V, T]:
    raise NotImplemented

  def popitem(self) -> tuple[K, V]:
    raise NotImplemented

  @tp.overload
  def setdefault(self, __key: K) -> tp.Optional[V]:
    ...

  @tp.overload
  def setdefault(self, __key: K, __default: V) -> V:
    ...

  def setdefault(self, __key: K, __default: tp.Optional[V] = None) -> tp.Optional[V]:
    if __key not in self._tree:
      self._tree[__key] = __default  # type: ignore
    return self._tree[__key]

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

  @tp.overload
  def get(self, __key: K) -> V:
    ...

  @tp.overload
  def get(self, __key: K, default: tp.Union[V, T]) -> tp.Union[V, T]:
    ...

  def get(self, __key: K, default: tp.Union[T, Missing] = MISSING) -> tp.Union[V, T]:
    result = self._tree.get(__key)
    if result is None:
      if isinstance(default, Missing):
        raise KeyError
      return default
    return result

  def items(self) -> tp.ItemsView[K, V]:
    return self._tree.items()

  def keys(self) -> tp.KeysView[K]:
    return self._tree.keys()

  def values(self) -> tp.ValuesView[V]:
    return self._tree.values()

  def nth(self, n: int) -> tuple[K, V]:
    output = self._tree.nth(n)
    if output is None:
      raise IndexError
    return output

  def __eq__(self, __other: object) -> bool:
    return self._tree == __other
