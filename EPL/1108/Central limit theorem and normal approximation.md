# Central limit theorem and normal approximation

## Topics and results

- For i.i.d. variables with mean $\mu$ and finite nonzero variance $\sigma^2$,
  $$\frac{\sum_{i=1}^nX_i-n\mu}{\sigma\sqrt n}
  \mathrel{\Rightarrow}N(0,1).$$
- This concerns standardized fluctuations, whereas the law of large numbers
  concerns the sample average (`P4`, pp. 26–34).
- It yields normal approximations for sums, including binomial counts, with a
  continuity correction for lattice variables.
- Berry–Esseen bounds the uniform approximation error by a constant times
  $E|X-\mu|^3/(\sigma^3\sqrt n)$ (`PROBA`, pp. 67–68; `P4`, pp. 35–40).

## Related courses

- Statistical use: [LEPL1109 — normal approximations and reference laws](../1109/Normal%20approximations%20and%20reference%20laws.md)

## Internal connections

- [Exponential, Poisson, and normal laws](Exponential%2C%20Poisson%2C%20and%20normal%20laws.md)
- [Moments, covariance, and concentration](Moments%2C%20covariance%2C%20and%20concentration.md)
