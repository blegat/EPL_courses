# Bayesian decision theory, MAP, and ROC analysis

- Bayes' rule combines a model prior and data likelihood into a posterior; MAP
  maximizes posterior probability and ML maximizes likelihood (`BAYES`, pp. 3–10).
- Under zero-one loss, choosing the largest posterior class probability minimizes
  conditional and total misclassification risk (`BAYES`, pp. 11–15).
- General loss matrices yield minimum expected-cost decisions rather than
  minimum-error decisions (`BAYES`, pp. 15–19).
- MDL interprets model selection as minimizing description length; squared-error
  minimization corresponds to ML under Gaussian errors (`BAYES`, pp. 20–25).
- Varying a score threshold trades true-positive against false-positive rate;
  ROC curves and AUC summarize ranking across thresholds (`BAYES`, pp. 26–38).

## Related courses

- Prerequisite: [LEPL1109 — statistical decision theory and Bayes optimality](../LEPL1109/Statistical%20decision%20theory%20and%20Bayes%20optimality.md)
- Bayesian extension: [LDACS1110 — Bayesian inference](../LDACS1110/Bayesian%20inference.md)

