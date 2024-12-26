# First Steps

This page will guide you through the first steps of using the `pymdps` package. We will first make sure that the package is installed correctly, and then we will create a simple Markov Decision Process (MDP) and solve it using the Value Iteration algorithm.

## Installation

To install the package, you can use pip:

```bash
pip install pymdps
```

This will install the latest version of the package from PyPI. If you want to install the latest development version from GitHub, you can use:

```bash
pip install git+https://github.com/duncaneddy/pymdps
```

## Creating an MDP

<!-- A Markov Decision Process (MDP) is defined by a tuple $(S, A, P, R, \gamma)$
where $S$ is the set of states, $A$ is the set of actions, $P(s' \mid s, a)$ is the transition probability function, $$R(s, a, s')$$ is the reward function, and $$\gamma$$ is the discount factor. -->

<!-- === "Python"

    ``` python
    --8<-- "../examples/gridworld.py"
    ``` -->