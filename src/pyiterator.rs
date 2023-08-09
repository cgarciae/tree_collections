use std::collections::btree_map;

use crate::pyobject_wrapper::Elem;
use pyo3::prelude::*;

// -------------------
// PyBTreeKeyIterator
// -------------------
#[pyclass]
pub struct PyBTreeKeyIterator {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Keys<'static, Elem, Elem>,
}

#[pymethods]
impl PyBTreeKeyIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        self.iter
            .next()
            .map(|x| Python::with_gil(|py| x.to_pyobject(py)))
    }
}

// -------------------
// PyBTreeValueIterator
// -------------------
#[pyclass]
pub struct PyBTreeValueIterator {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Values<'static, Elem, Elem>,
}

#[pymethods]
impl PyBTreeValueIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        self.iter
            .next()
            .map(|x| Python::with_gil(|py| x.to_pyobject(py)))
    }
}

// -------------------
// PyBTreeItemsIterator
// -------------------
#[pyclass]
pub struct PyBTreeIterator {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Iter<'static, Elem, Elem>,
}

#[pymethods]
impl PyBTreeIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<(PyObject, PyObject)> {
        self.iter
            .next()
            .map(|(k, v)| Python::with_gil(|py| (k.to_pyobject(py), v.to_pyobject(py))))
    }
}
