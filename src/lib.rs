mod elem;
mod iterator;
mod tree_dict;

use pyo3::prelude::*;
use tree_dict::PyTreeDict;

/// A Python module implemented in Rust.
#[pymodule]
fn tree_collections(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTreeDict>()?;
    Ok(())
}
