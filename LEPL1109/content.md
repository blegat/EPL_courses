# LEPL1109: detailed course-content and prerequisite map

This document inventories the topics, methods, and principal results covered by
the LEPL1109 material available in this directory. It is intended to support the
design of a later course that takes LEPL1109 as a prerequisite.

## Reference convention and scope

- References use **physical PDF pages**, counted from the first page of each
  file. This remains unambiguous when a printed slide number differs from the
  PDF page.
- `STAT` =
  [`Slides partie 1 statistics-20260814/Slides_25_26.pdf`](Slides%20partie%201%20statistics-20260814/Slides_25_26.pdf).
- `APP` =
  [`Slides partie 1 statistics-20260814/Appendix1.pdf`](Slides%20partie%201%20statistics-20260814/Appendix1.pdf).
- `FORM` =
  [`Slides partie 1 statistics-20260814/formulaire.pdf`](Slides%20partie%201%20statistics-20260814/formulaire.pdf).
- `SUPP` =
  [`Slides partie 1 statistics-20260814/slides_supplémentaires.pdf`](Slides%20partie%201%20statistics-20260814/slides_suppl%C3%A9mentaires.pdf).
- `ERR` =
  [`Slides partie 1 statistics-20260814/typo_25-26_stat.pdf`](Slides%20partie%201%20statistics-20260814/typo_25-26_stat.pdf).
- `DS-I` = [`DS_intro_handout.pdf`](DS_intro_handout.pdf).
- `SL-1` = [`SL_1_handout.pdf`](SL_1_handout.pdf).
- `SL-2` = [`SL_2_handout.pdf`](SL_2_handout.pdf).
- `SL-3` = [`SL_3_handout.pdf`](SL_3_handout.pdf).
- `UL` = [`UL_handout.pdf`](UL_handout.pdf).
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

## 1. Probability foundations and random variables

### Random variables and distributions

- Experiment, sample space, events, probability, random variable as a map
  `X: Omega -> R`, realization, range/state space, and discrete versus
  continuous random variables [STAT, pp. 7-8].
- PMF, PDF, and CDF:
  \(p(x)=P(X=x)\), \(P(X\in I)=\sum_{x\in I}p(x)\),
  \(P(X\in I)=\int_I f(x)dx\), and \(F(x)=P(X\le x)\), with
  \(f=F'\) in the continuous case [STAT, pp. 9-12].
- Affine density transformation: if \(Y=a+bX\), then
  \(f_Y(y)=|b|^{-1}f_X((y-a)/b)\) [STAT, pp. 22-23].

### Expectation, moments, and variability

- Discrete/continuous expectation; empirical mean; center-of-mass
  interpretation; linearity; expectation of a transformed variable; raw
  moments \(E[X^k]\) [STAT, pp. 13-15].
- Students are warned that generally \(E[h(X)]\ne h(E[X])\) and
  \(E[XY]\ne E[X]E[Y]\) [STAT, p. 15].
- Variance and standard deviation:
  \(V(X)=E[(X-E[X])^2]=E[X^2]-E[X]^2\), affine-transformation rules, and
  sample variance \(S^2=(n-1)^{-1}\sum_i(X_i-\bar X)^2\) [STAT, pp. 16-17].
- Moment-generating functions: \(M_X(t)=E[e^{tX}]\), recovery of moments from
  derivatives at zero, \(M_{aX+b}(t)=e^{bt}M_X(at)\), products for sums of
  independent variables, and characterization of a distribution [FORM, p. 1].
- Law of large numbers for uncorrelated variables with common finite mean and
  variance: \(\bar X_n\to\mu\) in probability, supported by
  \(E\bar X_n=\mu\) and \(V(\bar X_n)=\sigma^2/n\) [STAT, p. 18].
- Quantiles: in general \(q_p=\inf\{x:F(x)\ge p\}\). Under suitable continuity
  and invertibility assumptions, \(F(q_p)=p\) and \(q_p=F^{-1}(p)\). The course
  develops the upper-tail relation and an exponential-lifetime calculation
  [STAT, pp. 19-21].

### Assumed distribution family

The following distributions, including their support, PMF/PDF, mean, variance,
and listed transformation/additivity properties, are assumed mastered [APP,
p. 2]:

- Discrete uniform [APP, p. 4].
- Bernoulli [APP, p. 5].
- Binomial: independent-trial construction, PMF, sum of Bernoulli variables,
  additivity at common \(p\), MGF, mean, and variance [APP, pp. 6-9].
- Geometric: waiting time to first success, PMF, MGF, mean, and variance [APP,
  p. 10].
- Poisson: count interpretation, PMF, MGF, mean/variance \(\lambda\), Poisson
  limit, and approximation \(Bin(n,p)\approx Po(np)\) for large \(n\), small
  \(p\) [APP, pp. 11-12].
- Continuous uniform: PDF, CDF, MGF, mean, and variance [APP, p. 14].
- Normal and standard normal: density, shape, parameters, MGF, tail quantiles,
  symmetry, standardization, and affine transformations [APP, pp. 15-23].
- Exponential: nonnegative waiting-time model, density, MGF, mean, and variance
  [APP, p. 24].
- Gamma: density, gamma function, shape/scale parameters, exponential special
  case, MGF, scaling, and additivity at common scale [APP, pp. 25-27].
- Chi-square as a sum of squared independent standard normals and as
  \(Gamma(n/2,2)\), with mean \(n\) and variance \(2n\) [APP, pp. 28-29].
- Student \(t_n=Z/\sqrt{Y/n}\) for independent \(Z\sim N(0,1)\) and
  \(Y\sim\chi_n^2\), including symmetry and convergence to normal [APP,
  pp. 30-31].
- Fisher-Snedecor \(F=(X/n_1)/(Y/n_2)\), reciprocal property, and use in
  variance comparison [APP, pp. 32-33].
- Bivariate and multivariate normal distributions, covariance parameterization,
  normal linear combinations, zero-correlation/independence equivalence in the
  jointly normal case, and affine closure [APP, pp. 34-37].

## 2. Descriptive statistics and exploratory data analysis

- Population, sample, and descriptive statistics; loading tabular data and
  obtaining a summary with pandas [STAT, pp. 25-27, 30].
- Sample mean, median, empirical quantiles, and order statistics [STAT,
  pp. 28-30].
- Range, interquartile range, suspected-outlier rule
  \(x<q_{.25}-1.5IQR\) or \(x>q_{.75}+1.5IQR\), and the need to investigate
  measurement/encoding errors [STAT, pp. 31-33].
- Boxplots, histograms as empirical distribution views, sample variance, and
  sample standard deviation [STAT, pp. 34-36].
- General data-science workflow: inspect bad/missing labels and errors, use
  ranges/histograms/plots, impute or correct anomalies, recode, regroup, smooth,
  subset, and present each visualization for a defined analytical purpose
  [DS-I, pp. 9-11].

## 3. Dependence and multivariate probability

### Independence, covariance, and correlation

- Independence as factorization of joint probabilities/densities [STAT, p. 38].
- Covariance
  \(C(X,Y)=E[(X-E[X])(Y-E[Y])]=E[XY]-E[X]E[Y]\), empirical covariance,
  bilinearity, symmetry, and
  \(V(aX+bY)=a^2V(X)+b^2V(Y)+2abC(X,Y)\) [STAT, pp. 39-42].
- Independence implies zero covariance, but zero covariance need not imply
  independence; the course uses \(Y=X^2\) as a counterexample [STAT, p. 42].
- Correlation \(\rho=C(X,Y)/(\sigma_X\sigma_Y)\), empirical correlation,
  Cauchy-Schwarz bound \(|\rho|\le1\), scale invariance, affine equality case,
  and the limitation to linear dependence [STAT, pp. 43-45].

### Random vectors and conditioning

- Random vectors, joint continuous/discrete distributions, marginalization by
  integration/summation, and conditional PDF/PMF [STAT, pp. 60-66].
- Mean vector, expectation of multivariate functions, vector linearity, and
  conditional expectation [STAT, pp. 67-68].
- Tower property \(E[X_A]=E[E(X_A\mid X_B)]\) and MMSE result
  \(E[X_A\mid X_B]=\arg\min_gE[\|X_A-g(X_B)\|_2^2]\), with an ordinary square
  in the scalar case [STAT, pp. 69-71].
- Covariance matrix
  \(\Sigma=E[(X-\mu)(X-\mu)^T]\), entries as variances/covariances, and affine
  transformation \(\Sigma_{MX+m}=M\Sigma_XM^T\) [STAT, pp. 71-73].
- Mutual independence by factorization; consequences for conditioning and a
  diagonal covariance matrix [STAT, pp. 73-75].
- Multivariate normal density, affine closure, marginal normals, whitening, and
  diagonal-covariance/independence equivalence [STAT, pp. 76-78].
- Conditional multivariate-normal theorem. For a partition into \(A,B\),
  \(X_B\mid X_A=x_A\) has mean
  \(\mu_B+\Sigma_{BA}\Sigma_{AA}^{-1}(x_A-\mu_A)\) and covariance
  \(\Sigma_{BB}-\Sigma_{BA}\Sigma_{AA}^{-1}\Sigma_{AB}\) [STAT, pp. 79-81].

## 4. Normal approximations and reference laws

- Normal density, mean/variance, MGF, and numerical PDF/CDF/quantile evaluation
  [STAT, pp. 47-48].
- Standardization and affine transformation of normals; means and variances of
  linear combinations [STAT, p. 49]. Joint normality is the condition needed
  for the combination itself to be normal.
- Central limit theorem:
  \(\sqrt n(\bar X_n-\mu)/\sigma\Rightarrow N(0,1)\), hence
  \(\bar X_n\approx N(\mu,\sigma^2/n)\) and
  \(\sum X_i\approx N(n\mu,n\sigma^2)\) [STAT, pp. 50-52].
- Binomial normal approximation [STAT, pp. 51-52].
- Definitions, densities, moments, numerical use, and constructions of
  chi-square, Student, and Fisher laws [STAT, pp. 54-58].

## 5. Parametric estimation

### Estimators and their quality

- Parametric family \(f(x\mid\theta)\), i.i.d. random sample, estimator as a
  statistic, and estimate as its observed value [STAT, pp. 83-85].
- Unbiasedness, bias, estimator variance, MSE, and
  \(MSE(\hat\theta)=Bias(\hat\theta)^2+V(\hat\theta)\) [STAT, pp. 86-87].
- Model-family selection is illustrated by comparing empirical histograms to
  candidate distributions and respecting support [STAT, p. 84].

### Method of moments

- Match \(d\) theoretical moments \(E[X^k]\) to empirical moments
  \(n^{-1}\sum_iX_i^k\) and solve for a \(d\)-parameter model [STAT,
  pp. 88-89].
- Worked estimators for exponential, normal, gamma, Bernoulli, and bivariate
  normal parameters [STAT, pp. 90-93].
- Sample mean unbiasedness and CLT sampling approximation; consistency of
  moment estimators; ease versus lower statistical efficiency than MLE [STAT,
  pp. 94-95].

### Maximum likelihood

- Likelihood \(L(\theta)=\prod_i f(x_i\mid\theta)\), log-likelihood as a sum,
  score equations, and comparison of candidate fits by likelihood [STAT,
  pp. 97-99].
- MLE asymptotics: under suitable regularity conditions, asymptotic
  unbiasedness, normality, and efficiency among the relevant regular
  asymptotically unbiased estimators. No general finite-sample minimum-variance
  result is established [STAT, p. 100].
- MLEs for exponential, Bernoulli, Poisson, and normal models [STAT,
  pp. 99, 101-105].
- Normal variance MLE uses denominator \(n\) and has expectation
  \((n-1)\sigma^2/n\); replacing it by denominator \(n-1\) gives the unbiased
  sample variance [STAT, pp. 104-105].

## 6. Simulation and bootstrap

- Pseudorandom-number generators and the linear congruential generator
  \(X_{n+1}=(aX_n+c)\bmod m\) [STAT, pp. 107-108].
- Inverse-transform theorem: if \(U\sim U[0,1]\), then \(F^{-1}(U)\) has CDF
  \(F\); simulation procedure and SciPy generation for common laws [STAT,
  pp. 109-110].
- Nonparametric bootstrap: sample with replacement from the empirical
  distribution, recompute \(\hat\theta\) over \(M\) replications, and estimate
  its mean, variance, sampling distribution, and percentile confidence interval
  [STAT, pp. 111-117].
- Empirical CDF \(\hat F_n(x)=n^{-1}\sum_i1_{X_i\le x}\), bootstrap sample and
  replication, bootstrap variance, and percentile interval formulas [STAT,
  pp. 112-115].
- Worked result: reducing the original sample size widens the bootstrap
  interval, illustrating uncertainty reduction with more data [STAT,
  pp. 116-117].

## 7. Sampling distributions and confidence intervals

### One normal population

- Sampling distribution of \(\bar X\), exact under normality and approximate by
  CLT; known-variance normal confidence interval for \(\mu\) [STAT,
  pp. 119-121].
- Unbiasedness of \(S^2\), independence of \(\bar X\) and \(S^2\) for normal
  data, and
  \((n-1)S^2/\sigma^2\sim\chi^2_{n-1}\) [STAT, pp. 122-124].
- Chi-square confidence intervals for variance and standard deviation [STAT,
  pp. 125-126].
- Studentized mean
  \((\bar X-\mu)/(S/\sqrt n)\sim t_{n-1}\) and unknown-variance confidence
  interval for \(\mu\) [STAT, pp. 127-130].

### Two independent normal populations

- Difference of means distribution with known variances [STAT, p. 132].
- Variance-ratio pivot
  \(S_1^2\sigma_2^2/(S_2^2\sigma_1^2)\sim F_{n_1-1,n_2-1}\) [STAT, p. 133].
- Pooled variance for equal unknown variances, its chi-square law, and the pooled
  two-sample Student pivot [STAT, pp. 134-136].
- Welch's unequal-variance two-sample procedure is not developed.

`FORM`, pp. 1-2 consolidates these pivots, degrees of freedom, and quantile
conventions, indicating that students are expected to select the correct law
from the assumptions rather than merely memorize isolated formulas.

## 8. Hypothesis testing

### General framework

- Null/alternative hypotheses, statistical decision rule, Type I and Type II
  errors, significance level, test statistic, rejection region, and observed
  statistic [STAT, pp. 138-141].
- General method: choose \(\alpha\), construct a null rejection region of
  probability \(\alpha\), compute the statistic, and reject iff it lies in the
  region [STAT, p. 141].
- p-value as the smallest significance level producing rejection; lower-tail,
  upper-tail, and symmetric two-sided formulas; reject iff \(p<\alpha\)
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

## 9. Linear regression and ANOVA

### Multiple and simple linear regression

- Multiple Gaussian linear model
  \(Y_i=\beta_0+\sum_{j=1}^d\beta_jx_{ij}+\epsilon_i\),
  \(\epsilon\sim N(0,\sigma^2I)\), matrix form \(Y=X\beta+\epsilon\),
  conditional mean, and response distribution [STAT, pp. 167-172].
- Gaussian MLE is equivalent to least squares. OLS minimizes
  \((y-X\beta)^T(y-X\beta)\), giving normal equations and
  \(\hat\beta=(X^TX)^{-1}X^Ty\) when invertible [STAT, pp. 172-173].
- Fitted values, residuals, and hat matrix
  \(H=X(X^TX)^{-1}X^T\), including symmetry and idempotence [STAT, p. 174].
- Simple-regression slope/intercept formulas and interpretation [STAT,
  pp. 175-176].
- Total, residual, and regression sums of squares:
  \(SST=SSE+SSR\), and \(R^2=SSR/SST=1-SSE/SST\) as explained-variance
  proportion [STAT, pp. 177-179].

### Regression inference

- Sampling law
  \(\hat\beta\sim N(\beta,\sigma^2(X^TX)^{-1})\), unbiasedness, and residual
  variance estimator \(\hat\sigma^2=SSE/[n-(d+1)]\) [STAT, pp. 181-182].
- Residual chi-square law
  \(SSE/\sigma^2\sim\chi^2_{n-d-1}\) [STAT, pp. 182-185].
- Variance and covariance of simple-regression coefficients and plug-in
  standard errors [STAT, pp. 186-187].
- Global significance test
  \(H_0:\beta_1=\cdots=\beta_d=0\) with
  \(F^*=(SSR/d)/(SSE/(n-d-1))\sim F_{d,n-d-1}\) [STAT, pp. 188-190].
- Individual coefficient Student tests and confidence intervals based on the
  diagonal of \((X^TX)^{-1}\) [STAT, pp. 191-197].
- New-response prediction interval in simple regression, including variance
  \(\hat\sigma^2[1+1/n+(x_0-\bar x)^2/S_{xx}]\) [STAT, p. 198].
- Statsmodels use and interpretation of estimates, standard errors, t-tests,
  p-values, confidence intervals, and model summaries [STAT, pp. 187, 195-197].

### One-factor ANOVA

- Equality-of-means testing for multiple independent normal populations,
  motivation versus repeated pairwise tests, and expression as categorical
  linear regression [STAT, pp. 199-204].
- Dummy encoding with one reference category; including an intercept and every
  dummy causes rank deficiency [STAT, pp. 202-203].
- Assumptions: normality, equal within-group variances, and independence.
  Equality of group means is tested by the regression global F-test; Bartlett's
  test can assess equal variances [STAT, pp. 204-205].

## 10. Time series and autoregressive models

- Time series \((X_t)\), mean function, autocovariance
  \(\gamma(t,s)\), autocorrelation \(\rho(t,s)\), and interpretation of
  persistence [STAT, pp. 207-210; SUPP, pp. 2-5].
- AR(\(p\)) model
  \(X_t=\sum_{j=1}^p\beta_jX_{t-j}+\epsilon_t\), Gaussian innovations, and
  \(E[X_t\mid X_{t-1:t-p}=x_{t-1:t-p}]=\sum_j\beta_jx_{t-j}\), with constant
  conditional variance [STAT, p. 211; SUPP, p. 6].
- Construction of lagged design matrix, OLS/MLE fit, innovation-variance
  estimate, and Statsmodels `AutoReg` implementation [STAT, pp. 212-214;
  SUPP, pp. 7-9].
- Recursive multi-step forecasting and degradation with forecast horizon
  because predictions become later inputs [STAT, pp. 215-216; SUPP,
  pp. 10-11].
- Lag-order/model selection: likelihood alone overfits; minimize
  \(AIC=2p-2\log L\) or \(BIC=(\log n)p-2\log L\). BIC penalizes complexity
  more strongly [STAT, pp. 217-220; SUPP, pp. 12-15].
- Partial autocorrelation is explicitly mentioned but not covered [STAT,
  p. 220; SUPP, p. 15].

## 11. Gaussian-process regression

- Motivation: flexible nonlinear, nonparametric regression with a predictive
  distribution and uncertainty [STAT, p. 222].
- Gaussian process definition: every finite function-value vector is jointly
  normal; mean function \(m(x)\), covariance kernel \(k(x,x')\), and Gram
  matrix [STAT, pp. 223-224].
- Observation model \(y_i=f(x_i)+\epsilon_i\),
  \(\epsilon_i\sim N(0,\sigma_\epsilon^2)\), and
  \(Y\sim N(0,K+\sigma_\epsilon^2I)\) [STAT, p. 225; ERR, p. 1, correction
  for slide 232].
- Joint Gaussian conditioning gives posterior mean
  \(k(x_*,X)^T[K(X,X)+\sigma_\epsilon^2I]^{-1}y\) and posterior variance
  \(k(x_*,x_*)-k(x_*,X)^T[K+\sigma_\epsilon^2I]^{-1}k(X,x_*)\)
  [STAT, pp. 226-227; ERR, p. 1, correction for slide 227].
- The noise term both models observation noise and regularizes Gram-matrix
  inversion [STAT, p. 227].
- Valid kernels are symmetric and produce positive-semidefinite Gram matrices;
  kernels encode similarity, smoothness, and periodicity [STAT, p. 229].
- RBF, Matern, and rational-quadratic kernels and their hyperparameters [STAT,
  pp. 230-231].
- Kernel hyperparameters are fitted by maximizing Gaussian marginal likelihood;
  scikit-learn's `GaussianProcessRegressor` is demonstrated [STAT, p. 232].

# Part B: data science and machine learning

## 12. Data-science framing and workflow

- Historical motivation through John Snow's cholera map and Gauss/Legendre's
  least-squares orbit fitting for Ceres [DS-I, pp. 4-7].
- Data-science goals: automatically extract information while cleaning,
  processing, visualizing, and controlling computational complexity; tasks
  include prediction, regression, classification, dimensionality reduction,
  clustering, restoration, denoising, compression, and transmission [DS-I,
  p. 8].
- Six divisions: exploration/preparation, representation/transformation,
  computing with data, generative and predictive modeling, visualization and
  presentation, and research on data-science workflows/methods [DS-I,
  pp. 9-11].
- Machine learning is framed as an algorithm taking a dataset and returning a
  predictor. Supervised learning uses labeled data for regression or
  classification; unsupervised learning finds structure, clusters, or
  lower-dimensional representations [DS-I, pp. 12-13].
- Statistics is framed primarily around inference and distribution properties,
  while the second part emphasizes prediction accuracy, metrics, datasets, and
  validated processing methodology [DS-I, pp. 15-16].
- Hackathon capability: discover a real dataset; explore, clean, process, and
  visualize it; make predictions; estimate errors; validate results; and report
  a reproducible methodology with plots, Python scripts/notebooks, and a short
  referenced report [DS-I, pp. 19-20].

## 13. Supervised-learning formulation

### Data, objectives, and preprocessing

- Notation: dataset size \(N\), feature dimension \(p\), design matrix
  \(X\in R^{N\times p}\), random variables versus observations, indicators,
  i.i.d. samples, and basic set notation [SL-1, pp. 4-7].
- Regression predicts ordered/quantitative values; classification predicts
  qualitative categories. The boundary is application-dependent [SL-1,
  pp. 8-9].
- One-hot/dummy encoding maps \(K\) categories to canonical vectors in
  \(R^K\) [SL-1, p. 10].
- Features/input/predictors/covariates \(X\) and outcome/label/response \(Y\);
  noisy functional relation for regression and noisy labels for binary
  classification [SL-1, p. 11].
- Goal: learn a reliable \(\hat f\) from representative samples. Training
  consists of obtaining data, preprocessing, and fitting; prediction must apply
  **exactly the same preprocessing** to independent data [SL-1, pp. 12-13].
- Preprocessing examples: missing-value imputation, categorical encoding,
  outlier handling, normalization/standardization, and dimensionality reduction
  [SL-1, p. 13].

### Models, losses, and accuracy

- Learning algorithm \(A\) selects \(\hat f\in\mathcal F_\gamma\) from a
  restricted function class; assumptions are unavoidable. Parameters \(\beta\)
  are fitted, while hyperparameters \(\gamma\) characterize the model class
  [SL-1, pp. 38-39; SL-2, p. 3].
- Parametric models: explicit finite parameterization, loss minimization,
  feature maps \(\phi(X)\), linear and logistic examples, optimization and
  overfitting/interpretability tradeoffs [SL-1, pp. 39-40].
- Nonparametric models: no explicit finite functional form but implicit
  assumptions, e.g. piecewise-constant behavior for k-NN; more flexibility and
  data demand, hyperparameter selection, and weaker interpretability [SL-1,
  pp. 41-42].
- Squared-loss risk for fixed \(\hat f,X\) decomposes into reducible model error
  \([f(X)-\hat f(X)]^2\) and irreducible noise variance [SL-1, p. 44].
- Empirical MSE for regression and 0/1 misclassification rate for
  classification are training empirical risks [SL-1, p. 45].
- Training error may be zero while unseen-data risk is not; this is overfitting.
  Test error typically has a U-shape versus flexibility, motivating validation
  [SL-1, pp. 46-47].

## 14. Linear least squares and k-nearest neighbors

### Multivariate linear model and least squares

- Linear model \(\hat Y=X^T\hat\beta\), with optional intercept; global,
  parametric, and geometrically a fitted hyperplane [SL-1, pp. 18-19].
- Empirical squared risk
  \(N^{-1}\|y-X\beta\|^2\), vertical-distance interpretation, and distinction
  from PCA's orthogonal reconstruction criterion [SL-1, p. 20].
- Assuming \(N>p\) and invertible \(X^TX\),
  \(\hat\beta=(X^TX)^{-1}X^Ty=X^\dagger y\), fitted vector
  \(\hat y=Hy\), and hat matrix \(H=XX^\dagger\) [SL-1, p. 21].
- Binary LS classifier = continuous LS regression followed by thresholding;
  this yields a linear decision boundary but can produce uninterpretable values
  outside \([0,1]\) and cannot represent nonlinear class geometry [SL-1,
  pp. 22-26].

### k-nearest neighbors

- \(N_k(x)\) is the set of the \(k\) closest training inputs. k-NN regression
  predicts their average label, making it a local, nonparametric estimator
  [SL-1, p. 27].
- Binary k-NN classification thresholds the local average at 0.5, equivalent to
  majority voting; multiclass prediction chooses the most represented class
  [SL-1, p. 28].
- Small \(k\) yields irregular, highly flexible boundaries; \(k=1\) induces a
  Voronoi tessellation and zero training error. \(k=N\) reduces to global
  majority voting [SL-1, pp. 29-32].
- Classification error
  \(N^{-1}\sum_i1_{y_i\ne\hat f(x_i)}\); training error alone cannot select
  \(k\) or establish generalization [SL-1, p. 33].
- Effective flexibility/number of parameters is approximately \(N/k\), not
  \(k\) [SL-1, p. 35].
- k-NN has mild explicit assumptions and nonlinear boundaries but depends on
  the distance and preprocessing, needs hyperparameter selection, and can be
  unstable at high flexibility [SL-1, pp. 34-36].
- Scikit-learn estimator workflow: instantiate a model, call `fit(X,y)`, then
  `predict(X_new)` [SL-1, p. 43].

## 15. Resampling, model assessment, and model selection

- Generalization error is empirical risk on an independent dataset. Resampling
  simulates held-out data from the available sample to assess models and select
  model family, features, or hyperparameters [SL-2, pp. 3-7].
- The workflow separates a final test set, resamples the available data for
  training/validation, selects the model, refits on all available data, and
  evaluates on test data [SL-2, p. 6]. The slides place preprocessing before
  these splits and do not develop leakage-safe fitting of transformations
  inside folds, so a later course should not assume mastery of pipelines or
  leakage prevention.
- Validation-set approach: random permutation and split, fit on training data,
  estimate MSE/error on held-out validation data. Randomization avoids a
  structurally biased split when ordering induces dependence [SL-2, pp. 8-11].
- Validation drawbacks: estimate variability across random splits and possible
  test-error overestimation because the model sees fewer training observations
  [SL-2, p. 12].
- LOOCV: fit \(N\) models, each excluding one observation, and average its
  one-point prediction loss. It uses almost all data and helps parameter
  selection but is expensive [SL-2, pp. 13-16].
- K-fold CV: randomize, partition into \(K\) folds, fit \(K\) times while each
  fold serves once as validation, and average fold errors. Typical \(K\) is 5
  or 10; it is much cheaper than LOOCV [SL-2, pp. 17-19].
- Bootstrap is named as another resampling method but is not covered in the
  data-science lectures because it appears in the statistics part [SL-2, p. 7;
  STAT, pp. 111-117].

## 16. Logistic regression and classification assessment

### Logistic regression

- Linear classification partitions the feature space with hyperplane decision
  boundaries from discriminant functions [SL-2, p. 23].
- Least-squares classification has arbitrary category-ordering problems for
  more than two classes and lacks probability interpretation even for binary
  coding [SL-2, pp. 24-25].
- Logistic regression models
  \(h_\beta(x)=P(Y=1\mid X=x)=S(\beta^Tx)\), where
  \(S(t)=e^t/(1+e^t)\). Thresholding this probability gives a classifier
  [SL-2, pp. 26-30].
- Log-odds/logit is linear:
  \(\log(P(Y=1\mid X)/P(Y=0\mid X))=\beta^TX\). At threshold 0.5 the
  equiprobability boundary is \(\beta^Tx=0\); coefficient signs describe how
  features change positive-class probability [SL-2, pp. 28-32].
- Under independent Bernoulli conditional outcomes, MLE minimizes negative
  log-likelihood/binary cross-entropy
  \(\sum_i[\log(1+e^{\beta^Tx_i})-y_i\beta^Tx_i]\). The objective is convex
  with gradient \(\sum_i(S(\beta^Tx_i)-y_i)x_i\) [SL-2, pp. 33-35].
- Gradient descent, Newton-Raphson, and LBFGS are named solvers; scikit-learn
  fitting/prediction is demonstrated on digit `3` versus `not 3` [SL-2,
  pp. 35-38].

### Classification metrics and thresholds

- Accuracy = \(1-\)misclassification rate, estimated with CV; accuracy can be
  misleading for imbalanced classes because a constant majority classifier may
  score highly [SL-2, p. 39].
- True/false positives and negatives and the binary/multiclass confusion matrix;
  obtain out-of-fold predictions before computing it to avoid training-data
  optimism [SL-2, pp. 40-42].
- Precision \(TP/(TP+FP)\), recall/sensitivity/TPR \(TP/(TP+FN)\), their
  conditional-probability interpretations, and task-dependent tradeoff [SL-2,
  pp. 43-45].
- F1 is the harmonic mean of precision and recall [SL-2, p. 46].
- Changing the decision threshold trades precision against recall; PR curves
  visualize achievable pairs and threshold choice must reflect application
  costs [SL-2, pp. 47-49].
- ROC plots TPR versus FPR \(=FP/(FP+TN)\); AUC near 1 indicates a strong
  ranking classifier, while random classification gives about 0.5. PR is
  preferred for rare positives/imbalanced classes or when false positives
  matter especially; ROC is suggested otherwise [SL-2, p. 50].

## 17. Statistical decision theory and Bayes optimality

- Inputs and outputs are jointly distributed, and the training set is an i.i.d.
  sample from this population [SL-3, pp. 3-4].
- Losses: squared error, 0/1 loss, and logistic/cross-entropy loss [SL-3, p. 5].
- Expected risk \(R(f)=E[\ell(Y,f(X))]\), empirical risk, and convergence of
  empirical to expected risk under suitable conditions [SL-3, p. 6].
- Marginal and conditional distributions, Bayes' rule, and law of total
  expectation are reviewed to express risk as expected conditional risk
  [SL-3, pp. 7-11].
- Bayes predictor
  \(f^*(x)=\arg\min_zE[\ell(Y,z)\mid X=x]\), Bayes risk \(R^*=R(f^*)\),
  lower bound \(R(f)\ge R^*\), and excess risk \(R(f)-R^*\) [SL-3, p. 12].
- For squared loss, the Bayes regressor is the conditional mean
  \(f^*(x)=E[Y\mid X=x]\); the deck proves this by completing the square
  [SL-3, pp. 13-14].
- k-NN approximates that conditional mean by sample averaging in a neighborhood.
  Consistency is stated for \(N,k\to\infty\), \(k/N\to0\), under regularity
  conditions [SL-3, p. 15].
- For 0/1 loss, the Bayes classifier selects the most probable conditional
  class: \(f^*(x)=\arg\max_gP(Y=g\mid X=x)\), with
  \(R^*=1-E_X\max_gP(Y=g\mid X)\) [SL-3, p. 16].
- A Gaussian-mixture example derives the Bayes decision by comparing
  class-conditional density times prior and compares it with k-NN [SL-3,
  pp. 17-19].

## 18. Bias-variance tradeoff and dimensionality

### Bias-variance decomposition

- Overfitting means excessive model flexibility; underfitting means too little.
  Model parameters, feature choices, and hyperparameters all affect flexibility
  [SL-3, p. 22].
- For \(Y=f(X)+\epsilon\), \(E\epsilon=0\), \(V\epsilon=\sigma_\epsilon^2\),
  and a fitted predictor random through its training dataset, expected test
  error at \(x_0\) decomposes as
  \[
  EPE(x_0)=\sigma_\epsilon^2+
  (f(x_0)-E_T\hat f(x_0))^2+V_T(\hat f(x_0)).
  \]
  The terms are irreducible noise, squared bias, and variance [SL-3,
  pp. 23-26].
- Typically bias decreases and variance increases with flexibility, producing a
  U-shaped test error and motivating model selection [SL-3, p. 26].
- For k-NN, flexibility is \(N/k\); under fixed-neighbor simplifications the
  variance is \(\sigma_\epsilon^2/k\), while increasing \(k\) raises local
  averaging bias [SL-3, pp. 27-29].

### Curse of dimensionality (explicitly extra material)

- In high dimension, most volume lies near a shell/boundary and fixed local
  neighborhoods become large in each coordinate [SL-3, pp. 31-34].
- For uniform points in \([0,1]^p\), a cube of side \(r\) contains about
  \(r^pN\) points, so obtaining \(k\) neighbors requires
  \(r\approx(k/N)^{1/p}\to1\) as \(p\) grows [SL-3, p. 33].
- Maintaining fixed sampling density requires sample size exponential in
  dimension; local nonparametric methods such as k-NN are especially affected
  [SL-3, pp. 32-34].
- This section is tagged **Extra Material** [SL-3, pp. 2, 30-34], so it should
  not be treated as firm examinable prerequisite knowledge.

## 19. Unsupervised learning

### Scope and limitations

- Unsupervised learning has observations \(x_i\) but no labels. Its aims are to
  discover arrangements, clusters, patterns, and hierarchies rather than make
  supervised predictions [UL, pp. 4-5].
- The course focuses on PCA for dimensionality reduction/visualization and
  preprocessing, and K-means for subgroup discovery [UL, p. 5].
- UL objectives and performance are less universally defined; analysis is often
  explanatory and lacks the simple validation target available in supervised
  learning [UL, p. 6].

### Principal component analysis

- Dimensionality-reduction objective: map data from \(R^p\) to \(R^{p'}\),
  \(p'\ll p\), while retaining essential information. Merely selecting pairs
  of original axes is combinatorial and restricts the representation [UL,
  pp. 8-9].
- PCA finds axes maximizing projected variance for centered data [UL, p. 10].
- For unit direction \(\phi\), scores are \(z_i=x_i^T\phi\) and directional
  variance is \(V(\phi)=N^{-1}\|X\phi\|^2\) [UL, pp. 11-12].
- The first PC maximizes the Rayleigh quotient and is the leading eigenvector of
  empirical covariance \(C=N^{-1}X^TX\), with eigenvalue equal to explained
  variance. Later PCs maximize variance subject to orthogonality to prior PCs
  [UL, pp. 13-15].
- All PCs form an orthonormal basis adapted to the data; an observation can be
  expanded in canonical coordinates or PC scores [UL, p. 16].
- For centered \(X\in R^{N\times p}\), \(rank(X)\le\min(p,N-1)\), so at most
  \(\min(p,N-1)\) PCs have nonzero explained variance [UL, p. 17].
- The first \(k\) PCs span the \(k\)-dimensional subspace of maximum variance,
  equivalently the subspace minimizing total squared orthogonal reconstruction
  distance [UL, pp. 17-18].
- Forward transform stores scores
  \(z_i=(\phi_1^Tx_i,\ldots,\phi_k^Tx_i)\); inverse transform reconstructs
  \(x_i' =\sum_{j=1}^kz_{ij}\phi_j\) [UL, p. 19].
- Total variance is invariant under the orthonormal change of basis. Proportion
  of variance explained is \(PVE(j)=V(\phi_j)/V_X\), decreases with PC index,
  and cumulative PVE supports choosing \(k\) through scree/cumulative plots
  [UL, pp. 20-22].
- Data must be centered. Features should often be standardized to unit variance
  for meaningful PCA, but this can be inappropriate when physical scaling is
  meaningful [UL, p. 23].
- In practice PCA uses SVD \(X=U\Sigma V^T\); PCs are columns of \(V\), their
  variances relate to squared singular values, and PC signs are arbitrary.
  Randomized methods address very large dimensions [UL, p. 24].
- Scikit-learn forward/inverse transforms and explained-variance attributes are
  demonstrated on 3-D Gaussian data and handwritten digits [UL, pp. 25-32].
- PCA applications: visualization, interpretable directions of variation,
  feature extraction/noise filtering, and preprocessing before supervised
  learning [UL, pp. 30-35]. Eigenfaces are tagged extra material [UL,
  pp. 33-34].

### Clustering and K-means

- Clustering discovers homogeneous subgroups. The number/meaning of clusters is
  often application-dependent and ill-posed; unlike PCA, clustering reduces
  data into groups rather than a lower-dimensional linear space [UL,
  pp. 37-38].
- Applications include medicine, image/video segmentation, biology/ecology,
  recommender systems, network analysis, and marketing [UL, p. 39].
- Partitioning, hierarchical, density-based, connectivity-based, and
  centroid-based families are identified, but only K-means is developed [UL,
  p. 40].
- A clustering \(\mathcal C=\{C_1,\ldots,C_K\}\) partitions observation
  indices. K-means minimizes within-cluster pairwise squared variation [UL,
  pp. 41-43].
- Identity relating pairwise variation to centroid distances gives the standard
  objective \(\sum_j\sum_{i\in C_j}\|x_i-c_j\|^2\), where
  \(c_j=|C_j|^{-1}\sum_{i\in C_j}x_i\). Global optimization is NP-hard [UL,
  p. 43].
- Lloyd-Max algorithm alternates nearest-centroid assignment and centroid
  recomputation until within-cluster variation stops decreasing [UL,
  pp. 44-50].
- Number of clusters \(K\) is a hyperparameter with no universal selection
  rule. Silhouette score uses mean within-cluster distance \(a(x)\), nearest
  other-cluster distance \(b(x)\), and
  \(s(x)=(b-a)/\max(a,b)\in[-1,1]\) [UL, pp. 51-53].
- Good solutions seek high average silhouette, avoid clusters below that
  average, and avoid wide within-cluster score fluctuations; silhouette plots
  compare candidate \(K\) [UL, pp. 53-57].
- A 64-dimensional digit example shows centroids, arbitrary cluster-label
  permutation, and post-hoc comparison to true labels [UL, pp. 58-60].
- Lloyd iterations monotonically decrease the objective but the problem is
  nonconvex and can end in local minima. Mitigation: multiple starts or
  K-means++ [UL, p. 61].
- Complexity is \(O(NKpJ)\) for \(J\) iterations; mini-batch and sparse
  variants address large \(N\) or \(p\) [UL, p. 62].
- K-means is sensitive to outliers and Euclidean cluster geometry. Preprocessing,
  K-medians, feature mappings, or spectral clustering are named alternatives
  [UL, pp. 62-63].

# Part C: boundaries, caveats, and source corrections

## 20. Explicitly extra, deferred, or not covered

### Extra/supporting material

- Detailed matrix/eigendecomposition proofs for the sample-variance chi-square
  law [STAT, pp. 122-124] and regression residual variance [STAT,
  pp. 183-185] are presented for information.
- The simple-regression prediction-interval proof is deferred to an exercise
  session [STAT, p. 198].
- Curse of dimensionality is explicitly extra [SL-3, pp. 30-34].
- Eigenfaces/randomized PCA example is explicitly extra [UL, pp. 33-34].

### Named but not developed

- Deep neural networks, semi-supervised, self-supervised, and reinforcement
  learning [DS-I, p. 13].
- LDA, QDA, SVM, and kernel SVM [SL-2, p. 22].
- Hierarchical and density-based clustering [UL, p. 40].
- Partial autocorrelation for AR order selection [STAT, p. 220; SUPP, p. 15].
- Welch's unequal-variance two-sample t-test is absent; the taught two-sample
  unknown-variance test assumes equal variances [STAT, pp. 158-161].
- The statistics table of contents mentions jump-diffusion estimation, but no
  corresponding section appears in `STAT` [STAT, p. 2]. It should not be assumed
  covered from the available material.

## 21. Important modeling assumptions students have seen

- Independence/i.i.d. sampling is required in many likelihood, CLT, bootstrap,
  and testing derivations.
- Normal-population assumptions underlie exact Student, chi-square, Fisher,
  regression, and ANOVA inference.
- The pooled two-sample Student procedure assumes equal variances.
- Linear regression's closed-form inverse assumes full column rank; LS remains
  usable without Gaussian errors, but its exact MLE and finite-sample inference
  interpretations change [STAT, pp. 172-174].
- Logistic likelihood assumes independent conditional Bernoulli outcomes
  [SL-2, p. 33].
- Random train/validation splitting presumes exchangeable observations; naive
  randomization is not automatically valid for dependent/time-ordered data even
  though the introductory CV discussion uses randomization [SL-2, p. 9].
- PCA is scale-sensitive and ordinarily needs centering; standardization is a
  modeling decision, not a universal requirement [UL, p. 23].
- k-NN and K-means depend critically on distance, feature scaling, dimension,
  and geometry [SL-1, p. 34; UL, pp. 62-63].

## 22. Errata affecting substantive formulas

The supplied errata page corrects three formulas [ERR, p. 1]:

1. The conditional-density example associated with `STAT`, p. 66 must use the
   corrected marginal density in its denominator.
2. The Gaussian-process posterior mean associated with `STAT`, p. 227 must
   include the cross-covariance vector and noisy Gram matrix:
   \(k(x_*,X)^T[K+\sigma_\epsilon^2I]^{-1}y\).
3. The noisy GP observation distribution associated with `STAT`, p. 232 is
   \(N(0,K+\sigma_\epsilon^2I)\).

Other apparent slide issues relevant when reusing results:

- A linear combination is guaranteed normal from **joint** normality, not merely
  from normal marginals [STAT, p. 49; APP, p. 22].
- The multivariate-normal density uses \(\Sigma^{-1}\) in its quadratic form
  [STAT, p. 76; APP, p. 36].
- The p-value is not the probability that the null hypothesis is true [STAT,
  p. 152].
- PCA itself is a linear projection after fitting, despite a takeaway slide
  describing it as nonlinear [UL, p. 35]. The learned axes depend on the data,
  but the forward map is linear for centered data.

## 23. Formula-sheet evidence of expected mastery

The two-page statistics formula sheet [FORM, pp. 1-2] includes:

- expectation/covariance/variance identities, normal combinations, CLT, gamma
  function, MGFs, and Student/Fisher constructions;
- one- and two-population sampling pivots and test rejection rules;
- pooled variance and degrees of freedom;
- simple and multiple OLS estimators and sampling distributions;
- SST/SSR/SSE matrix formulas, global regression F-test, coefficient t-tests,
  confidence intervals, and prediction intervals.

This indicates that a dependent course may expect students to recognize the
appropriate assumptions, statistic, reference law, degrees of freedom, tail,
and interpretation. The sheet supports formula recall; it does not replace the
method-selection knowledge developed in the lectures.

## 24. Practical software exposure

The slides explicitly use or reference:

- **NumPy:** arrays, means, standard deviations, quantiles, covariance,
  correlation, and numerical operations [STAT, pp. 30, 36, 40, 45].
- **pandas:** CSV import, summaries, and categorical dummy encoding [STAT,
  pp. 25, 30, 202].
- **Matplotlib:** histograms, boxplots, scatter/correlation views, and diagnostic
  plots [STAT, pp. 34-35, 41, 45].
- **SciPy:** distribution PDF/CDF/quantile/sampling methods, regression, and
  hypothesis tests [STAT, pp. 48, 55, 58, 110, 146, 150, 161, 164-165, 176,
  197].
- **Statsmodels:** OLS summaries and autoregression [STAT, pp. 187, 195-197,
  213-214].
- **scikit-learn:** train/test splitting, estimator `fit`/`predict`, linear and
  logistic regression, k-NN, CV scoring/prediction, confusion matrices, PCA,
  K-means, and Gaussian processes [SL-1, pp. 22, 29, 43; SL-2, pp. 35-42;
  UL, pp. 26-28, 59; STAT, p. 232].

The expected level is introductory scientific programming and correct workflow,
not software engineering or implementation of all algorithms from first
principles. However, exam questions may ask students to diagnose or correct
code/pseudocode [DS-I, p. 23].
