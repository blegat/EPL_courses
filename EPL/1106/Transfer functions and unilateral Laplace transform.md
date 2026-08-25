# Transfer functions and unilateral Laplace transform

## Topics and results

- With zero initial conditions, an LTI transfer function is
  $H(s)=Y(s)/U(s)$ and equals the Laplace transform of the impulse response
  (`LAPLACE`, pp. 39–44).
- Differential equations become polynomial equations in $s$; state space gives
  $$H(s)=C(sI-A)^{-1}B+D.$$
- Poles encode natural modes, while zeros suppress input-output modes. Pole-zero
  cancellation may hide internal dynamics (`LAPLACE`, pp. 45–59).
- Block-diagram series, parallel and feedback connections translate into
  algebraic combinations of transfer functions.
- The unilateral transform starts at $0^-$ and incorporates initial conditions
  into derivative formulas, making it convenient for initial-value differential
  equations (`LAPLACE`, pp. 60–72).

## Internal connections

- [Differential equations, block diagrams, and state space](Differential%20equations%2C%20block%20diagrams%2C%20and%20state%20space.md)
- [Laplace transform and region of convergence](Laplace%20transform%20and%20region%20of%20convergence.md)
- [BIBO and internal stability](BIBO%20and%20internal%20stability.md)
