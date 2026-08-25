# State-space models and Kalman filtering

## Topics and results

- A state-space model separates state dynamics from noisy measurements:
  $s_n=F_ns_{n-1}+u_n$ and $x_n=H_ns_n+w_n$ in the linear case
  (`SP`, pp. 125–130).
- Dynamic estimation recursively combines a prediction from the process model
  with an update from the latest measurement.
- The scalar Kalman filter propagates prior mean and variance, forms the
  innovation, and weights it by the Kalman gain. The posterior variance is no
  larger than the prediction variance (`SP`, pp. 131–141).
- The gain balances process/measurement uncertainty: accurate measurements
  receive greater weight, while noisy measurements leave the prediction nearly
  unchanged.
- Matrix covariance prediction and update give the vector Kalman filter. Under
  linear-Gaussian assumptions it computes the exact conditional mean and
  covariance and hence the recursive MMSE estimator (`SP`, p. 142).

## Connections

- Signals-and-systems prerequisite: [LEPL1106 — differential equations, block diagrams, and state space](../../EPL/1106/Differential%20equations,%20block%20diagrams,%20and%20state%20space.md)
- Static Gaussian estimator: [Bayesian linear models, MAP, and linear MMSE](Bayesian%20linear%20models,%20MAP,%20and%20linear%20MMSE.md)
- Nonlinear/non-Gaussian generalization: [Recursive Bayesian filtering](Recursive%20Bayesian%20filtering.md)
- Tracking application: [LELEC2885 — recursive appearance-based tracking](../../ELEC/2885/Recursive%20appearance-based%20tracking.md)
- Control counterpart: [LINMA2725 — linear quadratic Gaussian control](../2725/Linear%20quadratic%20Gaussian%20control.md)
