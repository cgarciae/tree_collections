use std::collections::btree_map;

use crate::pybtree_map::PyBTreeMap;
use crate::pyobject_wrapper::PyObjectWrapper;
use ouroboros::self_referencing;
use pyo3::exceptions::PyStopIteration;
use pyo3::prelude::*;

#[pyclass]
#[self_referencing]
struct PyBTreeKeyIterator {
    owner: PyObject,
    #[borrows(owner)]
    data: &'this PyRef<'static, PyBTreeMap>,
    #[borrows(data)]
    iter: btree_map::Keys<'static, PyObjectWrapper, PyObjectWrapper>,
}

unsafe impl Send for PyBTreeKeyIterator {}

impl PyBTreeKeyIterator {
    fn create(py: Python, owner: PyObject) -> Self {
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
                // transmute data to have a static lifetime
                let x = unsafe {
                    std::mem::transmute::<
                        btree_map::Keys<'_, PyObjectWrapper, PyObjectWrapper>,
                        btree_map::Keys<'static, PyObjectWrapper, PyObjectWrapper>,
                    >(data)
                };
                x
            },
        );
        return x;
    }

    pub fn next(&mut self) -> PyResult<PyObject> {
        self.with_iter_mut(|x| match x.next() {
            Some(x) => Ok(x.obj.clone()),
            None => Err(PyErr::new::<PyStopIteration, _>(())),
        })
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
