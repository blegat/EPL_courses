# Controllability and observability

## Topics and results

- Controllability asks whether an input can drive the state between arbitrary
  states in finite time (`CO`, pp. 7–13).
- For an $n$-state LTI system, the Kalman controllability matrix
  $$\mathcal C=[B\ AB\ \cdots\ A^{n-1}B]$$
  has full row rank exactly when the system is controllable (`CO`, pp. 14–24).
- Observability asks whether the initial state is uniquely determined from a
  finite input-output record. The matrix
  $$\mathcal O=\begin{bmatrix}C\\CA\\\vdots\\CA^{n-1}\end{bmatrix}$$
  has full column rank exactly when the system is observable (`CO`, pp. 27–38).
- Cayley–Hamilton limits both tests to the first $n$ powers of $A$.
- Eigenvector/geometric tests identify modes that inputs cannot excite or
  outputs cannot see (`CO`, pp. 39–44).
- Similarity changes preserve controllability and observability; a decomposition
  separates controllable/observable, hidden and unreachable subspaces.

## Related courses

- Control continuation: [LINMA2725 — linear quadratic Gaussian control](../../INMA/2725/Linear%20quadratic%20Gaussian%20control.md)
