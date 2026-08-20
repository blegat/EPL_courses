# LEPL1109: detailed course-content and prerequisite map

This document inventories the topics, methods, and principal results covered by
the LEPL1109 material available in this directory. It is intended to support the
design of a later course that takes LEPL1109 as a prerequisite.

## Reference convention and scope

- References use **physical PDF pages**, counted from the first page of each
  file. This remains unambiguous when a printed slide number differs from the
  PDF page.
- `STAT` =
  [`slides/Slides partie 1 statistics-20260814/Slides_25_26.pdf`](slides/Slides%20partie%201%20statistics-20260814/Slides_25_26.pdf).
- `APP` =
  [`slides/Slides partie 1 statistics-20260814/Appendix1.pdf`](slides/Slides%20partie%201%20statistics-20260814/Appendix1.pdf).
- `FORM` =
  [`slides/Slides partie 1 statistics-20260814/formulaire.pdf`](slides/Slides%20partie%201%20statistics-20260814/formulaire.pdf).
- `SUPP` =
  [`slides/Slides partie 1 statistics-20260814/slides_supplémentaires.pdf`](slides/Slides%20partie%201%20statistics-20260814/slides_suppl%C3%A9mentaires.pdf).
- `ERR` =
  [`slides/Slides partie 1 statistics-20260814/typo_25-26_stat.pdf`](slides/Slides%20partie%201%20statistics-20260814/typo_25-26_stat.pdf).
- `DS-I` = [`slides/DS_intro_handout.pdf`](slides/DS_intro_handout.pdf).
- `SL-1` = [`slides/SL_1_handout.pdf`](slides/SL_1_handout.pdf).
- `SL-2` = [`slides/SL_2_handout.pdf`](slides/SL_2_handout.pdf).
- `SL-3` = [`slides/SL_3_handout.pdf`](slides/SL_3_handout.pdf).
- `UL` = [`slides/UL_handout.pdf`](slides/UL_handout.pdf).
- The data-science exam scope includes all slides not tagged "Extra Material",
  lecture additions, and definitions/concepts from practical sessions and the
  hackathon. External links are excluded. Students must explain definitions and
  short derivations, diagnose false methodologies or code, and write or correct
  pseudocode; no formula sheet is allowed for this part [DS-I, p. 23].
- The appendix explicitly says its distributions were introduced in an earlier
  probability course and are **assumed mastered** [APP, p. 2]. They are therefore
  included below as incoming knowledge used by LEPL1109, not necessarily taught
  from scratch.
- Practical-session notebooks and hackathon files are not present in this
  directory. Consequently, Python details that occur only there cannot be
  inventoried. The lecture slides explicitly make that material examinable
  [DS-I, p. 23].

## Executive prerequisite profile

After LEPL1109, a later course can reasonably expect students to be able to:

1. Manipulate univariate and multivariate probability distributions,
   expectations, covariance matrices, conditional distributions, normal
   transformations, LLN, and CLT.
2. Explore and summarize data using means, quantiles, variance, covariance,
   correlation, histograms, boxplots, outlier rules, and basic preprocessing.
3. Estimate parametric models by moments and maximum likelihood; reason about
   estimator bias, variance, MSE, consistency, and asymptotic behavior.
4. Simulate random variables by inverse transforms and quantify estimator
   uncertainty using the nonparametric bootstrap.
5. Construct one-population confidence intervals for a normal mean, variance,
   and standard deviation; perform one- and two-population tests for normal
   means and variances using normal, Student, chi-square, and Fisher reference
   laws and p-values.
6. Fit and analyze simple and multiple linear regression, including OLS in
   matrix form, coefficient and global tests, and ANOVA decomposition; construct
   coefficient confidence intervals and the taught new-response prediction
   interval for simple regression.
7. Formulate supervised learning as empirical-risk minimization; distinguish
   parameters from hyperparameters, training from generalization, and
   parametric from nonparametric models.
8. Implement and assess least-squares classifiers, k-nearest neighbors, and
   binary logistic regression; use validation, LOOCV, K-fold CV, confusion
   matrices, precision, recall, F1, PR curves, ROC, and AUC.
9. Use statistical decision theory: loss, risk, conditional risk, Bayes risk,
   Bayes regression/classification, excess risk, and the bias-variance
   decomposition.
10. Perform PCA using covariance eigendecomposition/SVD, select dimensions via
    explained variance, transform and reconstruct observations, and understand
    centering/scaling requirements.
11. Formulate K-means as within-cluster variation minimization, run Lloyd's
    algorithm, select/diagnose clusters with silhouette scores, and recognize
    initialization, complexity, outlier, and geometry limitations.
12. Fit autoregressive time-series models, make recursive forecasts, and select
    lag order with AIC/BIC; formulate Gaussian-process regression and its
    posterior mean/variance from kernels.
13. Use the scientific Python ecosystem at an introductory level: NumPy,
    pandas, Matplotlib, SciPy, Statsmodels, and scikit-learn.

The course is an introduction rather than an advanced ML course [DS-I,
pp. 16-17]. Deep learning, semi/self-supervised learning, reinforcement learning,
SVM, LDA/QDA, hierarchical clustering, and density-based clustering are not
developed [DS-I, p. 13; SL-2, p. 22; UL, p. 40].

# Part A: probability and statistics

| Topic | Note |
|---:|---|
| 1 | [Probability foundations and random variables](Probability%20foundations%20and%20random%20variables.md) |
| 2 | [Descriptive statistics and exploratory data analysis](Descriptive%20statistics%20and%20exploratory%20data%20analysis.md) |
| 3 | [Dependence and multivariate probability](Dependence%20and%20multivariate%20probability.md) |
| 4 | [Normal approximations and reference laws](Normal%20approximations%20and%20reference%20laws.md) |
| 5 | [Parametric estimation](Parametric%20estimation.md) |
| 6 | [Simulation and bootstrap](Simulation%20and%20bootstrap.md) |
| 7 | [Sampling distributions and confidence intervals](Sampling%20distributions%20and%20confidence%20intervals.md) |
| 8 | [Hypothesis testing](Hypothesis%20testing.md) |
| 9 | [Linear regression and ANOVA](Linear%20regression%20and%20ANOVA.md) |
| 10 | [Time series and autoregressive models](Time%20series%20and%20autoregressive%20models.md) |
| 11 | [Gaussian-process regression](Gaussian-process%20regression.md) |

# Part B: data science and machine learning

| Topic | Note |
|---:|---|
| 12 | [Data-science framing and workflow](Data-science%20framing%20and%20workflow.md) |
| 13 | [Supervised-learning formulation](Supervised-learning%20formulation.md) |
| 14 | [Linear least squares and k-nearest neighbors](Linear%20least%20squares%20and%20k-nearest%20neighbors.md) |
| 15 | [Resampling, model assessment, and model selection](Resampling,%20model%20assessment,%20and%20model%20selection.md) |
| 16 | [Logistic regression and classification assessment](Logistic%20regression%20and%20classification%20assessment.md) |
| 17 | [Statistical decision theory and Bayes optimality](Statistical%20decision%20theory%20and%20Bayes%20optimality.md) |
| 18 | [Bias-variance tradeoff and dimensionality](Bias-variance%20tradeoff%20and%20dimensionality.md) |
| 19 | [Unsupervised learning](Unsupervised%20learning.md) |

# Part C: boundaries, caveats, and source corrections

| Topic | Note |
|---:|---|
| 20 | [Explicitly extra, deferred, or not covered](Explicitly%20extra,%20deferred,%20or%20not%20covered.md) |
| 21 | [Important modeling assumptions students have seen](Important%20modeling%20assumptions%20students%20have%20seen.md) |
| 22 | [Errata affecting substantive formulas](Errata%20affecting%20substantive%20formulas.md) |
| 23 | [Formula-sheet evidence of expected mastery](Formula-sheet%20evidence%20of%20expected%20mastery.md) |
| 24 | [Practical software exposure](Practical%20software%20exposure.md) |
