# Bellman equations, value iteration, and policy iteration

## Topics and results

- The value function assigns to each initial state the minimum accumulated cost;
  the Q-function exposes the first action. Bellman's principle gives the
  fixed-point optimality equation (`CH1`, pp. 3–5).
- Value iteration repeatedly applies the Bellman minimization operator and
  obtains a greedy policy at each iteration (`CH3A`, pp. 2–3).
- Policy iteration alternates exact or approximate policy evaluation with
  greedy policy improvement (`CH3A`, pp. 4–5).
- In general state spaces both algorithms are conceptual until one chooses a
  representable function class, a tractable optimization procedure, and either
  a model or sampled trajectories (`CH3A`, pp. 3–6).

## Related courses

- Statistical decision precursor: [LEPL1109 — statistical decision theory and Bayes optimality](../../EPL/1109/Statistical%20decision%20theory%20and%20Bayes%20optimality.md)
- Approximate evaluation: [Temporal-difference learning and projected Bellman equations](Temporal-difference%20learning%20and%20projected%20Bellman%20equations.md)
- Stochastic extension: [Stochastic optimal control and dynamic programming](Stochastic%20optimal%20control%20and%20dynamic%20programming.md)
- Introductory MDP treatment: [LINMA2470 — Markov decision processes and Bellman equations](../../INMA/2470/Markov%20decision%20processes%20and%20Bellman%20equations.md)
