# 3. Concentration inequalities

**Status:** Core, Bridge.

### Concepts

- Tail probability versus variance and asymptotic approximation.
- Polynomial versus exponential tails.
- Boundedness, independence, and variance assumptions.
- High-probability statement: an event holds with probability at least
  `1 - delta`.
- Solve a tail bound for the sample size needed to achieve error `epsilon` and
  confidence `1 - delta`.
- Chernoff's exponential-moment method as a reusable proof technique.

### Candidate results

**Markov inequality.** If `X >= 0` and `a > 0`,

\[
P(X\geq a)\leq \frac{E[X]}{a}.
\]

**Chebyshev inequality.** If `X` has finite variance,

\[
P(|X-E[X]|\geq t)\leq\frac{\operatorname{Var}(X)}{t^2}.
\]

**Chernoff method.** For `lambda > 0`,

\[
P(S\geq t)
\leq \inf_{\lambda>0}e^{-\lambda t}E[e^{\lambda S}].
\]

**Hoeffding inequality.** For independent `X_i in [a_i,b_i]`,

\[
P\left(\sum_i(X_i-E[X_i])\geq t\right)
\leq
\exp\left(-\frac{2t^2}{\sum_i(b_i-a_i)^2}\right).
\]

For independent Bernoulli observations,

\[
P(|\widehat p-p|\geq\varepsilon)
\leq 2e^{-2n\varepsilon^2}.
\]

Hence it suffices that

\[
n\geq\frac{\log(2/\delta)}{2\varepsilon^2}
\]

to obtain error at most `epsilon` with probability at least `1-delta`.

**Multiplicative Chernoff bounds, recommended extension.** For a sum of
independent Bernoulli variables with mean `mu`, representative forms are

\[
P(S\geq(1+\eta)\mu)
\leq \exp\left(-\frac{\eta^2\mu}{2+\eta}\right),
\qquad
P(S\leq(1-\eta)\mu)
\leq e^{-\eta^2\mu/2}.
\]

**Bernstein inequality, optional.** Add a variance-sensitive exponential bound
for bounded independent variables.

**McDiarmid inequality, optional.** A function with bounded coordinate changes
concentrates around its expectation; this extends concentration from sums to
stable functions of independent data.

### LEPL1109 dependency

- Expectation, variance, independence, and MGFs:
  [probability foundations](../LEPL1109/Probability%20foundations%20and%20random%20variables.md)
  [STAT, pp. 13-18; FORM, p. 1].
- Bernoulli and binomial laws:
  [assumed distribution family](../LEPL1109/Probability%20foundations%20and%20random%20variables.md#assumed-distribution-family)
  [APP, pp. 5-9].
- CLT, to contrast asymptotic and nonasymptotic statements:
  [normal approximations](../LEPL1109/Normal%20approximations%20and%20reference%20laws.md)
  [STAT, pp. 50-52].

### What is new beyond LEPL1109

- Finite-sample exponential guarantees.
- Explicit `epsilon`, `delta`, and sample-complexity dependence.
- Uniform control through a union bound.
- MGF-based proof techniques.

### FoC reuse

- Security amplification and negligible bad-event probabilities.
- Collision/occupancy analysis and random constructions.
- Bounding adversarial success over many keys, messages, or queries.

### Bibliography

[B2](content.md#b2), [B12, Chs. 2-3](content.md#b12), [B7, Parts I-II](content.md#b7).
