use crate::pyiterator::{PyBTreeIterator, PyBTreeKeyIterator, PyBTreeValueIterator};
use crate::pyobject_wrapper::Elem;
use pyo3::prelude::*;
use std::collections::{btree_map, BTreeMap};

#[pyclass]
pub struct PyBTreeMap {
    pub tree: BTreeMap<Elem, Elem>,
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
        // cast to orderable type
        let (key, value) = Python::with_gil(|py| {
            let key = key.extract::<Elem>(py).unwrap();
            let value = value.extract::<Elem>(py).unwrap();
            (key, value)
        });

        self.tree.insert(key, value);
    }

    pub fn get(&self, key: PyObject) -> Option<PyObject> {
        Python::with_gil(|py| {
            let key = key.extract::<Elem>(py).unwrap();
            self.tree.get(&key).map(|x| x.to_pyobject(py))
        })
    }

    pub fn remove(&mut self, key: PyObject) -> Option<PyObject> {
        Python::with_gil(|py| {
            let key = key.extract::<Elem>(py).unwrap();
            self.tree.remove(&key).map(|x| x.into_py(py))
        })
    }

    pub fn __len__(&self) -> usize {
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
                        btree_map::Keys<'_, Elem, Elem>,
                        btree_map::Keys<'static, Elem, Elem>,
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
                        btree_map::Values<'_, Elem, Elem>,
                        btree_map::Values<'static, Elem, Elem>,
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
                        btree_map::Iter<'_, Elem, Elem>,
                        btree_map::Iter<'static, Elem, Elem>,
                    >(iter)
                },
            };
        });
    }

    fn __iter__(slf: PyRef<Self>) -> PyBTreeKeyIterator {
        PyBTreeMap::keys(slf)
    }
}
