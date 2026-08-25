# BIBO and internal stability

## Topics and results

- BIBO stability means every bounded input gives a bounded output. An LTI
  system is BIBO stable exactly when its impulse response is absolutely
  integrable or summable (`STAB`, pp. 7–17).
- For a causal rational transfer function, BIBO stability requires all poles in
  the open left half-plane in continuous time or inside the unit disk in
  discrete time (`STAB`, pp. 18–22).
- Internal stability concerns the zero-input state dynamics. Continuous-time
  $\dot x=Ax$ is asymptotically stable iff every eigenvalue of $A$ has negative
  real part; discrete-time $x_{k+1}=Ax_k$ is asymptotically stable iff
  $\rho(A)<1$ (`STAB`, pp. 24–34).
- Lyapunov stability and attractivity are distinct for nonlinear systems,
  though strict spectral stability of an LTI system gives both.
- Internal asymptotic stability implies BIBO stability for a finite-dimensional
  realization, but pole-zero cancellations can make an externally stable
  transfer function hide unstable internal modes (`STAB`, pp. 35–43).
