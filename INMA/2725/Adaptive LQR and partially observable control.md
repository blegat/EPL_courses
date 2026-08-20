# Adaptive LQR and partially observable control

## Topics and results

- Adaptive LQR controls an unknown linear dynamical system while estimating its
  matrices, so exploration, identification error, stability, and control regret
  interact (`ONLINE`, pp. 32–34).
- Robust model-based control chooses a controller valid for a confidence set of
  identified models; the slides state an $O(T^{2/3})$ regret scale for the
  presented method (`ONLINE`, pp. 35–37).
- Certainty-equivalence control uses the current least-squares model estimate;
  the presented analysis reaches an $O(T^{1/2})$ scale under its assumptions
  (`ONLINE`, pp. 38–42).
- A POMDP has hidden Markov state and observations. The posterior distribution
  of the current state—the belief state—is a sufficient information state for
  optimal control (`ONLINE`, pp. 43–50).
- Bayesian filtering gives deterministic controlled belief-state dynamics, so
  the POMDP becomes a fully observed control problem over distributions
  (`ONLINE`, pp. 45–51).
- In LQG the belief is summarized by its Gaussian mean and covariance, and the
  separation principle combines Kalman estimation with LQR control (`ONLINE`,
  pp. 52–53).

## Related courses

- Foundation: [Linear quadratic Gaussian control](Linear%20quadratic%20Gaussian%20control.md)
- Estimation background: [LEPL1109 — parametric estimation](../../EPL/1109/Parametric%20estimation.md)
