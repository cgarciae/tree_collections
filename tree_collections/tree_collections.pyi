import typing as tp

K = tp.TypeVar("K")
V = tp.TypeVar("V")

class PyTreeDict(tp.MutableMapping[K, V]):
  def __init__(
      self, other: tp.Union[tp.Mapping[K, V], tp.Iterable[tp.Tuple[K, V]], None] = None
  ) -> None: ...
  def nth(self, n: int) -> tp.Tuple[K, V]: ...
