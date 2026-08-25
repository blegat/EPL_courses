# Differential equations, block diagrams, and state space

## Topics and results

- Constant-coefficient differential or difference equations express an LTI
  input-output relation. Their solutions split into free and forced responses
  (`MODELS`, pp. 8–21).
- The characteristic roots determine natural modes; repeated or complex roots
  generate polynomial factors or damped sinusoids.
- Block diagrams compose adders, gains, integrators or delays and expose
  feedforward and feedback structure (`MODELS`, pp. 22–33).
- A state-space model has
  $$\dot x=Ax+Bu,\qquad y=Cx+Du$$
  in continuous time, or $x_{k+1}=Ax_k+Bu_k$ in discrete time
  (`MODELS`, pp. 34–44).
- The zero-input solution is $e^{At}x(0)$ in continuous time and $A^kx_0$ in
  discrete time; variation of constants adds the forced response.
- State coordinates are not unique: an invertible similarity transformation
  changes $(A,B,C)$ but not the external input-output behavior.
- Differential equations, block diagrams, impulse responses and state-space
  models provide complementary representations (`MODELS`, pp. 45–58).

## Related courses

- Estimation continuation: [LINMA1731 — state-space models and Kalman filtering](../../INMA/1731/State-space%20models%20and%20Kalman%20filtering.md)
