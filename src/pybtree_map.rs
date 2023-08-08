use crate::pyiterator::{PyBTreeIterator, PyBTreeKeyIterator, PyBTreeValueIterator};
use crate::pyobject_wrapper::PyObjectWrapper;
use pyo3::prelude::*;
use std::collections::{btree_map, BTreeMap};

#[pyclass]
pub struct PyBTreeMap {
    pub tree: BTreeMap<PyObjectWrapper, PyObjectWrapper>,
}

#[pymethods]
impl PyBTreeMap {
    #[new]
    pub fn new() -> Self {
        PyBTreeMap {
            tree: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: PyObject, value: PyObject) {
        let key = PyObjectWrapper::new(key);
        let value = PyObjectWrapper::new(value);
        // cast to orderable type
        self.tree.insert(key, value);
    }

    pub fn get(&self, key: PyObject) -> Option<&PyObject> {
        let key = PyObjectWrapper::new(key);
        return self.tree.get(&key).map(|x| &x.obj);
    }

    pub fn remove(&mut self, key: PyObject) -> Option<PyObject> {
        let key = PyObjectWrapper::new(key);
        return self.tree.remove(&key).map(|x| x.obj);
    }

    pub fn len(&self) -> usize {
        return self.tree.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.tree.is_empty();
    }

    pub fn clear(&mut self) {
        self.tree.clear();
    }

    pub fn keys(slf: PyRef<'_, Self>) -> PyBTreeKeyIterator {
        let slf = &slf;
        return Python::with_gil(|py| {
            let owner = slf.into_py(py);
            let iter = slf.tree.keys();

            return PyBTreeKeyIterator {
                owner: owner.clone(),
                iter: unsafe {
                    std::mem::transmute::<
                        btree_map::Keys<'_, PyObjectWrapper, PyObjectWrapper>,
                        btree_map::Keys<'static, PyObjectWrapper, PyObjectWrapper>,
                    >(iter)
                },
            };
        });
    }

    pub fn values(slf: PyRef<'_, Self>) -> PyBTreeValueIterator {
        let slf = &slf;
        return Python::with_gil(|py| {
            let owner = slf.into_py(py);
            let iter = slf.tree.values();

            return PyBTreeValueIterator {
                owner: owner.clone(),
                iter: unsafe {
                    std::mem::transmute::<
                        btree_map::Values<'_, PyObjectWrapper, PyObjectWrapper>,
                        btree_map::Values<'static, PyObjectWrapper, PyObjectWrapper>,
                    >(iter)
                },
            };
        });
    }

    pub fn items(slf: PyRef<'_, Self>) -> PyBTreeIterator {
        let slf = &slf;
        return Python::with_gil(|py| {
            let owner = slf.into_py(py);
            let iter = slf.tree.iter();

            return PyBTreeIterator {
                owner: owner.clone(),
                iter: unsafe {
                    std::mem::transmute::<
                        btree_map::Iter<'_, PyObjectWrapper, PyObjectWrapper>,
                        btree_map::Iter<'static, PyObjectWrapper, PyObjectWrapper>,
                    >(iter)
                },
            };
        });
    }

    fn __iter__(slf: PyRef<Self>) -> PyBTreeKeyIterator {
        PyBTreeMap::keys(slf)
    }
}
