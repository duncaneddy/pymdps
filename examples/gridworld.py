from pymdps import BaseMDP, MDPSolver

class MyMDP(BaseMDP):
    def __init__(self):
        super().__init__()

    def states(self):
        return ['s1', 's2', 's3']

    def actions(self, state):
        return ['a1', 'a2'] if state == 's1' else ['a3', 'a4']

    def transition_probabilities(self, state, action):
        return {'s1': 0.5, 's2': 0.5} if action == 'a1' else {'s3': 1.0}

    def reward(self, state, action, next_state):
        return 1.0
    
if __name__ == '__main__':
    mdp = MyMDP()
    
    solver = MDPSolver(mdp)

    solver.solve()

    print('Done')