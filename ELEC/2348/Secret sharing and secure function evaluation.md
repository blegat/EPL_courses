# Secret sharing and secure function evaluation

## Topics and results

- A threshold secret-sharing scheme lets any qualified set reconstruct a
  secret while every smaller forbidden set learns nothing (`CRYPTO`, pp. 36–39).
- Shamir sharing chooses a random degree-$t$ polynomial with secret constant
  term and distributes evaluations; $t+1$ shares reconstruct by Lagrange
  interpolation (`CRYPTO`, pp. 39–42).
- Shares add locally. Multiplication raises polynomial degree, so secure
  multiparty computation needs degree reduction and enough honest parties
  (`CRYPTO`, pp. 43–48).
- Secure function evaluation aims to reveal prescribed outputs and nothing
  beyond them; an ideal functionality provides the reference behavior
  (`CRYPTO`, pp. 32–35; `MPC`, Chs. 1–2).
- Passive and active adversaries require different protocols and corruption
  thresholds. Simulation compares a real protocol with the ideal resource.
- Arithmetic circuits reduce general computation to secure additions and
  multiplications, yielding composition from secure building blocks
  (`CRYPTO`, pp. 43–49; `MPC`, Ch. 3).
