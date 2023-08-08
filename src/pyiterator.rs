use crate::pyobject_wrapper::PyObjectWrapper;
use pyo3::{exceptions::PyStopIteration, prelude::*};

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
