# Dynamic programming

## Topics and results

- Dynamic programming applies when subproblems overlap and optimal solutions
  have exploitable substructure. It stores each subproblem rather than
  recomputing its recursion tree (`NOTES`, pp. 32–36).
- Top-down memoization follows demanded recursive calls; bottom-up tabulation
  chooses an order in which dependencies are already solved.
- Rod cutting and matrix-chain multiplication illustrate state definition,
  recurrence, base cases and reconstruction of an optimizer (`S5`, pp. 2–7).
- Further formulations cover shortest paths, knapsack, longest common
  subsequences and optimal binary-search trees (`NOTES`, pp. 37–39).
- Computing an optimal value and reconstructing an optimal solution may require
  different stored information.
- Generating functions provide an algebraic alternative for recurrences such as
  Fibonacci and Catalan numbers (`NOTES`, pp. 39–45).
