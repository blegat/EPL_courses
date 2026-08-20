# Multi-armed bandits and regret

## Topics and results

- A multi-armed bandit repeatedly chooses among unknown reward distributions;
  regret measures loss relative to always choosing the best mean-reward arm
  (`ONLINE`, pp. 4–8).
- Pure greedy selection can stop exploring after unlucky initial observations;
  optimistic initialization and $\varepsilon$-greedy improve exploration but
  fixed $\varepsilon$ incurs linear expected regret (`ONLINE`, pp. 9–16).
- Upper Confidence Bound adds an uncertainty bonus to each empirical mean. The
  stated analysis uses concentration to obtain logarithmic expected regret for
  suboptimal arms under its assumptions (`ONLINE`, pp. 17–23).
- Gradient bandits parameterize a softmax policy and apply stochastic gradient
  ascent; subtracting a reward baseline reduces variance without changing the
  expected gradient (`ONLINE`, pp. 24–29).
- Regret and explicit exploration distinguish online learning from ordinary
  offline estimation (`ONLINE`, pp. 30–31).

## Related courses

- Finite-sample tool: [LDACS1110 — concentration inequalities](../../DACS/1110/Concentration%20inequalities.md)
- Statistical estimation precursor: [LEPL1109 — parametric estimation](../../EPL/1109/Parametric%20estimation.md)
- Exploration foundation: [Exploration and stochastic-approximation ODEs](Exploration%20and%20stochastic-approximation%20ODEs.md)
- Introductory treatment: [LINMA2470 — bandits and Monte Carlo reinforcement learning](../../INMA/2470/Bandits%20and%20Monte%20Carlo%20reinforcement%20learning.md)
