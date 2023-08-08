mod pybtree_map;
mod pyiterator;
mod pyobject_wrapper;

use pybtree_map::PyBTreeMap;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn tree_collections(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBTreeMap>()?;
    Ok(())
}
