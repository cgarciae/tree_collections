use crate::elem::Elem;
use crate::iterator::{PyBTreeIterator, PyBTreeKeyIterator, PyBTreeValueIterator};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PyMapping, PySequence, PyTuple};
use std::collections::{btree_map, BTreeMap};

#[pyclass]
pub struct PyBTreeMap {
    pub tree: BTreeMap<Elem, Elem>,
}

unsafe impl Send for PyBTreeMap {}

#[pymethods]
impl PyBTreeMap {
    #[new]
    #[pyo3(signature = (input=None))]
    pub fn new(input: Option<PyObject>, py: Python) -> PyResult<Self> {
        let mut btree = BTreeMap::<Elem, Elem>::new();

        if let Some(input) = input {
            let iter: &PyIterator = if let Ok(input) = input.downcast::<PyMapping>(py) {
                input.items()?.iter()?
            } else if let Ok(input) = input.downcast::<PySequence>(py) {
                input.iter()?
            } else if let Ok(input) = input.downcast::<PyIterator>(py) {
                input
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(
                    "Expected a mapping or iterable of tuples",
                ));
            };

            for x in iter {
                let x = x?.downcast::<PyTuple>()?;
                let (key, value) = match (x.get_item(0), x.get_item(1)) {
                    (Ok(key), Ok(value)) => (key, value),
                    _ => {
                        return Err(PyErr::new::<exceptions::PyTypeError, _>(
                            "iterable of tuples must contain two elements",
                        ))
                    }
                };
                let elem_key = key.extract::<Elem>()?;
                let elem_value = value.extract::<Elem>()?;
                btree.insert(elem_key, elem_value);
            }
        }

        Ok(PyBTreeMap { tree: btree })
    }

    pub fn __setitem__(
        mut slf: PyRefMut<'_, Self>,
        key: PyObject,
        value: PyObject,
    ) -> PyResult<()> {
        // cast to orderable type
        let py = slf.py();
        let elem_key = key.extract::<Elem>(py)?;
        let elem_value = value.extract::<Elem>(py)?;
        slf.tree.insert(elem_key, elem_value);

        Ok(())
    }

    pub fn __getitem__(slf: PyRef<'_, Self>, key: PyObject) -> PyResult<PyObject> {
        let py = slf.py();
        let key = key.extract::<Elem>(py)?;

        slf.tree
            .get(&key)
            .map(|x| x.to_pyobject(py))
            .ok_or_else(|| {
                PyErr::new::<exceptions::PyKeyError, _>(format!("Key not found: {key:?}"))
            })
    }

    pub fn pop(
        mut slf: PyRefMut<'_, Self>,
        key: PyObject,
        default: Option<PyObject>,
    ) -> PyResult<PyObject> {
        let py = slf.py();
        let key = key.extract::<Elem>(py)?;
        let output = slf.tree.remove(&key).map(|x| x.into_py(py));

        match output {
            None => match default {
                Some(default) => Ok(default),
                None => Err(PyErr::new::<exceptions::PyKeyError, _>(format!(
                    "Key not found: {key:?}"
                ))),
            },
            Some(output) => Ok(output),
        }
    }

    pub fn popitem(mut slf: PyRefMut<'_, Self>, key: PyObject) -> PyResult<(PyObject, PyObject)> {
        let py = slf.py();
        let elem_key = key.extract::<Elem>(py)?;
        let output = slf.tree.remove(&elem_key).map(|x| x.into_py(py));

        match output {
            None => {
                let key_repr = key.to_string();
                Err(PyErr::new::<exceptions::PyKeyError, _>(format!(
                    "Key not found: {key_repr:?}"
                )))
            }
            Some(value) => Ok((key, value)),
        }
    }

    pub fn __delitem__(mut slf: PyRefMut<'_, Self>, key: PyObject) -> PyResult<()> {
        let py = slf.py();
        let elem_key = key.extract::<Elem>(py)?;
        let result = slf.tree.remove(&elem_key).map(|x| x.into_py(py));

        match result {
            None => {
                let key_repr = key.to_string();
                Err(PyErr::new::<exceptions::PyKeyError, _>(format!(
                    "Key not found: {key_repr:?}"
                )))
            }
            Some(_) => Ok(()),
        }
    }
    pub fn __contains__(slf: PyRef<'_, Self>, key: PyObject) -> PyResult<bool> {
        let py = slf.py();
        let elem_key = key.extract::<Elem>(py)?;
        Ok(slf.tree.contains_key(&elem_key))
    }

    pub fn nth(mut slf: PyRefMut<'_, Self>, n: i64) -> PyResult<(PyObject, PyObject)> {
        let py = slf.py();

        let n = match n {
            n if n < 0 => {
                let n = slf.tree.len() as i64 + n;
                if n < 0 {
                    return Err(PyErr::new::<exceptions::PyIndexError, _>(
                        "index out of range",
                    ));
                }
                n
            }
            n => n,
        } as usize;

        let key_value_fn = |(key, value): (&Elem, &Elem)| -> (PyObject, PyObject) {
            (Elem::to_pyobject(key, py), Elem::to_pyobject(value, py))
        };
        let entry_fn = |entry: btree_map::OccupiedEntry<Elem, Elem>| -> (PyObject, PyObject) {
            key_value_fn((entry.key(), entry.get()))
        };

        let result = if n == 0 {
            slf.tree.first_entry().map(entry_fn)
        } else if n == slf.tree.len() - 1 {
            slf.tree.last_entry().map(entry_fn)
        } else {
            slf.tree.iter().nth(n).map(key_value_fn)
        };

        match result {
            None => Err(PyErr::new::<exceptions::PyIndexError, _>(
                "Index out of range",
            )),
            Some(result) => Ok(result),
        }
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
        let owner = slf.into_py(slf.py());
        let iter = slf.tree.keys();

        return PyBTreeKeyIterator {
            py_obj: owner.clone(),
            // py_ref: slf.clone(),
            iter: unsafe {
                std::mem::transmute::<
                    btree_map::Keys<'_, Elem, Elem>,
                    btree_map::Keys<'static, Elem, Elem>,
                >(iter)
            },
        };
    }

    pub fn values(slf: PyRef<'_, Self>) -> PyBTreeValueIterator {
        let slf = &slf;
        let owner = slf.into_py(slf.py());
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
    }

    pub fn items(slf: PyRef<'_, Self>) -> PyBTreeIterator {
        let slf = &slf;
        let owner = slf.into_py(slf.py());
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
    }

    fn __iter__(slf: PyRef<Self>) -> PyBTreeKeyIterator {
        PyBTreeMap::keys(slf)
    }
}
