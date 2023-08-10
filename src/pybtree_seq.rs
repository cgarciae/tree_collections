use crate::elem::Elem;
use crate::iterators::{InternalPyBTreeSeqIter, PyBTreeSeqIter};
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PySequence};
use std::collections::{btree_map, BTreeMap};

#[pyclass]
pub struct PyBTreeSeq {
    pub btree_map: BTreeMap<Elem, usize>,
    pub length: usize,
}

unsafe impl Send for PyBTreeSeq {}

#[pymethods]
impl PyBTreeSeq {
    #[new]
    #[pyo3(signature = (input=None))]
    pub fn new(input: Option<PyObject>, py: Python) -> PyResult<Self> {
        let mut btree_map = BTreeMap::<Elem, usize>::new();
        let mut length: usize = 0;

        if let Some(input) = input {
            let iter: &PyIterator = if let Ok(input) = input.downcast::<PySequence>(py) {
                input.iter()?
            } else if let Ok(input) = input.downcast::<PyIterator>(py) {
                input
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(
                    "Expected a sequence or iterable",
                ));
            };

            for x in iter {
                let elem = x?.downcast::<PyAny>()?;
                let elem = elem.extract::<Elem>()?;
                btree_map.entry(elem).and_modify(|x| *x += 1).or_insert(1);
                length += 1;
            }
        }

        Ok(PyBTreeSeq {
            btree_map: btree_map,
            length: length,
        })
    }

    pub fn insert(mut slf: PyRefMut<'_, Self>, key: PyObject) -> PyResult<bool> {
        // cast to orderable type
        let py = slf.py();
        let elem = key.extract::<Elem>(py)?;
        let output = slf
            .btree_map
            .entry(elem)
            .and_modify(|x| *x += 1)
            .or_insert(1);

        Ok(output == &1)
    }

    pub fn get(slf: PyRef<'_, Self>, key: PyObject) -> PyResult<Option<PyObject>> {
        let py = slf.py();
        let key = key.extract::<Elem>(py)?;
        let output = slf.btree_map.get_key_value(&key);

        Ok(output.map(|(x, _)| x.to_pyobject(py)))
    }

    pub fn remove(mut slf: PyRefMut<'_, Self>, key: PyObject) -> PyResult<bool> {
        let py = slf.py();
        let key = key.extract::<Elem>(py)?;
        let entry = slf.btree_map.entry(key).and_modify(|x| *x -= 1);

        match entry {
            btree_map::Entry::Vacant(_) => Ok(false),
            btree_map::Entry::Occupied(entry) => {
                if *entry.get() == 0 {
                    entry.remove();
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
        }
    }

    pub fn contains(slf: PyRef<'_, Self>, key: PyObject) -> PyResult<bool> {
        let py = slf.py();
        let elem_key = key.extract::<Elem>(py)?;
        Ok(slf.btree_map.contains_key(&elem_key))
    }

    pub fn nth(mut slf: PyRefMut<'_, Self>, mut n: i64) -> PyResult<Option<PyObject>> {
        let py = slf.py();

        if n >= slf.length as i64 {
            return Ok(None);
        }
        if n < 0 {
            n = slf.length as i64 + n;
        }
        if n < 0 {
            return Ok(None);
        }
        let n = n as usize;

        let output = if n == 0 {
            let entry = slf.btree_map.first_entry().unwrap();
            entry.key().to_pyobject(py)
        } else if n == slf.length - 1 {
            let entry = slf.btree_map.last_entry().unwrap();
            entry.key().to_pyobject(py)
        } else {
            let key = PyBTreeSeq::interal_iter(&slf).nth(n).unwrap();
            key.to_pyobject(py)
        };

        Ok(Some(output))
    }

    pub fn len(&self) -> usize {
        return self.btree_map.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.btree_map.is_empty();
    }

    pub fn clear(&mut self) {
        self.btree_map.clear();
    }

    pub fn iter(slf: PyRefMut<'_, Self>) -> PyBTreeSeqIter {
        let slf = &slf;
        let owner = slf.into_py(slf.py());
        let iter = PyBTreeSeq::interal_iter(slf);

        return PyBTreeSeqIter {
            py_obj: owner.clone(),
            iter: unsafe {
                std::mem::transmute::<InternalPyBTreeSeqIter<'_>, InternalPyBTreeSeqIter<'static>>(
                    iter,
                )
            },
        };
    }
}

impl PyBTreeSeq {
    fn interal_iter<'a>(slf: &'a PyRefMut<'_, Self>) -> InternalPyBTreeSeqIter<'a> {
        let slf = &slf;
        let iter = slf.btree_map.iter();

        return InternalPyBTreeSeqIter {
            iter: iter,
            elem: None,
            elem_count: 0,
        };
    }
}
