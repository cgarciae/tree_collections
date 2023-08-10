use std::collections::{btree_map, btree_set};

use crate::elem::Elem;
use pyo3::prelude::*;

// -------------------
// PyBTreeMapKeys
// -------------------
#[pyclass]
pub struct PyBTreeMapKeys {
    #[pyo3(get)]
    pub py_obj: PyObject,
    // pub py_ref: Arc<RwLock<PyRef<'static, PyBTreeMap>>>,
    pub iter: btree_map::Keys<'static, Elem, Elem>,
}

#[pymethods]
impl PyBTreeMapKeys {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.iter.next().map(|x| x.to_pyobject(slf.py()))
    }
}

// -------------------
// PyBTreeMapValues
// -------------------
#[pyclass]
pub struct PyBTreeMapValues {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Values<'static, Elem, Elem>,
}

#[pymethods]
impl PyBTreeMapValues {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.iter.next().map(|x| x.to_pyobject(slf.py()))
    }
}

// -------------------
// PyBTreeMapIter
// -------------------
#[pyclass]
pub struct PyBTreeMapIter {
    #[pyo3(get)]
    pub owner: PyObject,
    pub iter: btree_map::Iter<'static, Elem, Elem>,
}

#[pymethods]
impl PyBTreeMapIter {
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

// -------------------
// PyBTreeSetIter
// -------------------
#[pyclass]
pub struct PyBTreeSetIter {
    #[pyo3(get)]
    pub py_obj: PyObject,
    // pub py_ref: Arc<RwLock<PyRef<'static, PyBTreeMap>>>,
    pub iter: btree_set::Iter<'static, Elem>,
}

#[pymethods]
impl PyBTreeSetIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.iter.next().map(|x| x.to_pyobject(slf.py()))
    }
}

// -------------------
// PyBTreeSeqIter
// -------------------
#[pyclass]
pub struct PyBTreeSeqIter {
    #[pyo3(get)]
    pub py_obj: PyObject,
    // pub py_ref: Arc<RwLock<PyRef<'static, PyBTreeMap>>>,
    pub iter: InternalPyBTreeSeqIter<'static>,
}

#[pymethods]
impl PyBTreeSeqIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        slf.iter.next().map(|x| x.to_pyobject(slf.py()))
    }
}

pub struct InternalPyBTreeSeqIter<'a> {
    pub iter: btree_map::Iter<'a, Elem, usize>,
    pub elem: Option<&'a Elem>,
    pub elem_count: usize,
}

impl<'a> Iterator for InternalPyBTreeSeqIter<'a> {
    type Item = &'a Elem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.elem.is_none() {
            let (elem, count) = self.iter.next()?;
            self.elem = Some(elem);
            self.elem_count = *count;
        }

        let elem = self.elem.unwrap();
        if self.elem_count == 0 {
            panic!("invalid state elem_count == 0");
        }

        self.elem_count -= 1;

        if self.elem_count == 0 {
            self.elem = None;
        }

        Some(elem)
    }
}
