use pyo3::{prelude::*, types::PyFloat};

// static mut GLOBALS: HashMap<>

#[derive(Debug)]
pub enum Elem {
    Float64(f64),
    Int64(i64),
    String(String),
    TwoTuple(Box<Elem>, Box<Elem>),
    Tuple(Vec<Elem>),
    Vec(Vec<Elem>),
    PyObj(PyObject),
}

impl Elem {
    pub fn to_pyobject(&self, py: Python<'_>) -> PyObject {
        match self {
            Elem::Float64(x) => x.to_object(py),
            Elem::Int64(x) => x.to_object(py),
            Elem::String(s) => s.to_object(py),
            Elem::TwoTuple(a, b) => {
                let a = a.to_pyobject(py);
                let b = b.to_pyobject(py);
                (a, b).to_object(py)
            }
            Elem::Tuple(v) => {
                let v = v.iter().map(|x| x.to_pyobject(py)).collect::<Vec<_>>();
                v.to_object(py)
            }
            Elem::Vec(v) => {
                let v = v.iter().map(|x| x.to_pyobject(py)).collect::<Vec<_>>();
                v.to_object(py)
            }
            Elem::PyObj(obj) => obj.clone(),
        }
    }
}

impl IntoPy<PyObject> for Elem {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Elem::Float64(x) => PyFloat::new(py, x).to_object(py),
            Elem::Int64(x) => x.to_object(py),
            Elem::String(s) => s.to_object(py),
            Elem::TwoTuple(a, b) => {
                let a = a.into_py(py);
                let b = b.into_py(py);
                (a, b).to_object(py)
            }
            Elem::Tuple(v) => {
                let v = v.into_iter().map(|x| x.into_py(py)).collect::<Vec<_>>();
                v.to_object(py)
            }
            Elem::Vec(v) => {
                let v = v.into_iter().map(|x| x.into_py(py)).collect::<Vec<_>>();
                v.to_object(py)
            }
            Elem::PyObj(obj) => obj,
        }
    }
}

impl FromPyObject<'_> for Elem {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        if let Ok(float) = ob.extract::<f64>() {
            return Ok(Elem::Float64(float));
        }
        if let Ok(int) = ob.extract::<i64>() {
            return Ok(Elem::Int64(int));
        }
        if let Ok(string) = ob.extract::<String>() {
            return Ok(Elem::String(string));
        }
        if let Ok(tuple) = ob.extract::<(Elem, Elem)>() {
            return Ok(Elem::TwoTuple(Box::new(tuple.0), Box::new(tuple.1)));
        }
        if let Ok(tuple) = ob.extract::<Vec<Elem>>() {
            return Ok(Elem::Tuple(tuple));
        }
        if let Ok(vec) = ob.extract::<Vec<Elem>>() {
            return Ok(Elem::Vec(vec));
        }
        return Ok(Elem::PyObj(ob.to_object(ob.py())));
    }
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Float64
            (Elem::Float64(a), Elem::Float64(b)) => a == b,
            (Elem::Float64(a), Elem::Int64(b)) => *a == *b as f64,
            // Int64
            (Elem::Int64(a), Elem::Float64(b)) => *a as f64 == *b as f64,
            (Elem::Int64(a), Elem::Int64(b)) => a == b,
            // String
            (Elem::String(a), Elem::String(b)) => a == b,
            // TwoTuple
            (Elem::TwoTuple(a1, a2), Elem::TwoTuple(b1, b2)) => a1 == b1 && a2 == b2,
            // Tuple
            (Elem::Tuple(a), Elem::Tuple(b)) => a == b,
            // Vec
            (Elem::Vec(a), Elem::Vec(b)) => a == b,
            // PyObjects
            (Elem::PyObj(a), Elem::PyObj(b)) => {
                Python::with_gil(|py| -> PyResult<bool> { pyobject_eq(py, a, b) }).unwrap()
            }
            (a, b) => panic!("Comparison not supported: {a:?} == {b:?}"),
        }
    }
}

impl Eq for Elem {}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Float64
            (Elem::Float64(a), Elem::Float64(b)) => a.partial_cmp(b),
            (Elem::Float64(a), Elem::Int64(b)) => a.partial_cmp(&(*b as f64)),
            // Int64
            (Elem::Int64(a), Elem::Float64(b)) => (*a as f64).partial_cmp(b),
            (Elem::Int64(a), Elem::Int64(b)) => a.partial_cmp(b),
            // Strings
            (Elem::String(a), Elem::String(b)) => a.partial_cmp(b),
            // TwoTuple
            (Elem::TwoTuple(a1, a2), Elem::TwoTuple(b1, b2)) => match a1.partial_cmp(b1) {
                Some(std::cmp::Ordering::Equal) => a2.partial_cmp(b2),
                x => x,
            },
            // Tuple
            (Elem::Tuple(a), Elem::Tuple(b)) => a.partial_cmp(b),
            // Vec
            (Elem::Vec(a), Elem::Vec(b)) => a.partial_cmp(b),
            // PyObjects
            (Elem::PyObj(a), Elem::PyObj(b)) => {
                Python::with_gil(|py| pyobject_partial_cmp(py, a, b)).unwrap()
            }
            (a, b) => panic!("Comparison not supported: {a:?} == {b:?}"),
        }
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // Float64
            (Elem::Float64(a), Elem::Float64(b)) => a.partial_cmp(b).unwrap(),
            (Elem::Float64(a), Elem::Int64(b)) => a.partial_cmp(&(*b as f64)).unwrap(),
            // Int64
            (Elem::Int64(a), Elem::Float64(b)) => (*a as f64).partial_cmp(b).unwrap(),
            (Elem::Int64(a), Elem::Int64(b)) => a.partial_cmp(b).unwrap(),
            // Strings
            (Elem::String(a), Elem::String(b)) => a.cmp(b),
            // TwoTuple
            (Elem::TwoTuple(a1, a2), Elem::TwoTuple(b1, b2)) => match a1.partial_cmp(b1) {
                Some(std::cmp::Ordering::Equal) => a2.partial_cmp(b2).unwrap(),
                x => x.unwrap(),
            },
            // Tuple
            (Elem::Tuple(a), Elem::Tuple(b)) => a.cmp(b),
            // Vec
            (Elem::Vec(a), Elem::Vec(b)) => a.cmp(b),
            // PyObjects
            (Elem::PyObj(a), Elem::PyObj(b)) => {
                Python::with_gil(|py| pyobject_cmp(py, a, b)).unwrap()
            }
            (a, b) => panic!("Comparison not supported: {a:?} == {b:?}"),
        }
    }
}

fn pyobject_eq(py: Python, a: &PyObject, b: &PyObject) -> PyResult<bool> {
    let operator = py.import("operator")?;
    return operator.call_method1("eq", (a, b))?.extract::<bool>();
}

fn pyobject_partial_cmp(
    py: Python,
    a: &PyObject,
    b: &PyObject,
) -> PyResult<Option<std::cmp::Ordering>> {
    let operator = py.import("operator")?;

    let lt = operator.call_method1("lt", (a, b))?.extract::<bool>()?;
    if lt {
        return Ok(Some(std::cmp::Ordering::Less));
    }

    let gt = operator.call_method1("gt", (a, b))?.extract::<bool>()?;
    if gt {
        return Ok(Some(std::cmp::Ordering::Greater));
    }

    let eq = operator.call_method1("eq", (a, b))?.extract::<bool>()?;
    if eq {
        return Ok(Some(std::cmp::Ordering::Equal));
    }

    return Ok(None);
}

fn pyobject_cmp(py: Python, a: &PyObject, b: &PyObject) -> PyResult<std::cmp::Ordering> {
    let operator = py.import("operator")?;

    let lt = operator.call_method1("lt", (a, b))?.extract::<bool>()?;
    if lt {
        return Ok(std::cmp::Ordering::Less);
    }

    let gt = operator.call_method1("gt", (a, b))?.extract::<bool>()?;
    if gt {
        return Ok(std::cmp::Ordering::Greater);
    }

    Ok(std::cmp::Ordering::Equal)
}
