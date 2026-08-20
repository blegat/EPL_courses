# Temporal-difference, SARSA, and Q-learning

## Topics and results

- TD prediction bootstraps from the next state's current estimate and updates
  before an episode ends, combining Monte Carlo sampling with dynamic-programming
  targets (`RL`, pp. 31–35).
- The TD error is immediate reward plus discounted next estimate minus current
  estimate (`RL`, pp. 33–35).
- On-policy SARSA updates an action value using the next action actually selected
  by the behavior policy (`RL`, pp. 37–39).
- Off-policy Q-learning targets the greedy next-action value while behavior may
  remain exploratory (`RL`, pp. 37, 40–41).
- Windy GridWorld illustrates that SARSA and Q-learning can learn different
  paths because their targets reflect different policies (`RL`, pp. 43–45).

## Related courses

- Monte Carlo precursor: [Bandits and Monte Carlo reinforcement learning](Bandits%20and%20Monte%20Carlo%20reinforcement%20learning.md)
- Advanced convergence treatment: [LINMA2725 — stochastic TD and off-policy Q-learning](../LINMA2725/Stochastic%20TD%20and%20off-policy%20Q-learning.md)

