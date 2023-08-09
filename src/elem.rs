use pyo3::prelude::*;

// static mut GLOBALS: HashMap<>

#[derive(Debug)]
pub enum Elem {
    Float(f64),
    Int(i64),
    String(String),
    TwoTuple(Box<Elem>, Box<Elem>),
    Tuple(Vec<Elem>),
    Vec(Vec<Elem>),
    PyObj(PyObject),
    PyNone,
}

fn elem2pyobject(elem: &Elem, py: Python<'_>) -> PyObject {
    match elem {
        Elem::Float(x) => x.to_object(py),
        Elem::Int(x) => x.to_object(py),
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
        Elem::PyNone => py.None(),
    }
}

fn pyobject2elem(ob: &PyAny) -> PyResult<Elem> {
    let type_name = ob.get_type().name().unwrap();

    if type_name == "float" {
        Ok(Elem::Float(ob.extract::<f64>()?))
    } else if type_name == "int" {
        Ok(Elem::Int(ob.extract::<i64>()?))
    } else if type_name == "str" {
        Ok(Elem::String(ob.extract::<String>()?))
    } else if type_name == "tuple" {
        if let Ok((a, b)) = ob.extract::<(Elem, Elem)>() {
            Ok(Elem::TwoTuple(Box::new(a), Box::new(b)))
        } else {
            Ok(Elem::Tuple(ob.extract::<Vec<Elem>>()?))
        }
    } else if type_name == "list" {
        Ok(Elem::Vec(ob.extract::<Vec<Elem>>()?))
    } else if type_name == "NoneType" {
        Ok(Elem::PyNone)
    } else {
        Ok(Elem::PyObj(ob.to_object(ob.py())))
    }
}

impl Elem {
    pub fn to_pyobject(&self, py: Python<'_>) -> PyObject {
        elem2pyobject(self, py)
    }
}

impl IntoPy<PyObject> for Elem {
    fn into_py(self, py: Python<'_>) -> PyObject {
        elem2pyobject(&self, py)
    }
}

impl FromPyObject<'_> for Elem {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        pyobject2elem(ob)
    }
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Numbers: Same type
            (Elem::Float(a), Elem::Float(b)) => a == b,
            (Elem::Int(a), Elem::Int(b)) => a == b,
            // String
            (Elem::String(a), Elem::String(b)) => a == b,
            // Numbers: Mixed types
            (Elem::Float(a), Elem::Int(b)) => *a == *b as f64,
            (Elem::Int(a), Elem::Float(b)) => *a as f64 == *b as f64,
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
            // PyNone
            (Elem::PyNone, Elem::PyNone) => true,
            // All other cases
            _ => false,
        }
    }
}

impl Eq for Elem {}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // Numbers: Same type
            (Elem::Float(a), Elem::Float(b)) => a.partial_cmp(b),
            (Elem::Int(a), Elem::Int(b)) => a.partial_cmp(b),
            // Strings
            (Elem::String(a), Elem::String(b)) => a.partial_cmp(b),
            // Numbers: Mixed types
            (Elem::Int(a), Elem::Float(b)) => (*a as f64).partial_cmp(b),
            (Elem::Float(a), Elem::Int(b)) => a.partial_cmp(&(*b as f64)),
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
            // Numbers: Same type
            (Elem::Float(a), Elem::Float(b)) => a.partial_cmp(b).unwrap(),
            (Elem::Int(a), Elem::Int(b)) => a.partial_cmp(b).unwrap(),
            // Strings
            (Elem::String(a), Elem::String(b)) => a.cmp(b),
            // Numbers: Mixed types
            (Elem::Float(a), Elem::Int(b)) => a.partial_cmp(&(*b as f64)).unwrap(),
            (Elem::Int(a), Elem::Float(b)) => (*a as f64).partial_cmp(b).unwrap(),
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
    let a = a.downcast::<PyAny>(py)?;
    let b = b.downcast::<PyAny>(py)?;
    a.eq(b)
}

fn pyobject_partial_cmp(
    py: Python,
    a: &PyObject,
    b: &PyObject,
) -> PyResult<Option<std::cmp::Ordering>> {
    let a = a.downcast::<PyAny>(py)?;
    let b = b.downcast::<PyAny>(py)?;

    if a.lt(b)? {
        Ok(Some(std::cmp::Ordering::Less))
    } else if a.gt(b)? {
        Ok(Some(std::cmp::Ordering::Greater))
    } else if a.eq(b)? {
        Ok(Some(std::cmp::Ordering::Equal))
    } else {
        Ok(None)
    }
}

fn pyobject_cmp(py: Python, a: &PyObject, b: &PyObject) -> PyResult<std::cmp::Ordering> {
    let a = a.downcast::<PyAny>(py)?;
    let b = b.downcast::<PyAny>(py)?;

    if a.lt(b)? {
        Ok(std::cmp::Ordering::Less)
    } else if a.gt(b)? {
        Ok(std::cmp::Ordering::Greater)
    } else {
        Ok(std::cmp::Ordering::Equal)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn make_elem_from_python(code: &str) -> Elem {
        Python::with_gil(|py| {
            py.eval(code, None, None)
                .unwrap()
                .into_py(py)
                .extract::<Elem>(py)
                .unwrap()
        })
    }

    #[ctor::ctor]
    fn setup() {
        pyo3::prepare_freethreaded_python();
    }

    #[test]
    fn test_conversion() {
        // Float
        let elem = make_elem_from_python("1.0");
        assert_eq!(elem, Elem::Float(1.0));

        // Int
        let elem = make_elem_from_python("1");
        assert_eq!(elem, Elem::Int(1));

        // String
        let elem = make_elem_from_python("'hello'");
        assert_eq!(elem, Elem::String("hello".to_string()));

        // TwoTuple
        let elem = make_elem_from_python("(1, 2)");
        assert_eq!(
            elem,
            Elem::TwoTuple(Box::new(Elem::Int(1)), Box::new(Elem::Int(2)))
        );

        // Tuple
        let elem = make_elem_from_python("(1, 2, 3)");
        assert_eq!(
            elem,
            Elem::Tuple(vec![Elem::Int(1), Elem::Int(2), Elem::Int(3),])
        );

        // Vec
        let elem = make_elem_from_python("[1, 2, 3]");
        assert_eq!(
            elem,
            Elem::Vec(vec![Elem::Int(1), Elem::Int(2), Elem::Int(3),])
        );

        // PyObj
        let elem = make_elem_from_python("{'a': 1, 'b': 2}");
        match elem {
            Elem::PyObj(_) => (),
            _ => panic!("Expected PyObj"),
        };

        // PyNone
        let elem = make_elem_from_python("None");
        assert_eq!(elem, Elem::PyNone);
    }
}
