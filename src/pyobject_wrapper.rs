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
