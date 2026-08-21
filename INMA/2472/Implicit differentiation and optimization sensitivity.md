# Implicit differentiation and optimization sensitivity

## Topics and results

- Iterating through every step of a numerical solver is often unnecessary and
  can be expensive or unstable. At a converged solution, an equation defining
  the solution can be differentiated instead (`IMPLICIT`, “Differentiating
  numerical procedures”).
- The implicit-function theorem turns $F(x,p)=0$ into the sensitivity equation
  $F_x\,dx/dp=-F_p$ when the state Jacobian is nonsingular.
- JVPs solve a linear system for each parameter direction; VJPs solve an adjoint
  transposed system and are preferable for scalar downstream losses
  (`IMPLICIT`, “Implicit VJP and JVP”).
- Factorizing the state Jacobian once amortizes repeated tangent or adjoint
  solves with different right-hand sides.
- Applying the same idea to KKT conditions differentiates an optimization layer.
  For a linear program, active-set regularity and nonsingularity determine when
  local primal and dual sensitivities are well defined (`IMPLICIT`,
  “Sensitivity of a linear program”).

## Connections

- AD primitives: [Forward and reverse automatic differentiation](Forward%20and%20reverse%20automatic%20differentiation.md)
- Sparse systems: [Higher-order and sparse automatic differentiation](Higher-order%20and%20sparse%20automatic%20differentiation.md)
