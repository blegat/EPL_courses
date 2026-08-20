# Markov decision processes and Bellman equations

## Topics and results

- A finite stationary MDP comprises states, actions, transition/reward law, and
  a decision policy; the Markov property makes the next reward/state depend only
  on the current state–action pair (`MDP`, pp. 4–11).
- Policies may be deterministic or randomized and map states to action choices
  (`MDP`, p. 11).
- State-value $v_\pi$ and action-value $q_\pi$ are expected discounted returns
  under a policy (`MDP`, pp. 24–25).
- Bellman's expectation equation decomposes a policy value into immediate reward
  plus discounted successor value (`MDP`, p. 25).
- Optimal values dominate all policy values and satisfy Bellman optimality
  equations with maximization over actions (`MDP`, pp. 26–27).

## Related courses

- Advanced formulation: [LINMA2725 — Bellman equations, value iteration, and policy iteration](../LINMA2725/Bellman%20equations,%20value%20iteration,%20and%20policy%20iteration.md)
- Algorithms: [Policy iteration, value iteration, and linear programming](Policy%20iteration,%20value%20iteration,%20and%20linear%20programming.md)
- Model-free extension: [Bandits and Monte Carlo reinforcement learning](Bandits%20and%20Monte%20Carlo%20reinforcement%20learning.md)
