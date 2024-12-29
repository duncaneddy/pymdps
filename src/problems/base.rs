use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::types::PyAny;

/// Define the BaseMDP trait with required methods
#[pyclass(subclass)]
pub struct BaseMDP;

#[pymethods]
impl BaseMDP {
    #[new]
    fn new() -> Self {
        BaseMDP
    }

    /// Abstract method to get states
    fn states(&self, _py: Python) -> PyResult<PyObject> {
        Err(exceptions::PyNotImplementedError::new_err(
            "Method 'states' not implemented",
        ))
    }

    /// Abstract method to get actions for a state
    fn actions(&self, _state: &str, _py: Python) -> PyResult<PyObject> {
        Err(exceptions::PyNotImplementedError::new_err(
            "Method 'actions' not implemented",
        ))
    }

    /// Abstract method to get transition probabilities
    fn transition(&self, _state: &str, _action: &str, _py: Python) -> PyResult<PyObject> {
        Err(exceptions::PyNotImplementedError::new_err(
            "Method 'transition_probabilities' not implemented",
        ))
    }

    /// Abstract method to get reward
    fn reward(&self, _state: &str, _action: &str, _next_state: &str, _py: Python) -> PyResult<f64> {
        Err(exceptions::PyNotImplementedError::new_err(
            "Method 'reward' not implemented",
        ))
    }
}