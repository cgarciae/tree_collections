use crate::pyobject_wrapper::PyObjectWrapper;
use pyo3::prelude::*;
use std::collections::BTreeMap;

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

    pub fn keys(&self) -> Vec<PyObject> {
        return self.tree.keys().map(|x| x.obj.clone()).collect();
    }

    pub fn values(&self) -> Vec<PyObject> {
        return self.tree.values().map(|x| x.obj.clone()).collect();
    }

    pub fn items(&self) -> Vec<(PyObject, PyObject)> {
        return self
            .tree
            .iter()
            .map(|(k, v)| (k.obj.clone(), v.obj.clone()))
            .collect();
    }

    pub fn __iter__(&self) -> PyResult<PyObject> {
        return Python::with_gil(|py| {
            let builtins = py.import("builtins")?;
            let py_iter = builtins.getattr("iter")?;
            let iter = py_iter.call1((self.keys(),))?;
            return Ok(iter.into_py(py));
        });
    }
}
