use crate::elem::Elem;
use crate::iterators::PyBTreeSetIter;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PySequence};
use std::collections::{btree_set, BTreeSet};

#[pyclass]
pub struct PyBTreeSet {
    pub btree_set: BTreeSet<Elem>,
}

unsafe impl Send for PyBTreeSet {}

#[pymethods]
impl PyBTreeSet {
    #[new]
    #[pyo3(signature = (input=None))]
    pub fn new(input: Option<PyObject>, py: Python) -> PyResult<Self> {
        let mut btree_set = BTreeSet::<Elem>::new();

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
                btree_set.insert(elem);
            }
        }

        Ok(PyBTreeSet { btree_set })
    }

    pub fn insert(mut slf: PyRefMut<'_, Self>, key: PyObject) -> PyResult<bool> {
        // cast to orderable type
        let py = slf.py();
        let elem = key.extract::<Elem>(py)?;
        let output = slf.btree_set.insert(elem);

        Ok(output)
    }

    pub fn get(slf: PyRef<'_, Self>, key: PyObject) -> PyResult<Option<PyObject>> {
        let py = slf.py();
        let key = key.extract::<Elem>(py)?;
        let output = slf.btree_set.get(&key);

        Ok(output.map(|x| x.to_pyobject(py)))
    }

    pub fn remove(mut slf: PyRefMut<'_, Self>, key: PyObject) -> PyResult<bool> {
        let py = slf.py();
        let key = key.extract::<Elem>(py)?;
        let output = slf.btree_set.remove(&key);

        Ok(output)
    }

    pub fn contains(slf: PyRef<'_, Self>, key: PyObject) -> PyResult<bool> {
        let py = slf.py();
        let elem_key = key.extract::<Elem>(py)?;
        Ok(slf.btree_set.contains(&elem_key))
    }

    pub fn nth(slf: PyRef<'_, Self>, mut n: i64) -> PyResult<Option<PyObject>> {
        let py = slf.py();

        if n >= slf.btree_set.len() as i64 {
            return Ok(None);
        }
        if n < 0 {
            n = slf.btree_set.len() as i64 + n;
        }
        if n < 0 {
            return Ok(None);
        }
        let n = n as usize;

        let output = if n == 0 {
            slf.btree_set.first()
        } else if n == slf.btree_set.len() - 1 {
            slf.btree_set.last()
        } else {
            slf.btree_set.iter().nth(n)
        };

        Ok(output.map(|x| x.to_pyobject(py)))
    }

    pub fn len(&self) -> usize {
        return self.btree_set.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.btree_set.is_empty();
    }

    pub fn clear(&mut self) {
        self.btree_set.clear();
    }

    pub fn iter(slf: PyRef<'_, Self>) -> PyBTreeSetIter {
        let slf = &slf;
        let owner = slf.into_py(slf.py());
        let iter = slf.btree_set.iter();

        return PyBTreeSetIter {
            py_obj: owner.clone(),
            // py_ref: slf.clone(),
            iter: unsafe {
                std::mem::transmute::<btree_set::Iter<'_, Elem>, btree_set::Iter<'static, Elem>>(
                    iter,
                )
            },
        };
    }
}
