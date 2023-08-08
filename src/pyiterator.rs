use std::collections::btree_map;

use crate::pyobject_wrapper::PyObjectWrapper;
use pyo3::prelude::*;

// -------------------
// PyBTreeKeyIterator
// -------------------
#[pyclass]
pub struct PyBTreeKeyIterator {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Keys<'static, PyObjectWrapper, PyObjectWrapper>,
}

#[pymethods]
impl PyBTreeKeyIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        self.iter.next().map(|x| x.obj.clone())
    }
}

// -------------------
// PyBTreeValueIterator
// -------------------
#[pyclass]
pub struct PyBTreeValueIterator {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Values<'static, PyObjectWrapper, PyObjectWrapper>,
}

#[pymethods]
impl PyBTreeValueIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<PyObject> {
        self.iter.next().map(|x| x.obj.clone())
    }
}

// -------------------
// PyBTreeItemsIterator
// -------------------
#[pyclass]
pub struct PyBTreeIterator {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Iter<'static, PyObjectWrapper, PyObjectWrapper>,
}

#[pymethods]
impl PyBTreeIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(&mut self) -> Option<(PyObject, PyObject)> {
        self.iter
            .next()
            .map(|(k, v)| (k.obj.clone(), v.obj.clone()))
    }
}
