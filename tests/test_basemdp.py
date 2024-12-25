from pymdps import BaseMDP

def test_basemdp():
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
    
    mdp = MyMDP()
    
    assert mdp.states() == ['s1', 's2', 's3']
    assert mdp.actions('s1') == ['a1', 'a2']
    assert mdp.actions('s2') == ['a3', 'a4']
    assert mdp.transition_probabilities('s1', 'a1') == {'s1': 0.5, 's2': 0.5}
    assert mdp.transition_probabilities('s2', 'a3') == {'s3': 1.0}
    assert mdp.reward('s1', 'a1', 's1') == 1.0
    assert mdp.reward('s2', 'a3', 's3') == 1.0