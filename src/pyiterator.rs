use std::collections::{btree_map, BTreeMap};

use crate::pybtree_map::PyBTreeMap;
use crate::pyobject_wrapper::PyObjectWrapper;
use ouroboros::self_referencing;
use pyo3::prelude::*;

#[pyclass]
#[self_referencing]
struct PyBTreeKeyIterator {
    owner: PyObject,
    #[borrows(owner)]
    data: &'this PyRef<'static, PyBTreeMap>,
    #[borrows(data)]
    iter: &'this btree_map::Keys<'static, PyObjectWrapper, PyObjectWrapper>,
}

unsafe impl Send for PyBTreeKeyIterator {}

impl PyBTreeKeyIterator {
    fn new2(py: Python, owner: PyObject) -> Self {
        let x = PyBTreeKeyIterator::new(
            owner,
            |owner| {
                let owner: &PyCell<PyBTreeMap> = owner.downcast(py).unwrap();
                let x = owner.borrow();
                // transmute x to have a static lifetime
                unsafe {
                    std::mem::transmute::<&PyRef<'_, PyBTreeMap>, &'static &PyRef<'_, PyBTreeMap>>(
                        &x,
                    )
                }
            },
            |data| {
                let data = data.tree.keys();
            },
        );
        return x;
    }
}

// #[pymethods]
// impl PyBTreeKeyIterator {
//     fn next(&mut self) -> PyResult<PyObject> {
//         match self.iter.next() {
//             Some(x) => Ok(x.obj),
//             None => Err(PyErr::new::<PyStopIteration, _>(())),
//         }
//     }
// }
