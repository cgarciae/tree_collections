use std::collections::btree_map;

use crate::elem::Elem;
use pyo3::prelude::*;

// -------------------
// PyBTreeKeyIterator
// -------------------
#[pyclass]
pub struct PyBTreeKeyIterator {
    #[pyo3(get)]
    pub py_obj: PyObject,
    // pub py_ref: Arc<RwLock<PyRef<'static, PyBTreeMap>>>,
    pub iter: btree_map::Keys<'static, Elem, Elem>,
}

#[pymethods]
impl PyBTreeKeyIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.iter.next().map(|x| x.to_pyobject(slf.py()))
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

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.iter.next().map(|x| x.to_pyobject(slf.py()))
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

    fn __next__(mut slf: PyRefMut<Self>) -> Option<(PyObject, PyObject)> {
        let py = slf.py();
        slf.iter
            .next()
            .map(|(k, v)| (k.to_pyobject(py), v.to_pyobject(py)))
    }
}
