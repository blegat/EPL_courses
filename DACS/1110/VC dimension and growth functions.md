# VC dimension and growth functions

## Concepts

- Dichotomies, shattering, growth function, and VC dimension.
- Capacity is combinatorial and need not equal parameter count.
- Infinite classes can be learnable if their growth is controlled.

## Principal results

**Examples.** Thresholds on the real line have VC dimension 1; intervals have
VC dimension 2; affine halfspaces in $\\mathbb R^d$ have VC dimension $d+1$.

**Sauer-Shelah lemma.** If $\\operatorname{VCdim}(\\mathcal H)=d<m$,

$$
\Pi_{\mathcal H}(m)
\leq\sum_{i=0}^d\binom mi
\leq\left(\frac{em}{d}\right)^d.
$$

**VC generalization rate.** A representative uniform bound has order

$$
\sup_{h\in\mathcal H}|R(h)-\widehat R(h)|
=O\left(
\sqrt{\frac{d\log(m/d)+\log(1/\delta)}{m}}
\right).
$$

**Fundamental theorem, statement.** Under standard measurability conditions for
binary classification, finite VC dimension characterizes distribution-free PAC
learnability and uniform convergence, up to the distinctions made in the exact
version of the theorem.

## Prerequisites from LEPL1109

- Linear decision boundaries and k-NN flexibility:
  [linear least squares and k-nearest neighbors](../../EPL/1109/Linear%20least%20squares%20and%20k-nearest%20neighbors.md)
  [SL-1, pp. 22-35].
- Model classes and overfitting:
  [supervised-learning formulation](../../EPL/1109/Supervised-learning%20formulation.md).

## Developments beyond LEPL1109

- Shattering, growth functions, and capacity of infinite classes.
- Capacity-dependent sample complexity.

## Connection to foundations of cryptography

Moderate methodological value through counting and uniform adversarial choices;
little direct dependency for the listed primitives.

## References

[B8, Chs. 5-7](README.md#b8), [B7](README.md#b7), [B15](README.md#b15).
