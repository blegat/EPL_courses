# Sample compression and description length

## Concepts

- Compress a labeled sample to a small subset plus finite side information.
- Reconstruct a hypothesis from the compression.
- Generalization by counting the possible compressed descriptions.
- Difference between sample compression and PCA/data compression.

## Principal results

**Compression generalization.** A consistent hypothesis reconstructed from `k`
sample points and bounded side information admits a realizable bound of the
representative form

$$
R(h)=O\left(\frac{k\log m+\log(1/\delta)}{m}\right),
$$

with exact constants and logarithmic terms depending on the compression-scheme
definition.

**Threshold example.** A consistent threshold can be reconstructed from at most
two extremal labeled examples.

**Description-length principle.** Shorter hypothesis descriptions reduce the
number of alternatives that must be controlled by the union bound.

## Prerequisites from LEPL1109

- Dataset, generalization, and empirical risk:
  [supervised-learning formulation](../../EPL/1109/Supervised-learning%20formulation.md)
  and [statistical decision theory](../../EPL/1109/Statistical%20decision%20theory%20and%20Bayes%20optimality.md).
- PCA compression is a distinct concept:
  [PCA](../../EPL/1109/Unsupervised%20learning.md#principal-component-analysis).

## Developments beyond LEPL1109

- Generalization from reconstructibility and short combinatorial descriptions.

## Connection to foundations of cryptography

Moderate proof-pattern value through counting and encodings.

## References

[B15](README.md#b15), [B8](README.md#b8).
