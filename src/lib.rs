use std::{
    cell::RefCell,
    collections::{btree_map, BTreeMap},
    sync::{Arc, RwLock},
};

use pyo3::prelude::*;

struct PyObjectWrapper {
    obj: PyObject,
}

impl PyObjectWrapper {
    fn new(obj: PyObject) -> Self {
        PyObjectWrapper { obj }
    }
}

impl PartialEq for PyObjectWrapper {
    fn eq(&self, other: &Self) -> bool {
        return Python::with_gil(|py| -> PyResult<bool> {
            let operator = py.import("operator")?;
            let eq = operator.getattr("eq")?;
            let result = eq.call1((&self.obj, &other.obj))?;
            return Ok(result.extract::<bool>()?);
        })
        .unwrap();
    }
}

impl Eq for PyObjectWrapper {}

impl PartialOrd for PyObjectWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Python::with_gil(|py| -> PyResult<Option<std::cmp::Ordering>> {
            let operator = py.import("operator")?;
            let lt = operator.getattr("lt")?;
            let result = lt.call1((&self.obj, &other.obj))?;
            let result = result.extract::<bool>()?;
            if result {
                return Ok(Some(std::cmp::Ordering::Less));
            }
            let gt = operator.getattr("gt")?;
            let result = gt.call1((&self.obj, &other.obj))?;
            let result = result.extract::<bool>()?;
            if result {
                return Ok(Some(std::cmp::Ordering::Greater));
            }
            return Ok(Some(std::cmp::Ordering::Equal));
        })
        .unwrap();
    }
}

impl Ord for PyObjectWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return Python::with_gil(|py| -> PyResult<std::cmp::Ordering> {
            let operator = py.import("operator")?;
            let lt = operator.getattr("lt")?;
            let result = lt.call1((&self.obj, &other.obj))?;
            let result = result.extract::<bool>()?;
            if result {
                return Ok(std::cmp::Ordering::Less);
            }
            let gt = operator.getattr("gt")?;
            let result = gt.call1((&self.obj, &other.obj))?;
            let result = result.extract::<bool>()?;
            if result {
                return Ok(std::cmp::Ordering::Greater);
            }
            return Ok(std::cmp::Ordering::Equal);
        })
        .unwrap();
    }
}

#[derive(Clone)]
#[pyclass]
struct PyKeysIterator {
    source: Arc<RwLock<BTreeMap<PyObjectWrapper, PyObjectWrapper>>>,
    inner: RefCell<Option<Arc<RwLock<btree_map::Keys<'static, PyObjectWrapper, PyObjectWrapper>>>>>,
}

impl PyKeysIterator {
    fn new(inner: Arc<RwLock<BTreeMap<PyObjectWrapper, PyObjectWrapper>>>) -> Self {
        PyKeysIterator {
            source: inner,
            inner: RefCell::new(None),
        }
    }
}

#[pymethods]
impl PyKeysIterator {
    fn __iter__(&self) -> Self {
        let ret = self.clone();
        let keys = self.source.read().unwrap();
        let keys = Arc::new(RwLock::new(keys.keys()));
        self.inner.borrow_mut().replace(keys);

        return ret;
    }

    fn __next__(&mut self) -> Option<PyObject> {
        self.inner
            .borrow()
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .next()
            .map(|x| x.obj.clone())
    }
}

#[pyclass]
struct PyBTreeMap {
    tree: Arc<RwLock<BTreeMap<PyObjectWrapper, PyObjectWrapper>>>,
}

#[pymethods]
impl PyBTreeMap {
    #[new]
    fn new() -> Self {
        PyBTreeMap {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    fn insert(&mut self, key: PyObject, value: PyObject) {
        let key = PyObjectWrapper::new(key);
        let value = PyObjectWrapper::new(value);
        // cast to orderable type
        self.tree.write().unwrap().insert(key, value);
    }

    fn get(&self, key: PyObject) -> Option<PyObject> {
        let key = PyObjectWrapper::new(key);
        return self.tree.read().unwrap().get(&key).map(|x| x.obj.clone());
    }

    fn remove(&mut self, key: PyObject) -> Option<PyObject> {
        let key = PyObjectWrapper::new(key);
        return self.tree.write().unwrap().remove(&key).map(|x| x.obj);
    }

    fn len(&self) -> usize {
        return self.tree.read().unwrap().len();
    }

    fn is_empty(&self) -> bool {
        return self.tree.read().unwrap().is_empty();
    }

    fn clear(&mut self) {
        self.tree.write().unwrap().clear();
    }

    // fn keys(&self) -> PyKeysIterator {
    //     let clone: &PyCell<PyBTreeMap> =
    //         Python::with_gil(|py| self.into_py(py).clone_ref(py).downcast(py).unwrap());
    //     PyKeysIterator::new(*clone)
    // }

    fn keys(&self) -> PyKeysIterator {
        PyKeysIterator::new(self.tree.clone())
    }

    // fn items(&self) -> PyIterator {
    //     let iter = self.tree.iter().map(|elems| {
    //         Python::with_gil(|py| {
    //             let tuple = Py::new(py, PyTuple::new(py, &[&elems.0.obj, &elems.1.obj])).unwrap();
    //             return tuple;
    //         })
    //     });
    //     return PyIterator::new(Box::new(iter.cloned()));
    // }
}

/// A Python module implemented in Rust.
#[pymodule]
fn tree_collections(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBTreeMap>()?;
    Ok(())
}
