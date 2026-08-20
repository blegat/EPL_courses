# 8. Hypothesis testing

### General framework

- Null/alternative hypotheses, statistical decision rule, Type I and Type II
  errors, significance level, test statistic, rejection region, and observed
  statistic [STAT, pp. 138-141].
- General method: choose $\alpha$, construct a null rejection region of
  probability $\alpha$, compute the statistic, and reject iff it lies in the
  region [STAT, p. 141].
- p-value as the smallest significance level producing rejection; lower-tail,
  upper-tail, and symmetric two-sided formulas; reject iff $p<\alpha$
  [STAT, pp. 151-156]. A large p-value means insufficient evidence against the
  null, not that the null has high posterior probability.

### Tests covered

- One-sample normal mean with unknown variance: one- and two-sided Student
  tests [STAT, pp. 142-146].
- One-sample normal variance: one- and two-sided chi-square tests [STAT,
  pp. 147-150].
- Difference of two independent normal means with known common variance or
  equal unknown variance: normal or pooled Student tests [STAT, pp. 158-161].
- Equality/order of two independent normal variances: Fisher test; Bartlett's
  test is mentioned for two-sided equality [STAT, pp. 162-165].
- Students calculate tests manually and through SciPy routines [STAT,
  pp. 146, 150, 161, 164-165].

## Related courses

- Linear-model application: [LSTAT2120 — inference, confidence regions, and hypothesis tests](../../STAT/2120/Inference,%20confidence%20regions,%20and%20hypothesis%20tests.md)
