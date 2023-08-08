use pyo3::prelude::*;

pub struct PyObjectWrapper {
    pub obj: PyObject,
}

impl PyObjectWrapper {
    pub fn new(obj: PyObject) -> Self {
        PyObjectWrapper { obj }
    }
}

impl PartialEq for PyObjectWrapper {
    fn eq(&self, other: &Self) -> bool {
        Python::with_gil(|py| -> PyResult<bool> {
            self.obj
                .call_method1(py, "__eq__", (&other.obj,))?
                .extract::<bool>(py)
        })
        .unwrap()
    }
}

impl Eq for PyObjectWrapper {}

impl PartialOrd for PyObjectWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Python::with_gil(|py| -> PyResult<std::cmp::Ordering> {
            let gt = self
                .obj
                .call_method1(py, "__gt__", (&other.obj,))?
                .extract::<bool>(py)?;
            if gt {
                return Ok(std::cmp::Ordering::Greater);
            }
            let lt = self
                .obj
                .call_method1(py, "__lt__", (&other.obj,))?
                .extract::<bool>(py)?;
            if lt {
                return Ok(std::cmp::Ordering::Less);
            }
            return Ok(std::cmp::Ordering::Equal);
        })
        .ok();
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
