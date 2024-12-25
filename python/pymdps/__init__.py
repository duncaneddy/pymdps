"""
PyMDPS is a Python library for solving Markov Decision Processes (MDPs) and Partially Observable MDPs (POMDPs).
"""

# Import the classes from the _pymdps Rust module
from ._pymdps import (
    BaseMDP,
    MDPSolver,
)