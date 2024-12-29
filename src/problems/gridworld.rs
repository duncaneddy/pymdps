use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::exceptions;
use std::collections::HashMap;

/// Gridworld MDP class that inherits from BaseMDP
#[pyclass(extends=BaseMDP)]
pub struct Gridworld {
    grid_size_x: usize,
    grid_size_y: usize,
    cells_with_rewards: HashMap<(usize, usize), f64>,
}

#[pymethods]
impl Gridworld {
    /// Constructor for Gridworld
    #[new]
    fn new(
        grid_size_x: usize,
        grid_size_y: usize,
        cells_with_rewards: Vec<((usize, usize), f64)>,
    ) -> PyResult<(BaseMDP, Self)> {
        let mut rewards_map = HashMap::new();
        for ((x, y), reward) in cells_with_rewards {
            rewards_map.insert((x, y), reward);
        }
        Ok((
            BaseMDP {}, // Initialize the base class
            Gridworld {
                grid_size_x,
                grid_size_y,
                cells_with_rewards: rewards_map,
            },
        ))
    }

    /// Returns all states in the grid
    fn states(&self, _py: Python) -> PyResult<PyObject> {
        let states: Vec<(usize, usize)> = (0..self.grid_size_x)
            .flat_map(|x| (0..self.grid_size_y).map(move |y| (x, y)))
            .collect();
        Ok(states.to_object(_py))
    }

    /// Returns available actions for a given state
    fn actions(&self, state: (usize, usize), _py: Python) -> PyResult<PyObject> {
        let (x, y) = state;
        let mut actions = Vec::new();

        if x > 0 {
            actions.push("left");
        }
        if x < self.grid_size_x - 1 {
            actions.push("right");
        }
        if y > 0 {
            actions.push("up");
        }
        if y < self.grid_size_y - 1 {
            actions.push("down");
        }

        Ok(actions.to_object(_py))
    }

    /// Returns transition probabilities for a given state and action
    fn transition(&self, state: (usize, usize), action: &str, _py: Python) -> PyResult<PyObject> {
        let (x, y) = state;
        let (new_x, new_y) = match action {
            "left" => {
                if x > 0 {
                    (x - 1, y)
                } else {
                    return Err(exceptions::PyValueError::new_err("Invalid action: Cannot move left"));
                }
            }
            "right" => {
                if x < self.grid_size_x - 1 {
                    (x + 1, y)
                } else {
                    return Err(exceptions::PyValueError::new_err("Invalid action: Cannot move right"));
                }
            }
            "up" => {
                if y > 0 {
                    (x, y - 1)
                } else {
                    return Err(exceptions::PyValueError::new_err("Invalid action: Cannot move up"));
                }
            }
            "down" => {
                if y < self.grid_size_y - 1 {
                    (x, y + 1)
                } else {
                    return Err(exceptions::PyValueError::new_err("Invalid action: Cannot move down"));
                }
            }
            _ => return Err(exceptions::PyValueError::new_err("Invalid action")),
        };

        let next_state = (new_x, new_y);
        let probability = 1.0; // Deterministic transition

        let transitions = vec![(next_state, probability)];
        Ok(transitions.to_object(_py))
    }

    /// Returns the reward for a given state, action, and next state
    fn reward(
        &self,
        _state: (usize, usize),
        _action: &str,
        next_state: (usize, usize),
        _py: Python,
    ) -> PyResult<f64> {
        match self.cells_with_rewards.get(&next_state) {
            Some(&reward) => Ok(reward),
            None => Ok(0.0), // Default reward if not specified
        }
    }
}