use pyo3::prelude::*;

// Reexport all the submodules
pub use problems::*;
pub use solvers::*;

pub mod problems;
pub mod solvers;

// #[pyclass]
// pub struct MDPSolver {
//     mdp: Py<PyAny>,
// }

// #[pymethods]
// impl MDPSolver {
//     #[new]
//     fn new(mdp: Py<PyAny>) -> Self {
//         MDPSolver { mdp }
//     }

//     fn solve(&self, py: Python) -> PyResult<()> {
//         // Call a method on the MDP object (e.g., "states")
//         let _states = self.mdp.call_method0(py, "states")?;

//         Ok(())
//     }
// }

// Construct the Python module
#[pymodule]
pub fn _pymdps(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<BaseMDP>()?;
    // module.add_class::<MDPSolver>()?;
    Ok(())
}