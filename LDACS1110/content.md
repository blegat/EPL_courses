# LDACS1110 Foundations of Learning: candidate topics and results

This document is a design inventory for the machine-learning part of
**LDACS1110 Foundations of Machine Learning and Cryptography**. It lists the
plausible topics and theorem-level results suggested by the official course
description, identifies what can already be assumed from LEPL1109, and records
which material should be taught before the cryptography part can reuse it.

It is deliberately broader than a final syllabus. The FoL part has about 15
contact hours, so the complete catalogue cannot be taught in one edition.

## Source documents and constraints

- The course has 30 hours of lectures and 30 hours of practical work, is worth
  5 credits, and is taught in English
  [[course template, p. 1](../Template_fiche%20et%20cahier%20des%20charges.pdf#page=1)].
- The first 15 hours are FoL for engineering students and precede the FoC part
  [[FoLC proposal, p. 1](../FoLC.pdf#page=1)].
- Prerequisites are LEPL1101 Linear Algebra, LEPL1109 Statistics and Data
  Science, and LEPL1402 Computer Science II; LEPL1108 Discrete Mathematics and
  Probability is prior knowledge
  [[course template, p. 1](../Template_fiche%20et%20cahier%20des%20charges.pdf#page=1)].
- The prescribed FoL headings are concentration, Monte Carlo and sampling,
  randomness and pseudorandomness, information theory, Bayesian inference and
  causality, generalization/PAC/sample complexity/compression/VC dimension,
  Gaussian-process regression, and applications
  [[FoLC proposal, pp. 1-2](../FoLC.pdf#page=1);
  [course template, p. 2](../Template_fiche%20et%20cahier%20des%20charges.pdf#page=2)].
- The intended learning outcomes emphasize convergence guarantees, the links
  between information/Bayesian uncertainty and learning, and the roles of
  bias-variance, regularization, generalization, evaluation, and robustness
  [[course template, pp. 1-2](../Template_fiche%20et%20cahier%20des%20charges.pdf#page=1)].

## Notation used below

- **Core**: strong candidate for the 15-hour examinable spine.
- **Bridge**: should precede FoC because cryptography can directly reuse it.
- **Optional**: valuable if time permits or for practical sessions.
- **Defer**: too large, too advanced, redundant with LEPL1109, or weakly related
  to the course outcomes.
- **LEPL1109 dependency** links point to the detailed prerequisite inventory:
  [`LEPL1109/content.md`](../LEPL1109/content.md).
- PDF references after those links are physical pages of the LEPL1109 source
  named in that inventory.

## Recommended 15-hour spine

The spine below is the most coherent theorem-oriented selection, but it requires
an approved narrowing of the official list: MCMC, Fano, causality, and GP are
represented only by applications, short statements, or optional practical
work, and only one of VC dimension and compression is developed. A second,
coverage-complete survey option follows the table.

| Block | Hours | Principal outcomes |
|---|---:|---|
| Randomized algorithms, Monte Carlo, concentration | 3.0 | Convert expectation estimates into finite-sample `(epsilon, delta)` guarantees |
| Entropy, KL, mutual information, statistical distance | 2.5 | Quantify uncertainty, dependence, leakage, and distinguishability |
| Bayesian inference and regularization | 2.0 | Derive posterior prediction and connect MAP to penalized learning |
| PAC learning and finite-class generalization | 2.5 | Derive sample complexity from concentration and a union bound |
| VC dimension **or** sample compression | 1.5 | Extend generalization beyond finite hypothesis classes |
| Universal hashing, min-entropy, leftover hashing | 2.0 | Turn weak randomness into nearly uniform bits and prepare FoC |
| Integrated applications and FoC handoff | 1.5 | Relate existing LEPL1109 models to the new theory and distinguish randomness notions |
| **Total** | **15.0** | |

MCMC, Fano's inequality, Gaussian processes, and causality remain in the
official topic pool, but a rigorous treatment of all four is incompatible with
this spine. Suggested ways to include them are given below.

### Coverage-complete 15-hour survey alternative

| Block                                                 |    Hours | Depth                                                                         |
| ----------------------------------------------------- | -------: | ----------------------------------------------------------------------------- |
| Randomized algorithms, Monte Carlo, concentration     |      2.5 | Prove Markov/Chernoff method/Hoeffding; state birthday and amplification      |
| **Metropolis-Hastings and Gibbs**                     |      1.0 | Derive detailed balance; state ergodic convergence; practical demonstration   |
| Entropy, KL, mutual information, statistical distance |      2.0 | Prove elementary identities and data processing selectively                   |
| Fano and information-theoretic lower bounds           |      0.5 | State and apply once                                                          |
| Bayesian inference, MAP, and regularization           |      1.5 | One conjugate model and Gaussian-prior/ridge connection                       |
| Causality                                             |      0.5 | Association/intervention distinction and one confounding example              |
| PAC, sample complexity, VC dimension, compression     |      3.0 | Prove finite-class bound; state Sauer-Shelah/VC theorem and compression bound |
| GP regression                                         |      0.5 | Bayesian reinterpretation of LEPL1109, no repeated derivation                 |
| Universal hashing, min-entropy, leftover hashing      |      2.0 | Prove collision facts; state or sketch LHL                                    |
| Applications and FoC handoff                          |      1.5 | Robustness, randomness hierarchy, and hybrid argument                         |
| **Total**                                             | **15.0** |                                                                               |

This alternative touches every prescribed heading but necessarily treats
several results as statement-only. Detailed derivations, coding experiments,
and applications should use the course's practical hours rather than being
added to the 15 lecture hours.

# Candidate topic catalogue

## 1. Randomized algorithms and probabilistic analysis

**Status:** Core, Bridge.

### Concepts

- Probability space attached to an algorithm's internal random choices.
- Random seed and random bits; deterministic behavior conditional on the seed.
- Monte Carlo algorithms: bounded running time with a controlled probability of
  an incorrect answer.
- Las Vegas algorithms: always correct, with random running time.
- One-sided versus two-sided error.
- Expected running time, failure probability, and adversarial versus average
  inputs.
- Failure-probability amplification by independent repetition and majority vote.
- Indicator variables and linearity of expectation for algorithm analysis.
- Universal hashing as the principal randomized-data-structure example.

### Candidate results

**Union bound.** For events `A_1, ..., A_m`,

\[
P\left(\bigcup_{i=1}^m A_i\right)\leq \sum_{i=1}^m P(A_i).
\]

**Amplification.** If independent runs fail with probability `p < 1/2`, a
majority of `r` runs has exponentially decreasing failure probability; a
Hoeffding or Chernoff bound makes this quantitative.

**Linearity of expectation.** Independence is unnecessary for
`E[sum_i X_i] = sum_i E[X_i]`, which supports collision, occupancy, and running
time analyses.

**Birthday bound.** For `q` independent uniform samples from a set of size `N`,
the collision probability satisfies

\[
P(\text{collision})
=1-\prod_{i=0}^{q-1}\left(1-\frac{i}{N}\right)
\leq \frac{q(q-1)}{2N}.
\]

The transition from unlikely to likely collision occurs around `q = Theta(sqrt(N))`.

### LEPL1109 dependency

- Random variables, expectation, indicators, and variance:
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables)
  [STAT, pp. 7-18; SL-1, pp. 4-7 for indicator notation].
- Independence and covariance:
  [dependence and multivariate probability](../LEPL1109/LEPL1109_course_content.md#3-dependence-and-multivariate-probability)
  [STAT, pp. 38-45].
- Numerical pseudorandom generators and seeds:
  [simulation and bootstrap](../LEPL1109/LEPL1109_course_content.md#6-simulation-and-bootstrap)
  [STAT, pp. 107-110].
- Algorithm/data-structure prerequisites mainly come from LEPL1402, not
  LEPL1109.

### What is new beyond LEPL1109

- Algorithms as probability experiments.
- Explicit failure probabilities and amplification.
- Randomness as a computational resource.
- Birthday-scale collision behavior.

### FoC reuse

- Security games, probabilistic adversaries, and reduction success probability.
- Hash collisions and random-oracle query bounds.
- PRF/PRP switching bounds and block-cipher analysis.
- Amplification and accounting for multiple bad events.

### Bibliography

[B1, Chs. 5 and 11](#b1), [B2, Chs. 3-5](#b2), [B11](#b11).

## 2. Monte Carlo estimation

**Status:** Core.

### Concepts

- Express a target quantity as an expectation `mu = E_P[f(X)]`.
- Draw independent samples and use
  `hat(mu)_n = n^{-1} sum_i f(X_i)`.
- Separate sampling error, model error, approximation error, and floating-point
  error.
- Estimate integrals, probabilities, risks, and expected algorithmic costs.
- Confidence based on asymptotic normality versus finite-sample concentration.
- Variance reduction as the central determinant of Monte Carlo efficiency.

### Candidate results

**Unbiasedness and variance.** For i.i.d. samples with finite variance,

\[
E[\widehat\mu_n]=\mu,
\qquad
\operatorname{Var}(\widehat\mu_n)
=\frac{\operatorname{Var}(f(X))}{n}.
\]

**Weak consistency.** The law of large numbers yields
`hat(mu)_n -> mu` in probability.

**CLT uncertainty.** Under a finite-variance assumption,

\[
\sqrt n\,\frac{\widehat\mu_n-\mu}{\sigma}
\Rightarrow N(0,1).
\]

**Root-n rate.** Halving the standard Monte Carlo error requires approximately
four times as many independent samples.

**Control variate, optional.** If `E[g(X)]` is known, then

\[
\widehat\mu_c
=\frac1n\sum_i\bigl(f(X_i)-c(g(X_i)-E[g(X)])\bigr)
\]

is unbiased, with an optimal coefficient determined by covariance.

### LEPL1109 dependency

- LLN, CLT, means, and variance of averages:
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables)
  and [normal approximations](../LEPL1109/LEPL1109_course_content.md#4-normal-approximations-and-reference-laws)
  [STAT, pp. 13-18, 50-52].
- Simulation and inverse transforms:
  [simulation and bootstrap](../LEPL1109/LEPL1109_course_content.md#6-simulation-and-bootstrap)
  [STAT, pp. 107-117].
- Risk and empirical risk:
  [statistical decision theory](../LEPL1109/LEPL1109_course_content.md#17-statistical-decision-theory-and-bayes-optimality)
  [SL-3, pp. 5-6].

### What is new beyond LEPL1109

- Monte Carlo as a general algorithmic design pattern.
- Explicit computational accuracy/cost tradeoffs.
- Finite-sample guarantees supplied by the next section.

### FoC reuse

- Estimation of adversarial success rates in experiments.
- Correct warning: empirical testing can reveal failures but cannot establish
  computational security against all efficient adversaries.

### Bibliography

[B3, Chs. 3-4](#b3), [B4, Chs. 10-11](#b4), [B2](#b2).

## 3. Concentration inequalities

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
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables)
  [STAT, pp. 13-18; FORM, p. 1].
- Bernoulli and binomial laws:
  [assumed distribution family](../LEPL1109/LEPL1109_course_content.md#assumed-distribution-family)
  [APP, pp. 5-9].
- CLT, to contrast asymptotic and nonasymptotic statements:
  [normal approximations](../LEPL1109/LEPL1109_course_content.md#4-normal-approximations-and-reference-laws)
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

[B2](#b2), [B12, Chs. 2-3](#b12), [B7, Parts I-II](#b7).

## 4. Direct and weighted sampling methods

**Status:** Optional.

### Candidate results

**Inverse-transform sampling.** Already covered by LEPL1109; use only as a
starting point.

**Rejection sampling.** If `p(x) <= M q(x)`, draw `X ~ q` and accept it with
probability `p(X)/(M q(X))`. The accepted sample has distribution `p`; the mean
acceptance probability is `1/M` when densities are normalized.

**Importance-sampling identity.** If `p` is absolutely continuous with respect
to `q`,

\[
E_p[f(X)]
=E_q\left[f(X)\frac{p(X)}{q(X)}\right].
\]

This gives an unbiased estimator when the normalizing constants and support
conditions permit it.

**Self-normalized importance sampling.** Useful when only unnormalized target
weights are known, but generally biased at finite sample size.

**Importance-weight variance.** A poor proposal with heavy or highly variable
weights can make the estimator unusable; effective sample size is only a
diagnostic approximation.

### LEPL1109 dependency

- Density transformations and common distributions:
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables)
  [STAT, pp. 7-23; APP, pp. 4-37].
- Inverse-transform simulation:
  [simulation and bootstrap](../LEPL1109/LEPL1109_course_content.md#6-simulation-and-bootstrap)
  [STAT, pp. 107-110].

### FoC reuse

Low direct value for the listed cryptography syllabus.

### Bibliography

[B3, Chs. 3-4](#b3), [B4](#b4).

## 5. Markov-chain Monte Carlo

**Status:** Optional; Defer rigorous mixing theory.

### Concepts

- Sampling from a target distribution known up to a normalizing constant.
- Markov transition kernel, stationary distribution, irreducibility,
  aperiodicity, recurrence, and ergodic averages.
- Burn-in, autocorrelation, effective sample size, and limitations of visual
  convergence diagnostics.
- Metropolis-Hastings proposal and accept/reject correction.
- Gibbs sampling from full conditional distributions.

### Candidate results

**Detailed balance implies stationarity.** If

\[
\pi(x)P(x,y)=\pi(y)P(y,x)
\]

for all states, then `pi` is stationary for `P`.

**Metropolis-Hastings acceptance probability.** For proposal `q(y|x)`,

\[
\alpha(x,y)=
\min\left\{1,
\frac{\pi(y)q(x\mid y)}{\pi(x)q(y\mid x)}
\right\}.
\]

The resulting transition satisfies detailed balance with the target under the
usual support conditions.

**Gibbs invariance.** Updating one coordinate from its full conditional leaves
the joint target distribution invariant.

**Ergodic theorem, stated with assumptions.** Under appropriate irreducibility,
aperiodicity, and positive-recurrence conditions,

\[
\frac1n\sum_{t=1}^n f(X_t)\to E_\pi[f(X)]
\]

almost surely for integrable `f`.

**MCMC variance warning.** Dependent draws do not have variance
`sigma^2/n`; integrated autocorrelation inflates Monte Carlo variance.

### LEPL1109 dependency

- Conditional and multivariate distributions:
  [dependence and multivariate probability](../LEPL1109/LEPL1109_course_content.md#3-dependence-and-multivariate-probability)
  [STAT, pp. 60-81].
- Time-series autocorrelation:
  [time series and autoregressive models](../LEPL1109/LEPL1109_course_content.md#10-time-series-and-autoregressive-models)
  [STAT, pp. 207-220].
- Bayesian posterior distributions are new and must precede MCMC.

### What is new beyond LEPL1109

- Markov-chain stationarity and detailed balance.
- Dependent sampling and ergodic convergence.
- Sampling from unnormalized posterior distributions.

### FoC reuse

Limited for the prescribed FoC topics. Random walks and sampling appear in
specialized cryptography, but MCMC should not displace the stronger shared
foundations of entropy, statistical distance, and hashing.

### Bibliography

[B3, Chs. 7-9](#b3), [B4, Chs. 10-12](#b4).

## 6. Shannon entropy and conditional entropy

**Status:** Core, Bridge.

### Concepts

- Self-information `-log p(x)`.
- Shannon entropy in bits when logarithms have base 2.
- Joint entropy, conditional entropy, and entropy rate only if needed.
- Entropy as average uncertainty, distinct from variance and from worst-case
  unpredictability.
- Compression as an operational motivation; coding theorems can be stated but
  need not be proved.

### Candidate results

**Shannon entropy.** For a discrete random variable,

\[
H(X)=-\sum_x p(x)\log p(x).
\]

**Bounds and equality cases.** If `X` takes values in a finite alphabet,

\[
0\leq H(X)\leq\log|\mathcal X|,
\]

with maximum entropy at the uniform distribution.

**Chain rule.**

\[
H(X,Y)=H(X)+H(Y\mid X).
\]

**Conditioning reduces entropy.**

\[
H(X\mid Y)\leq H(X).
\]

**Independent additivity.** If `X` and `Y` are independent,
`H(X,Y)=H(X)+H(Y)`.

**Entropy of a function.** For deterministic `g`,
`H(g(X)) <= H(X)`.

### LEPL1109 dependency

- Discrete distributions and expectation of functions:
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables)
  [STAT, pp. 7-15].
- Joint and conditional distributions:
  [random vectors and conditioning](../LEPL1109/LEPL1109_course_content.md#random-vectors-and-conditioning)
  [STAT, pp. 60-71].
- Logistic cross-entropy is known as a loss, but entropy theory is not:
  [logistic regression](../LEPL1109/LEPL1109_course_content.md#logistic-regression)
  [SL-2, pp. 33-35].

### What is new beyond LEPL1109

- Entropy and its chain rules.
- Quantitative uncertainty accounting.
- Operational link to compression and information.

### FoC reuse

- Perfect secrecy and key uncertainty.
- Leakage and information-theoretic security.
- Impossibility and key-length lower bounds.

### Bibliography

[B5, Ch. 2](#b5), [B6, Parts I-II](#b6).

## 7. KL divergence, cross-entropy, and mutual information

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

\[
D_{\mathrm{KL}}(P\|Q)
=\sum_x P(x)\log\frac{P(x)}{Q(x)}.
\]

**Gibbs inequality.** `D_KL(P||Q) >= 0`, with equality exactly when the
distributions agree on the relevant support.

**Cross-entropy decomposition.**

\[
H(P,Q)=H(P)+D_{\mathrm{KL}}(P\|Q).
\]

**Mutual information identities.**

\[
I(X;Y)
=D_{\mathrm{KL}}(P_{XY}\|P_XP_Y)
=H(X)-H(X\mid Y)
=H(Y)-H(Y\mid X).
\]

Consequently, `I(X;Y) >= 0`, and `I(X;Y)=0` exactly when `X` and `Y` are
independent.

**Chain rule for mutual information.**

\[
I(X;Y,Z)=I(X;Y)+I(X;Z\mid Y).
\]

**Data-processing inequality.** If `X -> Y -> Z`, then

\[
I(X;Z)\leq I(X;Y).
\]

**Pinsker inequality, recommended bridge.** Under natural logarithms,

\[
\Delta(P,Q)\leq\sqrt{\frac12D_{\mathrm{KL}}(P\|Q)}.
\]

This connects information divergence to statistical distinguishing advantage.

### LEPL1109 dependency

- Joint distributions, independence, conditioning, and likelihood:
  [dependence and multivariate probability](../LEPL1109/LEPL1109_course_content.md#3-dependence-and-multivariate-probability)
  and [maximum likelihood](../LEPL1109/LEPL1109_course_content.md#maximum-likelihood)
  [STAT, pp. 38, 60-81, 97-105].
- Logistic negative log-likelihood/cross-entropy:
  [logistic regression](../LEPL1109/LEPL1109_course_content.md#logistic-regression)
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

[B5, Chs. 2 and 11](#b5), [B6](#b6), [B7](#b7).

## 8. Statistical distance and couplings

**Status:** Core, Bridge.

### Concepts

- Statistical/total-variation distance between discrete distributions.
- Event and test characterizations.
- Statistical versus computational indistinguishability.
- Coupling as a joint construction with prescribed marginals.
- Data processing under randomized mappings.

### Candidate results

**Total-variation distance.**

\[
\Delta(P,Q)=\frac12\sum_x|P(x)-Q(x)|.
\]

**Event characterization.**

\[
\Delta(P,Q)=\max_A|P(A)-Q(A)|.
\]

**Distinguisher characterization.** For any test `T` with output in `{0,1}`,

\[
|P(T(X)=1)-P(T(Y)=1)|\leq\Delta(P,Q),
\]

and an optimal unbounded test attains equality.

**Data processing.** For any randomized mapping `K`,

\[
\Delta(KP,KQ)\leq\Delta(P,Q).
\]

**Triangle inequality.** This supports hybrid/game-hopping arguments.

**Coupling lemma.** For every coupling `(X,Y)`,
`Delta(P_X,P_Y) <= P(X != Y)`, and an optimal coupling attains equality.

### LEPL1109 dependency

- Discrete distributions, events, and conditional probability:
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables)
  [STAT, pp. 7-15].
- Classification tests provide useful intuition but total variation is new.

### What is new beyond LEPL1109

- A metric on distributions with operational test meaning.
- Statistical indistinguishability and data processing.
- Coupling and hybrids as proof tools.

### FoC reuse

- Real-versus-ideal definitions of information-theoretic security.
- Statistical secrecy, extractors, and privacy amplification.
- Game hops and distinguishing advantage.

### Bibliography

[B11, Chs. 3 and 6](#b11), [B17, probability appendix](#b17).

## 9. Fano's inequality and information-theoretic lower bounds

**Status:** Optional; Bridge if FoC develops impossibility results through
information theory.

### Candidate results

**Fano inequality.** Let `X` take `M` values and let `hat(X)(Y)` estimate it with
error probability `P_e`. Then

\[
H(X\mid Y)\leq h_2(P_e)+P_e\log(M-1).
\]

For uniform `X`, a common consequence is

\[
P_e\geq1-\frac{I(X;Y)+\log 2}{\log M},
\]

with constants adjusted to the logarithm convention.

**Lower-bound template.** Select a finite set of well-separated hypotheses,
bound the information conveyed by observations, then use Fano to lower-bound
the probability of identification error.

### LEPL1109 dependency

- Conditional probability, classification error, and Bayes risk:
  [statistical decision theory](../LEPL1109/LEPL1109_course_content.md#17-statistical-decision-theory-and-bayes-optimality)
  [SL-3, pp. 7-19].
- Entropy and mutual information must first be taught in FoL.

### What is new beyond LEPL1109

- Information-theoretic impossibility and minimax lower-bound methodology.

### FoC reuse

- Reliability/secrecy tradeoffs and information-theoretic impossibility.
- Optional: FoC may instead prove Shannon-style impossibility directly, making
  Fano unnecessary in the 15-hour core.

### Bibliography

[B5, Ch. 2](#b5), [B7](#b7).

## 10. Bayesian inference

**Status:** Core.

### Concepts

- Unknown parameter as a random variable.
- Prior, likelihood, evidence/marginal likelihood, and posterior.
- Sequential updating under conditional independence.
- Posterior summaries: mean, variance, credible intervals, and MAP.
- Posterior predictive distribution and Bayesian decision rules.
- Conjugacy as a tractable example, not as a general requirement.
- Prior sensitivity and distinction between epistemic and observation
  uncertainty.
- Credible intervals versus frequentist confidence intervals.

### Candidate results

**Bayes formula for parameters.**

\[
p(\theta\mid x)
=\frac{p(x\mid\theta)p(\theta)}{p(x)},
\qquad
p(x)=\int p(x\mid\theta)p(\theta)d\theta.
\]

**Posterior prediction.**

\[
p(y_{\mathrm{new}}\mid x)
=\int p(y_{\mathrm{new}}\mid\theta,x)
p(\theta\mid x)d\theta.
\]

This reduces to `integral p(y_new|theta) p(theta|x) dtheta` when the new
observation is conditionally independent of the observed data given `theta`.

**Beta-Bernoulli conjugacy.** If
`theta ~ Beta(alpha,beta)` and `s` successes occur in `n` Bernoulli trials,

\[
\theta\mid x_{1:n}
\sim\operatorname{Beta}(\alpha+s,\beta+n-s).
\]

The posterior predictive success probability is

\[
P(X_{n+1}=1\mid x_{1:n})
=\frac{\alpha+s}{\alpha+\beta+n}.
\]

**Bayes actions.** Posterior mean minimizes posterior expected squared loss;
posterior median minimizes absolute loss; posterior mode is a MAP point estimate
under regularity/uniqueness qualifications.

### LEPL1109 dependency

- Conditional distributions and Bayes' rule:
  [random vectors and conditioning](../LEPL1109/LEPL1109_course_content.md#random-vectors-and-conditioning)
  [STAT, pp. 60-71].
- Parametric models, likelihood, and MLE:
  [parametric estimation](../LEPL1109/LEPL1109_course_content.md#5-parametric-estimation)
  [STAT, pp. 83-105].
- Bayes predictor and conditional risk:
  [statistical decision theory](../LEPL1109/LEPL1109_course_content.md#17-statistical-decision-theory-and-bayes-optimality)
  [SL-3, pp. 7-19].
- Confidence intervals:
  [sampling distributions](../LEPL1109/LEPL1109_course_content.md#7-sampling-distributions-and-confidence-intervals)
  [STAT, pp. 119-136].

### What is new beyond LEPL1109

LEPL1109's Bayes predictor assumes a known joint population distribution. FoL
adds uncertainty over unknown parameters, prior/posterior updating, credible
sets, and posterior prediction.

### FoC reuse

- Adversarial inference and updating beliefs from observations.
- Important boundary: cryptographic security must not rely on a favorable
  subjective prior over adversaries.

### Bibliography

[B4, Chs. 1-5](#b4), [B6](#b6), [B8](#b8).

## 11. MAP estimation and regularization

**Status:** Core.

### Candidate results

**MAP objective.**

\[
\widehat\theta_{MAP}
=\arg\min_\theta
\{-\log p(x\mid\theta)-\log p(\theta)\}.
\]

**Gaussian prior to ridge penalty.** If
`theta ~ N(0,tau^2 I)`, then the negative log-prior contributes
`||theta||_2^2/(2 tau^2)`.

**Laplace prior to lasso penalty.** An independent Laplace prior contributes an
`L1` penalty proportional to `||theta||_1`.

**Gaussian linear model.** Gaussian likelihood plus Gaussian prior yields a
quadratic posterior and a regularized least-squares posterior mean/MAP.

**MAP limitation.** MAP is not invariant under arbitrary reparameterization and
does not retain posterior uncertainty; it should not be equated with full
Bayesian prediction.

### LEPL1109 dependency

- MLE and negative log-likelihood:
  [maximum likelihood](../LEPL1109/LEPL1109_course_content.md#maximum-likelihood)
  [STAT, pp. 97-105].
- OLS and model flexibility:
  [linear regression and ANOVA](../LEPL1109/LEPL1109_course_content.md#9-linear-regression-and-anova)
  and [supervised-learning formulation](../LEPL1109/LEPL1109_course_content.md#13-supervised-learning-formulation).
- Bias-variance tradeoff:
  [bias-variance](../LEPL1109/LEPL1109_course_content.md#18-bias-variance-tradeoff-and-dimensionality)
  [SL-3, pp. 22-29].

### What is new beyond LEPL1109

- Prior-penalty equivalence and an explicit probabilistic view of
  regularization.
- Distinction between posterior uncertainty, MAP, and regularized ERM.

### FoC reuse

Low direct dependence, but this topic is central to the official learning
outcome linking uncertainty, regularization, and generalization.

### Bibliography

[B6](#b6), [B8](#b8), [B9](#b9).

## 12. Causal inference

**Status:** Optional; Defer a complete treatment.

### Concepts

- Association, prediction, and causation are different questions.
- Confounders, colliders, mediators, and causal DAGs.
- Observational conditioning `P(Y|X=x)` versus intervention
  `P(Y|do(X=x))`.
- Randomized experiments and exchangeability.
- Identifiability depends on causal assumptions not recoverable from the joint
  observational distribution alone.
- Distribution shift can invalidate associational predictors.

### Candidate results

**Back-door adjustment.** Under the back-door criterion for an adjustment set
`Z`,

\[
P(Y\mid do(X=x))
=\sum_z P(Y\mid X=x,Z=z)P(Z=z).
\]

**Randomization.** In an ideal randomized experiment, treatment assignment is
independent of potential outcomes, identifying average treatment effects from
group contrasts under consistency and positivity assumptions.

**Observational non-identifiability.** Different causal graphs can induce the
same observational distribution while predicting different intervention
effects.

### LEPL1109 dependency

- Conditional distributions and regression:
  [dependence and multivariate probability](../LEPL1109/LEPL1109_course_content.md#3-dependence-and-multivariate-probability)
  and [linear regression](../LEPL1109/LEPL1109_course_content.md#9-linear-regression-and-anova).
- Correlation captures linear association, not causation:
  [independence covariance correlation](../LEPL1109/LEPL1109_course_content.md#independence-covariance-and-correlation)
  [STAT, pp. 38-45].

### What is new beyond LEPL1109

- Intervention semantics, causal assumptions, DAGs, and identification.

### FoC reuse

Negligible for the listed cryptography topics. A single motivating example is
realistic; a responsible causal-inference module requires more time.

### Bibliography

[B10, Chs. 9-10](#b10), [B13](#b13), [B14](#b14).

## 13. PAC learning and finite-class sample complexity

**Status:** Core.

### Concepts

- Instance space, label space, hypothesis class, learner, and unknown data
  distribution.
- True risk `R(h)` and empirical risk `hat(R)_S(h)`.
- Accuracy `epsilon`, confidence `1-delta`, and sample complexity.
- Realizable versus agnostic learning.
- Empirical-risk minimization and consistent learning.
- Pointwise concentration versus uniform convergence over a model class.
- Approximation error versus estimation error.

### Candidate results

**Realizable PAC definition.** A class is PAC learnable if there is a sample
complexity `m_H(epsilon,delta)` such that, whenever the labels are generated by
some `h* in H`, the learner returns, with probability at least `1-delta`, a
hypothesis satisfying `R(h) <= epsilon` from at least that many i.i.d. samples.

**Agnostic PAC definition.** Without realizability, the guarantee becomes

\[
R(h)\leq\inf_{g\in\mathcal H}R(g)+\varepsilon
\]

with probability at least `1-delta`.

**Efficient PAC learning.** Computational efficiency is an additional
requirement: sample size and running time should be polynomial in
`1/epsilon`, `log(1/delta)`, and an appropriate representation-size parameter.
It is not part of purely statistical learnability.

**Finite-class uniform convergence.** For binary or bounded loss, Hoeffding plus
a union bound gives

\[
P\left(
\sup_{h\in\mathcal H}
|\widehat R_S(h)-R(h)|>\varepsilon
\right)
\leq 2|\mathcal H|e^{-2m\varepsilon^2}.
\]

**Agnostic ERM guarantee.** With probability at least `1-delta`,

\[
R(\widehat h)
\leq \inf_{h\in\mathcal H}R(h)
+2\sqrt{\frac{\log(2|\mathcal H|/\delta)}{2m}},
\]

up to the exact convention used in the intermediate uniform bound.

**Agnostic sample complexity.** A representative scaling is

\[
m=O\left(
\frac{\log|\mathcal H|+\log(1/\delta)}{\varepsilon^2}
\right).
\]

**Realizable finite-class bound.** A consistent learner obtains the sharper
scaling

\[
m=O\left(
\frac{\log|\mathcal H|+\log(1/\delta)}{\varepsilon}
\right).
\]

**Occam interpretation.** Smaller describable hypothesis classes require fewer
samples, but only relative to the actual representation/modeling assumptions.

### LEPL1109 dependency

- Function classes, parameters/hyperparameters, training error, and overfitting:
  [supervised-learning formulation](../LEPL1109/LEPL1109_course_content.md#13-supervised-learning-formulation)
  [SL-1, pp. 38-47].
- Generalization error and CV:
  [resampling](../LEPL1109/LEPL1109_course_content.md#15-resampling-model-assessment-and-model-selection)
  [SL-2, pp. 3-19].
- Expected/empirical risk and excess risk:
  [statistical decision theory](../LEPL1109/LEPL1109_course_content.md#17-statistical-decision-theory-and-bayes-optimality)
  [SL-3, pp. 5-16].
- Bias-variance:
  [bias-variance](../LEPL1109/LEPL1109_course_content.md#18-bias-variance-tradeoff-and-dimensionality)
  [SL-3, pp. 22-29].

### What is new beyond LEPL1109

- Formal distribution-free learnability.
- Uniform convergence and confidence-qualified guarantees.
- Explicit sample complexity and model-class dependence.
- Separation of computational efficiency from statistical learnability.

### FoC reuse

- Quantified experiments, bad-event bounds, and union-bound proof patterns.
- Important boundary: PAC indistinguishability over sampled data is not
  cryptographic computational indistinguishability.

### Bibliography

[B7, Parts I-II](#b7), [B8, Chs. 2-6](#b8), [B10, Ch. 6](#b10).

## 14. VC dimension and growth functions

**Status:** Core candidate; choose this or sample compression if time is tight.

### Concepts

- Dichotomies, shattering, growth function, and VC dimension.
- Capacity is combinatorial and need not equal parameter count.
- Infinite classes can be learnable if their growth is controlled.

### Candidate results

**Examples.** Thresholds on the real line have VC dimension 1; intervals have
VC dimension 2; affine halfspaces in `R^d` have VC dimension `d+1`.

**Sauer-Shelah lemma.** If `VCdim(H)=d<m`,

\[
\Pi_{\mathcal H}(m)
\leq\sum_{i=0}^d\binom mi
\leq\left(\frac{em}{d}\right)^d.
\]

**VC generalization rate.** A representative uniform bound has order

\[
\sup_{h\in\mathcal H}|R(h)-\widehat R(h)|
=O\left(
\sqrt{\frac{d\log(m/d)+\log(1/\delta)}{m}}
\right).
\]

**Fundamental theorem, statement.** Under standard measurability conditions for
binary classification, finite VC dimension characterizes distribution-free PAC
learnability and uniform convergence, up to the distinctions made in the exact
version of the theorem.

### LEPL1109 dependency

- Linear decision boundaries and k-NN flexibility:
  [linear least squares and k-nearest neighbors](../LEPL1109/LEPL1109_course_content.md#14-linear-least-squares-and-k-nearest-neighbors)
  [SL-1, pp. 22-35].
- Model classes and overfitting:
  [supervised-learning formulation](../LEPL1109/LEPL1109_course_content.md#13-supervised-learning-formulation).

### What is new beyond LEPL1109

- Shattering, growth functions, and capacity of infinite classes.
- Capacity-dependent sample complexity.

### FoC reuse

Moderate methodological value through counting and uniform adversarial choices;
little direct dependency for the listed primitives.

### Bibliography

[B8, Chs. 5-7](#b8), [B7](#b7), [B15](#b15).

## 15. Sample compression and description length

**Status:** Optional alternative/complement to VC dimension.

### Concepts

- Compress a labeled sample to a small subset plus finite side information.
- Reconstruct a hypothesis from the compression.
- Generalization by counting the possible compressed descriptions.
- Difference between sample compression and PCA/data compression.

### Candidate results

**Compression generalization.** A consistent hypothesis reconstructed from `k`
sample points and bounded side information admits a realizable bound of the
representative form

\[
R(h)=O\left(\frac{k\log m+\log(1/\delta)}{m}\right),
\]

with exact constants and logarithmic terms depending on the compression-scheme
definition.

**Threshold example.** A consistent threshold can be reconstructed from at most
two extremal labeled examples.

**Description-length principle.** Shorter hypothesis descriptions reduce the
number of alternatives that must be controlled by the union bound.

### LEPL1109 dependency

- Dataset, generalization, and empirical risk:
  [supervised-learning formulation](../LEPL1109/LEPL1109_course_content.md#13-supervised-learning-formulation)
  and [statistical decision theory](../LEPL1109/LEPL1109_course_content.md#17-statistical-decision-theory-and-bayes-optimality).
- PCA compression is a distinct concept:
  [PCA](../LEPL1109/LEPL1109_course_content.md#principal-component-analysis).

### What is new beyond LEPL1109

- Generalization from reconstructibility and short combinatorial descriptions.

### FoC reuse

Moderate proof-pattern value through counting and encodings.

### Bibliography

[B15](#b15), [B8](#b8).

## 16. Further generalization frameworks

**Status:** Defer, but legitimate advanced candidates.

### Rademacher complexity

- Empirical Rademacher complexity as data-dependent richness.
- Symmetrization and contraction lemmas.
- Bounds for real-valued losses and norm-constrained linear predictors.
- Advantage: finer than cardinality/VC in many modern classes.

### Algorithmic stability

- Replace-one sensitivity of a learning algorithm.
- Uniform stability implies an expected/high-probability generalization bound.
- Strong convexity plus regularization can create stability.

### PAC-Bayes

- Prior and data-dependent posterior over predictors.
- Bounds involving empirical Gibbs risk and
  `D_KL(posterior || prior)`.
- Attractive synthesis of Bayesian notation and frequentist high-probability
  guarantees, but not ordinary Bayesian posterior inference.

### Structural risk minimization

- Nested classes and complexity penalties.
- Select a model by balancing empirical fit and a uniform-convergence penalty.

### Why defer

Each framework requires a separate proof toolkit. Including several would
prevent adequate treatment of entropy, Bayes, and the crypto bridge.

### LEPL1109 dependency

- Empirical/expected risk, optimization, model selection, and bias-variance:
  [Parts 13-18 of the LEPL1109 map](../LEPL1109/LEPL1109_course_content.md#part-b-data-science-and-machine-learning).

### Bibliography

[B7](#b7), [B8](#b8), [B9](#b9).

## 17. Gaussian-process regression as Bayesian learning

**Status:** Optional synthesis; do not reteach the LEPL1109 derivation.

### Already covered by LEPL1109

- GP prior as finite-dimensional jointly Gaussian function values.
- Mean and covariance kernel.
- Noisy observation model.
- Posterior predictive mean and variance.
- RBF, Matern, and rational-quadratic kernels.
- Marginal-likelihood hyperparameter fitting.

See [Gaussian-process regression in LEPL1109](../LEPL1109/LEPL1109_course_content.md#11-gaussian-process-regression)
[STAT, pp. 222-232; ERR, p. 1].

### Candidate new connections

**GP as a function prior.** Kernel choice expresses prior assumptions about
smoothness, scale, and structure.

**Posterior conditioning.** GP regression is a direct application of Bayesian
conditioning for a multivariate Gaussian model.

**Kernel ridge equivalence.** Under matching conventions, the GP posterior mean
at the training/prediction points agrees with kernel ridge regression, with the
noise/prior scale setting the regularization parameter.

**Marginal likelihood.** Hyperparameter fitting balances data fit and a
log-determinant complexity term, often called an Occam factor.

**Calibration caveat.** Posterior intervals are conditional on the kernel,
noise model, and hyperparameters; misspecification can invalidate uncertainty
claims.

### FoC reuse

Low. This is useful for the Bayesian/regularization learning outcome but should
not displace shared crypto foundations.

### Bibliography

[B16, Chs. 2, 4, and 5](#b16), [B6](#b6).

## 18. Universal hashing and randomized hash maps

**Status:** Core, Bridge.

### Concepts

- Family of hash functions with a uniformly random public choice of function.
- Universal and 2-universal collision guarantees.
- Pairwise independence versus full independence.
- Chaining and load factor in hash maps.
- Explicit separation among deterministic hashing, universal hashing,
  cryptographic collision resistance, random oracles, and PRFs.

### Candidate results

**Universal hashing.** A family `H` mapping `U` to `[m]` is universal if for
distinct `x,x'`,

\[
P_{h\leftarrow\mathcal H}[h(x)=h(x')]
\leq\frac1m.
\]

**Expected collisions.** For a fixed stored set and a fresh/randomly selected
universal hash function, linearity of expectation controls the expected number
of keys colliding with a query key.

**Expected lookup with chaining.** Under simple uniform/universal-hashing
assumptions, expected lookup cost is `O(1+alpha)` at load factor `alpha=n/m`.

**Pairwise-independent construction, candidate.** Over a finite field, affine
maps `h_{a,b}(x)=ax+b` with appropriate random parameters provide a simple
pairwise-independent family.

### LEPL1109 dependency

- Uniform discrete variables, indicators, independence, and expectation:
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables).
- Algorithmic data structures come from LEPL1402.

### What is new beyond LEPL1109

- Random function families and limited independence.
- Collision guarantees over the random function choice.
- Algorithmic use of weak randomness.

### FoC reuse

- Prepares keyed function families, universal-hash MACs, extraction, and the
  distinction from cryptographic hashes.
- FoC should own collision resistance, preimage resistance, random-oracle
  modeling, PRFs, and cryptographic hash constructions.

### Bibliography

[B1, Ch. 11](#b1), [B11, Ch. 6](#b11), [B17, universal hashing](#b17).

## 19. Min-entropy and randomness extraction

**Status:** Core, Bridge.

### Concepts

- Weak random source versus uniform distribution.
- Average uncertainty (Shannon entropy) versus maximum guessing probability
  (min-entropy).
- Seeded extractor and strong extractor.
- Public independent seed.
- Statistical closeness to uniform.

### Candidate results

**Min-entropy.**

\[
H_\infty(X)=-\log\max_xP(X=x).
\]

Thus the optimal one-shot guessing probability is `2^{-H_infinity(X)}`.

**Strong seeded extraction.** An extractor should make `(S,Ext(X,S))` close to
`(S,U_l)`, so the seed may be revealed.

**Leftover Hash Lemma.** Let `H` be chosen uniformly from a suitable
2-universal family independently of `X`. One standard convention gives

\[
\Delta\bigl((H,H(X)),(H,U_\ell)\bigr)
\leq \frac12\,2^{(\ell-H_\infty(X))/2}.
\]

Consequently, approximately

\[
\ell\leq H_\infty(X)-2\log(1/\varepsilon)
\]

bits can be extracted within statistical distance `epsilon`, up to additive
constants determined by the theorem convention.

**Classical side-information form.** If an observer holds correlated classical
side information `E` and the public hash seed is independent of the joint pair
`(X,E)`, the relevant source quality is average conditional min-entropy. A
corresponding form controls

\[
\Delta\bigl((E,H,H(X)),(E,H,U_\ell)\bigr)
\]

by the same type of expression with conditional min-entropy
`\widetilde H_\infty(X|E)`. This is the form needed for privacy amplification;
the unconditional statement alone only supports extraction when no correlated
observer information is present. Quantum side information requires a stronger
version outside the proposed scope.

**Public seed principle.** Security does not require the independently chosen
hash-function seed to remain secret because the joint output includes it.

### LEPL1109 dependency

- Discrete/joint distributions and independence:
  [probability foundations](../LEPL1109/LEPL1109_course_content.md#1-probability-foundations-and-random-variables)
  and [dependence](../LEPL1109/LEPL1109_course_content.md#3-dependence-and-multivariate-probability).
- Shannon entropy, statistical distance, and universal hashing are new FoL
  prerequisites and should precede the lemma.

### What is new beyond LEPL1109

- Min-entropy and worst-case predictability.
- Statistical extraction from weak randomness.
- A theorem joining hashing, entropy, and indistinguishability.

### FoC reuse

- Privacy amplification and key derivation from imperfect randomness.
- Information-theoretic secrecy and public discussion.
- Distinction between high Shannon entropy, high min-entropy, statistical
  uniformity, and computational pseudorandomness.

### Bibliography

[B11, Ch. 6](#b11), [B17](#b17).

## 20. Computational pseudorandomness and reductions

**Status:** Bridge overview in FoL; cryptographic formalization belongs to FoC.

### FoL handoff concepts

- Statistical indistinguishability quantifies all tests, including unbounded
  ones.
- Computational indistinguishability quantifies efficient tests and has no
  simple distribution metric known to the test.
- Efficient probabilistic algorithm and finite distinguisher advantage.
- Hybrid/telescoping argument from triangle inequality.
- Reduction: convert an adversary against a construction into an algorithm
  against an assumed primitive while tracking probability and running time.

### Candidate results

**Computational indistinguishability, finite-security preview.** Relative to a
specified efficient distinguisher class, two distributions are
`epsilon`-indistinguishable when every distinguisher has advantage at most
`epsilon`:

\[
|P(D(X)=1)-P(D(Y)=1)|\leq\varepsilon.
\]

**Hybrid lemma.** If a sequence contains `m` adjacent game hops and the total
endpoint advantage is `epsilon`, at least one adjacent hop has advantage at
least `epsilon/m`. Conversely, bounding each hop by `epsilon_i` bounds the
endpoint difference by `sum_i epsilon_i`.

### Boundary with FoC

FoL owns the statistical/computational contrast, finite efficient-distinguisher
advantage, and the generic hybrid/telescoping lemma. FoC owns asymptotic
security parameters and negligible functions, cryptographic PRGs/PRFs/PRPs,
random oracles, formal security games, concrete constructions, and reductions
to cryptographic assumptions.

### LEPL1109 dependency

- LEPL1109 only treats numerical PRNGs for simulation:
  [simulation](../LEPL1109/LEPL1109_course_content.md#6-simulation-and-bootstrap)
  [STAT, pp. 107-108].
- This must not be mistaken for cryptographic pseudorandomness.

### FoC reuse

Directly foundational for every computational-security topic in part B.

### Bibliography

[B11, Chs. 2-7](#b11), [B17, introductory chapters](#b17), [B18](#b18).

## 21. Integrated learning applications

**Status:** Core, distributed through lectures and practical sessions.

### Candidate applications

- Estimate a classifier's true risk from an independent test sample and attach
  a Hoeffding confidence guarantee.
- Compare a CLT interval with a nonasymptotic concentration interval.
- Select among finitely many classifiers and expose the extra `log |H|` term
  caused by uniform control.
- Interpret logistic cross-entropy as empirical cross-entropy and KL
  minimization.
- Compare MLE, MAP, posterior mean, and posterior prediction in a conjugate
  model.
- Demonstrate regularization as a prior and as a stability/bias-variance tool.
- Compare Shannon entropy and min-entropy for a skewed random source.
- Use one universal family both for a hash-map collision analysis and for
  leftover hashing, emphasizing that the guarantees and goals differ.
- Use GP regression to synthesize priors, conditioning, uncertainty, kernels,
  and regularization without repeating the LEPL1109 algebra.
- Show a causal counterexample in which high predictive accuracy does not
  identify an intervention effect.

### LEPL1109 relations

- Logistic regression and metrics:
  [classification](../LEPL1109/LEPL1109_course_content.md#16-logistic-regression-and-classification-assessment).
- Resampling and model selection:
  [resampling](../LEPL1109/LEPL1109_course_content.md#15-resampling-model-assessment-and-model-selection).
- Bias-variance:
  [bias-variance](../LEPL1109/LEPL1109_course_content.md#18-bias-variance-tradeoff-and-dimensionality).
- GP regression:
  [Gaussian processes](../LEPL1109/LEPL1109_course_content.md#11-gaussian-process-regression).

## 22. Robustness, sensitivity, and distribution shift

**Status:** Core application supporting an explicit course learning outcome.

### Concepts and candidate results

- Distinguish robustness to sampling variation, outliers/contamination,
  covariate shift, label noise, and adversarial perturbations.
- Use the LEPL1109 bias-variance decomposition to explain sensitivity to the
  sampled training set, but not as a complete theory of distribution shift.
- A bounded-loss Hoeffding guarantee is distribution-specific: it controls
  generalization under i.i.d. train/test sampling from the same population.
- If training and test distributions differ, for any loss bounded in `[0,1]`,
  total variation gives

\[
|E_P[\ell]-E_Q[\ell]|\leq\Delta(P,Q).
\]

- Regularization can reduce parameter sensitivity and variance, but does not by
  itself guarantee robustness to arbitrary distribution shift or adversarial
  examples.
- Robust summaries or losses can limit outlier influence; one practical may
  compare squared loss with absolute or Huber loss under contamination.
- Model-selection uncertainty should be acknowledged: repeated reuse of a
  validation set can overfit that set, and the untouched test set remains an
  estimate rather than a proof under future shift.

### LEPL1109 dependency

- Bias-variance and model selection:
  [bias-variance](../LEPL1109/LEPL1109_course_content.md#18-bias-variance-tradeoff-and-dimensionality)
  and [resampling](../LEPL1109/LEPL1109_course_content.md#15-resampling-model-assessment-and-model-selection).
- Outliers and preprocessing:
  [descriptive statistics](../LEPL1109/LEPL1109_course_content.md#2-descriptive-statistics-and-exploratory-data-analysis).
- LEPL1109 does not develop distribution shift, robust statistics, or
  adversarial robustness; those qualifications are new.

### FoC reuse

The distinction between empirical robustness and worst-case security is
important: robustness experiments do not establish a cryptographic security
property quantified over efficient adversaries.

### Bibliography

[B10](#b10), [B7](#b7), [B12](#b12).

# FoL-to-FoC interface

## Results that FoC should be able to reuse

| FoL result or notion | FoC use |
|---|---|
| Union bound | Compose failure events and security games |
| Hoeffding/Chernoff bounds | Amplification and random-construction guarantees |
| Birthday bound | Hash collisions, random-oracle queries, PRF/PRP switching |
| Total-variation distance | Information-theoretic real/ideal security |
| Test characterization of total variation | Statistical distinguishing advantage |
| Data processing and triangle inequality | Post-processing and hybrid games |
| Coupling lemma | Compare ideal and real experiments |
| Shannon/conditional entropy and chain rule | Perfect secrecy, leakage, impossibility |
| Mutual information and data processing | Zero leakage and limits of processing observations |
| Pinsker inequality | Convert KL/information bounds to distinguishing bounds |
| Min-entropy | Model weak keys and weak random sources |
| Universal hashing | MACs, collision analysis, and extractors |
| Leftover Hash Lemma | Privacy amplification and nearly uniform key derivation |
| Statistical versus finite computational indistinguishability | Handoff to PRGs, PRFs, encryption, MACs, signatures |
| Generic hybrid argument | Handoff to game-based security proofs and reductions |
| Bernoulli sample-complexity bound | Interpret empirical attack experiments without confusing them with proofs |
| Fano inequality, if included | Information-theoretic impossibility arguments |

## Recommended ownership boundary

| FoL owns | FoC owns |
|---|---|
| Statistical distance/indistinguishability, finite efficient-distinguisher advantage, generic hybrid lemma | Security parameters, negligible asymptotics, formal games, and concrete reductions |
| Shannon entropy, mutual information, min-entropy | Perfect-secrecy theorems and cryptographic leakage definitions |
| Universal/2-universal hashing | Collision resistance, preimage resistance, random oracles, hash constructions |
| Leftover hashing and extraction | Key-derivation use and protocol-level privacy amplification |
| Concentration, union bounds, birthday bound | Concrete cryptographic security bounds and query accounting |
| Generic hybrid/telescoping argument | Game-based proofs and reductions to PRF/PRP/public-key assumptions |
| Numerical PRNG versus true randomness distinction | Cryptographic PRGs, PRFs, and PRPs |
| Statistical sample complexity | Security parameter and adversarial query complexity |

## Terminology that must remain distinct

| Distinguish | Reason |
|---|---|
| Uniform random / high Shannon entropy / high min-entropy / computationally pseudorandom | These are non-equivalent guarantees |
| Statistical distance / KL divergence / mutual information | Metric, divergence, and dependence measure answer different questions |
| Statistical / computational indistinguishability | The former controls all tests; the latter only efficient tests |
| Universal hash / cryptographic hash / random oracle / PRF | Different objects, quantifiers, and security properties |
| Collision probability / collision resistance | Average over a random family is not resistance to an adversarial search |
| Sample complexity / query complexity | Statistical estimation and adversarial interaction are different resources |
| Empirical robustness / cryptographic security | Testing is not a reduction or universal adversarial guarantee |
| Bayes-optimal decision / Bayesian parameter inference | LEPL1109 covers the former; FoL adds the latter |

## Minimal handoff contract

Before FoC begins, students should be able to:

1. Apply union, Hoeffding/Chernoff, and birthday bounds.
2. Compute and interpret total-variation distance and distinguishing advantage.
3. Use entropy chain rules and interpret mutual information as leakage.
4. Distinguish Shannon entropy from min-entropy.
5. State and apply the Leftover Hash Lemma for a 2-universal family.
6. Distinguish universal hashing from cryptographic hashing.
7. Explain statistical versus computational indistinguishability using a finite
   efficient-distinguisher advantage; FoC will add negligible asymptotics.
8. Follow a hybrid argument and track its advantage loss.
9. Explain why empirical attack success does not prove computational security.

# Topics to defer or sharply limit

| Topic | Recommendation | Reason |
|---|---|---|
| Rigorous Markov-chain mixing times | Defer | Requires a Markov-chain foundation absent from LEPL1109 |
| Full MCMC convergence proofs | Defer | Too large relative to direct FoC value |
| Full source/channel coding theorems | Defer | Separate course-sized subject |
| Differential entropy in depth | Defer | Technical caveats distract from the discrete crypto interface |
| Le Cam/Assouad/minimax theory | Defer | Advanced statistical decision theory |
| Full fundamental theorem proof for VC learning | State, do not fully prove | Requires substantial combinatorics and technical qualifications |
| Rademacher complexity | Defer | VC or compression is enough for the first course |
| PAC-Bayes and stability | Defer | Each needs an independent proof toolkit |
| Bayesian asymptotics and variational inference | Defer | Exceeds the intended Bayesian introduction |
| Full causal inference/do-calculus | Defer | Cannot be treated responsibly as a short add-on |
| Re-derivation of GP regression | Do not repeat | Already covered by LEPL1109 |
| RKHS and representer theorem | Defer | Functional-analysis overhead and little FoC reuse |
| Cryptographic PRGs/PRFs/random oracles | FoC | Explicitly belongs to the cryptography syllabus |
| Extractor theory beyond leftover hashing | Defer | Specialized and unnecessary for the handoff |
| Modern deep-learning generalization | Defer | Not in the official topic list and too broad |

# Bibliography

## Primary references named in the FoLC proposal

### B1

Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, and Clifford Stein,
*Introduction to Algorithms*, 4th ed., MIT Press, 2022.
[Publisher page](https://mitpress.mit.edu/9780262046305/introduction-to-algorithms/).
Use Chapter 5 for probabilistic analysis/randomized algorithms and Chapter 11
for hash tables.

### B5

Thomas M. Cover and Joy A. Thomas, *Elements of Information Theory*, 2nd ed.,
Wiley, 2006.
[Publisher page](https://www.wiley.com/en-us/elements-of-information-theory-2nd-edition-p-9780471241959).
Chapter 2 covers entropy, KL divergence, mutual information, data processing,
and Fano's inequality; Chapter 11 connects information and statistics.

### B8

Shai Shalev-Shwartz and Shai Ben-David, *Understanding Machine Learning: From
Theory to Algorithms*, Cambridge University Press, 2014.
[Legal author page and PDF](https://www.cs.huji.ac.il/~shais/UnderstandingMachineLearning/).
Chapters 2-7 cover PAC learning, sample complexity, uniform convergence,
bias-complexity, and VC dimension.

### B18

Jonathan Katz and Yehuda Lindell, *Introduction to Modern Cryptography*, 3rd
ed., CRC Press, 2020.
[Publisher/DOI](https://doi.org/10.1201/9781351133036).
The opening foundations and pseudorandomness chapters define the FoC side of the
statistical/computational-randomness boundary.

### B3

Christian P. Robert and George Casella, *Monte Carlo Statistical Methods*, 2nd
ed., Springer, 2004.
[Publisher/DOI](https://doi.org/10.1007/978-1-4757-4145-2).
Chapters 3-4 cover random generation and Monte Carlo integration; Chapters 7-9
cover Metropolis-Hastings, Gibbs sampling, convergence, and diagnostics.

### B9

Stephen J. Wright and Benjamin Recht, *Optimization for Data Analysis*,
Cambridge University Press, 2022.
[Official book site](https://optimizationfordataanalysis.com/).
Useful for empirical-risk optimization, regularization, stochastic methods, and
the optimization side of learning.

### B10

Moritz Hardt and Benjamin Recht, *Patterns, Predictions, and Actions:
Foundations of Machine Learning*, Princeton University Press, 2022.
[Legal interactive text and PDF](https://mlstory.org/).
Chapter 6 treats generalization/capacity; Chapters 9-10 provide a compact
ML-oriented causal-inference introduction.

### B7

Francis Bach, *Learning Theory from First Principles*, MIT Press, 2024.
[Publisher page](https://mitpress.mit.edu/9780262048903/learning-theory-from-first-principles/)
and [legal author PDF](https://www.di.ens.fr/~fbach/ltfp_book.pdf).
Modern treatment of concentration, ERM, generalization, complexity,
regularization, kernels, and information-theoretic bounds.

### B19

Nigel P. Smart, *Cryptography Made Simple*, Springer, 2016.
[Publisher/DOI](https://doi.org/10.1007/978-3-319-21936-3).
Accessible support for the FoC security and primitive-design material.

### B20

Christof Paar, Jan Pelzl, and Tim Güneysu, *Understanding Cryptography*, 2nd
ed., Springer, 2024.
[Official book site](https://www.cryptography-textbook.com/).
Concrete algorithms and engineering context for symmetric/public-key
cryptography.

### B17

Dan Boneh and Victor Shoup, *A Graduate Course in Applied Cryptography*.
[Legal author page and full text](https://crypto.stanford.edu/~dabo/cryptobook/).
Useful for probability background, universal-hash MACs, symmetric encryption,
hashing, authenticated encryption, and public-key constructions.

## Additional primary references

### B2

Michael Mitzenmacher and Eli Upfal, *Probability and Computing: Randomization
and Probabilistic Techniques in Algorithms and Data Analysis*, 2nd ed.,
Cambridge University Press, 2017.
[Publisher page](https://www.cambridge.org/highereducation/books/probability-and-computing/EB9A09E85E36A6C4DD23DA3F5500A6D0).
Direct reference for balls-and-bins, concentration, randomized algorithms,
hashing, random graphs, and probabilistic proof techniques.

### B4

Andrew Gelman et al., *Bayesian Data Analysis*, 3rd ed., CRC Press, 2013.
[Legal author page and course materials](https://www.stat.columbia.edu/~gelman/book/).
Chapters 1-5 cover Bayesian foundations and hierarchical models; Chapters 10-12
cover MCMC.

### B6

David J. C. MacKay, *Information Theory, Inference, and Learning Algorithms*,
Cambridge University Press, 2003.
[Legal author page and full text](https://www.inference.org.uk/itila/book.html).
A unified and readable bridge among information theory, Bayesian inference,
Monte Carlo, coding, and learning.

### B11

Salil P. Vadhan, *Pseudorandomness*, Foundations and Trends in Theoretical
Computer Science 7(1-3), 2012.
[Legal author page and full text](https://people.seas.harvard.edu/~salil/pseudorandomness/)
and [DOI](https://doi.org/10.1561/0400000010).
Chapters 2-3 introduce randomness and derandomization; Chapter 6 covers
statistical distance, min-entropy, extractors, universal hashing, and the
Leftover Hash Lemma; Chapter 7 introduces pseudorandom generators.

### B12

Stéphane Boucheron, Gábor Lugosi, and Pascal Massart, *Concentration
Inequalities: A Nonasymptotic Theory of Independence*, Oxford University Press,
2013. [Publisher/DOI](https://doi.org/10.1093/acprof:oso/9780199535255.001.0001).
Primary advanced reference for Hoeffding, Bennett, Bernstein, bounded
differences, and the entropy method.

### B13

Miguel A. Hernán and James M. Robins, *Causal Inference: What If*, Chapman &
Hall/CRC, 2020.
[Legal author page and full text](https://miguelhernan.org/whatifbook).
Use Chapters 1-3 and 6-9 for counterfactuals, randomized experiments,
confounding, standardization, and causal diagrams.

### B14

Jonas Peters, Dominik Janzing, and Bernhard Schölkopf, *Elements of Causal
Inference: Foundations and Learning Algorithms*, MIT Press, 2017.
[Publisher page](https://mitpress.mit.edu/9780262037310/elements-of-causal-inference/)
and [DOI](https://doi.org/10.7551/mitpress/11283.001.0001).
Best suited to connecting structural causal models and causal discovery with
machine learning.

### B15

Nick Littlestone and Manfred Warmuth, "Relating Data Compression and
Learnability," 1986.
[ACM DOI](https://doi.org/10.1145/7902.7905).
Foundational sample-compression reference.

### B16

Carl Edward Rasmussen and Christopher K. I. Williams, *Gaussian Processes for
Machine Learning*, MIT Press, 2006.
[Legal author site and chapter PDFs](https://gaussianprocess.org/gpml/).
Chapter 2 covers regression, Chapter 4 kernels, and Chapter 5 model selection.

# Final design recommendation

The most coherent identity for FoL is not a survey of additional ML algorithms.
LEPL1109 already supplies introductory models, evaluation, Bayes decision
theory, bias-variance, PCA, clustering, and GP regression. FoL should instead be
the course segment on:

> Probability and information tools that explain when randomized learning
> algorithms generalize, how uncertainty is represented, and when imperfect
> randomness can safely support computation and cryptography.

The highest-value chain is:

\[
\text{Monte Carlo}
\longrightarrow
\text{concentration}
\longrightarrow
\text{PAC/sample complexity}
\longrightarrow
\text{entropy and statistical distance}
\longrightarrow
\text{universal hashing and extraction}
\longrightarrow
\text{FoC computational security}.
\]

Bayesian inference and regularization form the second conceptual chain:

\[
\text{likelihood from LEPL1109}
\longrightarrow
\text{posterior uncertainty}
\longrightarrow
\text{MAP/regularization}
\longrightarrow
\text{generalization and robustness}.
\]

MCMC, causality, and GP reinterpretation are valid course-description topics,
but should be practical-session modules or rotating optional material unless
the core list is shortened.
