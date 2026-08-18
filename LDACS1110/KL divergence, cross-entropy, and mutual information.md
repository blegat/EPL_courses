# 7. KL divergence, cross-entropy, and mutual information

**Status:** Core, Bridge.

### Concepts

- Divergence between distributions rather than distance between parameters.
- Asymmetry and possible infinity of KL divergence.
- Cross-entropy as expected negative log-likelihood.
- Mutual information as distance from independence and expected information
  gain.
- Markov chains in the information-theoretic sense `X -> Y -> Z`.

### Candidate results

**KL divergence.**

$$
D_{\mathrm{KL}}(P\|Q)
=\sum_x P(x)\log\frac{P(x)}{Q(x)}.
$$

**Gibbs inequality.** `D_KL(P||Q) >= 0`, with equality exactly when the
distributions agree on the relevant support.

**Cross-entropy decomposition.**

$$
H(P,Q)=H(P)+D_{\mathrm{KL}}(P\|Q).
$$

**Mutual information identities.**

$$
I(X;Y)
=D_{\mathrm{KL}}(P_{XY}\|P_XP_Y)
=H(X)-H(X\mid Y)
=H(Y)-H(Y\mid X).
$$

Consequently, `I(X;Y) >= 0`, and `I(X;Y)=0` exactly when `X` and `Y` are
independent.

**Chain rule for mutual information.**

$$
I(X;Y,Z)=I(X;Y)+I(X;Z\mid Y).
$$

**Data-processing inequality.** If `X -> Y -> Z`, then

$$
I(X;Z)\leq I(X;Y).
$$

**Pinsker inequality, recommended bridge.** Under natural logarithms,

$$
\Delta(P,Q)\leq\sqrt{\frac12D_{\mathrm{KL}}(P\|Q)}.
$$

This connects information divergence to statistical distinguishing advantage.

### LEPL1109 dependency

- Joint distributions, independence, conditioning, and likelihood:
  [dependence and multivariate probability](../LEPL1109/Dependence%20and%20multivariate%20probability.md)
  and [maximum likelihood](../LEPL1109/Parametric%20estimation.md#maximum-likelihood)
  [STAT, pp. 38, 60-81, 97-105].
- Logistic negative log-likelihood/cross-entropy:
  [logistic regression](../LEPL1109/Logistic%20regression%20and%20classification%20assessment.md#logistic-regression)
  [SL-2, pp. 33-35].

### What is new beyond LEPL1109

- Information divergence and mutual information.
- Data processing and chain rules.
- Connection between log-loss, likelihood, and distribution approximation.

### FoC reuse

- Perfect secrecy as zero mutual information.
- Information leakage and processing of adversarial observations.
- Pinsker as a route from information bounds to indistinguishability.

### Bibliography

[B5, Chs. 2 and 11](content.md#b5), [B6](content.md#b6), [B7](content.md#b7).

### Related courses

- Applied use in variable selection: [LELEC2870 — feature selection](../LELEC2870/Feature%20selection.md)
- Applied use as an independence objective: [LELEC2870 — independent component analysis](../LELEC2870/Independent%20component%20analysis.md)
