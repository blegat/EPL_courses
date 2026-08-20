# Model predictive control

## Topics and results

- Linear MPC repeatedly solves a finite-horizon quadratic optimization problem,
  applies only its first input, observes the new state, and shifts the horizon
  (`CH2`, pp. 3–6).
- State and output trajectories can be eliminated to obtain a quadratic program
  in the input sequence subject to input/output constraints (`CH2`, pp. 7–11).
- Tracking MPC introduces a reference trajectory and commonly parametrizes
  input increments, enabling reference and rate penalties (`CH2`, pp. 12–16).
- Horizon length, terminal cost/constraints, sampling frequency, feasibility,
  computation time, and model mismatch are practical design considerations
  (`CH2`, pp. 16–17).
- Continuous-time and nonlinear MPC use discretization, linearization along a
  trajectory, or direct nonlinear optimization (`CH2`, pp. 17–18).

## Related courses

- Quadratic-control foundation: [Linear quadratic regulation and Riccati equations](Linear%20quadratic%20regulation%20and%20Riccati%20equations.md)

