# Policy iteration, value iteration, and linear programming

## Topics and results

- Policy evaluation solves the Bellman linear system or iterates its discounted
  contraction to a fixed point (`MDP`, pp. 30–38).
- The policy-improvement theorem proves that selecting greedy actions with
  respect to a policy's value cannot worsen that policy (`MDP`, pp. 40–42).
- Policy iteration alternates complete evaluation and greedy improvement until
  stable; in the finite discounted case a stable policy is optimal (`MDP`,
  pp. 44–46).
- Value iteration combines truncated evaluation and improvement by iterating the
  Bellman optimality operator; contraction proves convergence for
  $0\leq\gamma<1$ (`MDP`, pp. 48–54).
- Bellman inequalities yield primal/dual linear programs; dual variables have
  an occupation-measure interpretation (`MDP`, pp. 55–57).
- The finite-chain slides also treat average-reward relative values and policy
  improvement under an ergodic-unichain assumption (`FMC`, pp. 19–30).

## Related courses

- Foundation: [Markov decision processes and Bellman equations](Markov%20decision%20processes%20and%20Bellman%20equations.md)
- Advanced treatment: [LINMA2725 — stochastic optimal control and dynamic programming](../LINMA2725/Stochastic%20optimal%20control%20and%20dynamic%20programming.md)

