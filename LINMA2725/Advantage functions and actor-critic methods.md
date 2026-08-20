# Advantage functions and actor-critic methods

## Topics and results

- The advantage $A(x,u)=Q(x,u)-h(x)$ preserves action rankings while removing a
  state-dependent baseline, reducing the magnitude and potentially the variance
  of estimates (`S10`, pp. 4–10).
- Regenerative constructions normalize relative values and permit TD learning
  for average cost (`S10`, pp. 11–19).
- A critic estimates Q or advantage for the current parameterized randomized
  policy; an actor follows a score-function policy gradient (`S10`, pp. 20–25).
- The sensitivity theorem connects the average-cost gradient to the score and
  Q-function (`S10`, pp. 21–24).
- Compatible features make critic projection error orthogonal to the policy
  score, preserving the desired actor gradient under realistic approximation
  (`S10`, pp. 26–31).
- Advantage baselines reduce variance without biasing the ideal gradient, but
  finite-$\lambda$ and actor–critic coupling retain important variance and
  convergence caveats (`S10`, pp. 32–35).

## Related courses

- Sensitivity foundation: [Fluid models, policy sensitivity, and score functions](Fluid%20models,%20policy%20sensitivity,%20and%20score%20functions.md)
- Optimization background: [LELEC2870 — model selection, validation, and regularization](../LELEC2870/Model%20selection,%20validation,%20and%20regularization.md)

