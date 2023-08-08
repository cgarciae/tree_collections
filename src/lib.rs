use std::collections::BTreeMap;

use pyo3::{exceptions::PyStopIteration, prelude::*};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

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

#[pyclass]
struct PyIterator {
    iter: Box<dyn Iterator<Item = PyObjectWrapper>>,
}

unsafe impl Send for PyIterator {}
impl PyIterator {
    fn new(iter: Box<dyn Iterator<Item = PyObjectWrapper>>) -> Self {
        PyIterator { iter }
    }
}

#[pymethods]
impl PyIterator {
    fn next(&mut self) -> PyResult<PyObject> {
        match self.iter.next() {
            Some(x) => Ok(x.obj),
            None => Err(PyErr::new::<PyStopIteration, _>(())),
        }
    }
}

#[pyclass]
struct PyBTreeMap {
    tree: BTreeMap<PyObjectWrapper, PyObjectWrapper>,
}

#[pymethods]
impl PyBTreeMap {
    #[new]
    fn new() -> Self {
        PyBTreeMap {
            tree: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: PyObject, value: PyObject) {
        let key = PyObjectWrapper::new(key);
        let value = PyObjectWrapper::new(value);
        // cast to orderable type
        self.tree.insert(key, value);
    }

    fn get(&self, key: PyObject) -> Option<&PyObject> {
        let key = PyObjectWrapper::new(key);
        return self.tree.get(&key).map(|x| &x.obj);
    }

    fn remove(&mut self, key: PyObject) -> Option<PyObject> {
        let key = PyObjectWrapper::new(key);
        return self.tree.remove(&key).map(|x| x.obj);
    }

    fn len(&self) -> usize {
        return self.tree.len();
    }

    fn is_empty(&self) -> bool {
        return self.tree.is_empty();
    }

    fn clear(&mut self) {
        self.tree.clear();
    }

    fn keys(&self) -> Vec<PyObject> {
        let keys = self.tree.keys().map(|x| x.obj.clone());
        return keys.collect();
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
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PyBTreeMap>()?;
    Ok(())
}
