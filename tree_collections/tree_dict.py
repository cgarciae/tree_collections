from tree_collections.tree_collections import PyTreeDict
import typing as tp

K = tp.TypeVar("K")
V = tp.TypeVar("V")

if tp.TYPE_CHECKING:

  class TreeDict(PyTreeDict[K, V]):
    pass

else:

  class TreeDict(tp.MutableMapping[K, V]):

    def __init__(self, *args, **kwargs) -> None:
      self._tree = PyTreeDict(*args, **kwargs)

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
