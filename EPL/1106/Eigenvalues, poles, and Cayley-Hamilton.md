# Eigenvalues, poles, and Cayley-Hamilton

## Topics and results

- The characteristic polynomial $p_A(\lambda)=\det(\lambda I-A)$ has the
  eigenvalues of $A$ as roots.
- Cayley–Hamilton states $p_A(A)=0$ (`STAB`, pp. 47–53).
- Consequently, every sufficiently high power of $A$ is a linear combination
  of $I,A,\ldots,A^{n-1}$; matrix functions such as $e^{At}$ reduce to finitely
  many matrix powers.
- The resolvent $(sI-A)^{-1}$ has denominator dividing the characteristic
  polynomial, linking state eigenvalues to transfer-function poles.
- Not every eigenvalue must appear as an input-output pole: uncontrollable or
  unobservable modes can cancel from $C(sI-A)^{-1}B+D$ (`STAB`, pp. 54–58).
- Minimal realizations eliminate such hidden modes and align internal and
  external modal descriptions.

## Internal connections

- [Differential equations, block diagrams, and state space](Differential%20equations%2C%20block%20diagrams%2C%20and%20state%20space.md)
- [BIBO and internal stability](BIBO%20and%20internal%20stability.md)
