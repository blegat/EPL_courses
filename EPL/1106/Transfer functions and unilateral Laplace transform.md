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
