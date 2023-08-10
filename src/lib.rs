mod elem;
mod iterators;
mod pybtree_map;
mod pybtree_seq;
mod pybtree_set;

use pybtree_map::PyBTreeMap;
use pybtree_seq::PyBTreeSeq;
use pybtree_set::PyBTreeSet;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn tree_collections(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBTreeMap>()?;
    m.add_class::<PyBTreeSet>()?;
    m.add_class::<PyBTreeSeq>()?;
    Ok(())
}
