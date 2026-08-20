# Naive Bayes and Gaussian classifiers

- Naive Bayes factorizes the class-conditional joint distribution by assuming
  conditional independence of features given the class (`NB`, pp. 2–7).
- Discrete likelihoods use smoothed frequency estimates; continuous variants
  commonly fit per-class univariate Gaussians (`NB`, pp. 8–13).
- Despite misspecified independence, classification can remain effective because
  only posterior score ordering matters (`NB`, pp. 14–16).
- A multivariate Gaussian classifier estimates class priors, means, and
  covariance matrices (`GAUSS`, pp. 3–14).
- Shared covariance yields linear discriminant analysis; class-specific
  covariance yields quadratic boundaries (`GAUSS`, pp. 15–23).

## Related courses

- Probability prerequisite: [LEPL1109 — dependence and multivariate probability](../../EPL/1109/Dependence%20and%20multivariate%20probability.md)
- Projection counterpart: [Fisher discriminant analysis](Fisher%20discriminant%20analysis.md)
