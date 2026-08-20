# Bandits and Monte Carlo reinforcement learning

## Topics and results

- In a $k$-armed bandit, action values are unknown mean rewards; greedy and
  $\varepsilon$-greedy choices express the exploitation–exploration tradeoff
  (`RL`, pp. 5–11).
- Sample-average action values admit a constant-memory incremental update of the
  form old estimate plus step size times prediction error (`RL`, pp. 12–14).
- Reinforcement learning evaluates or improves policies without an explicit
  transition/reward model and distinguishes episodic from continuing tasks
  (`RL`, pp. 16–18).
- First-visit Monte Carlo prediction averages complete returns following first
  visits to a state (`RL`, pp. 19–24).
- Model-free control estimates action values, alternates evaluation and greedy
  improvement, and uses exploring starts or $\varepsilon$-soft policies to keep
  sampling all actions (`RL`, pp. 25–29).

## Related courses

- MDP foundation: [Markov decision processes and Bellman equations](Markov%20decision%20processes%20and%20Bellman%20equations.md)
- Regret-oriented extension: [LINMA2725 — multi-armed bandits and regret](../../INMA/2725/Multi-armed%20bandits%20and%20regret.md)
- Monte Carlo background: [LDACS1110 — Monte Carlo estimation](../../DACS/1110/Monte%20Carlo%20estimation.md)
- Bootstrapped continuation: [Temporal-difference, SARSA, and Q-learning](Temporal-difference,%20SARSA,%20and%20Q-learning.md)
