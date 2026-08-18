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
  [LEPL1109 content](../LEPL1109/content.md).
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
| [Randomized algorithms](Randomized%20algorithms%20and%20probabilistic%20analysis.md), [Monte Carlo](Monte%20Carlo%20estimation.md), [concentration](Concentration%20inequalities.md) | 3.0 | Convert expectation estimates into finite-sample `(epsilon, delta)` guarantees |
| [Entropy](Shannon%20entropy%20and%20conditional%20entropy.md), [KL, mutual information](KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md), [statistical distance](Statistical%20distance%20and%20couplings.md) | 2.5 | Quantify uncertainty, dependence, leakage, and distinguishability |
| [Bayesian inference](Bayesian%20inference.md) and [regularization](MAP%20estimation%20and%20regularization.md) | 2.0 | Derive posterior prediction and connect MAP to penalized learning |
| [PAC learning and finite-class generalization](PAC%20learning%20and%20finite-class%20sample%20complexity.md) | 2.5 | Derive sample complexity from concentration and a union bound |
| [VC dimension](VC%20dimension%20and%20growth%20functions.md) **or** [sample compression](Sample%20compression%20and%20description%20length.md) | 1.5 | Extend generalization beyond finite hypothesis classes |
| [Universal hashing](Universal%20hashing%20and%20randomized%20hash%20maps.md), [min-entropy, leftover hashing](Min-entropy%20and%20randomness%20extraction.md) | 2.0 | Turn weak randomness into nearly uniform bits and prepare FoC |
| [Integrated applications](Integrated%20learning%20applications.md) and FoC handoff | 1.5 | Relate existing LEPL1109 models to the new theory and distinguish randomness notions |
| **Total** | **15.0** | |

MCMC, Fano's inequality, Gaussian processes, and causality remain in the
official topic pool, but a rigorous treatment of all four is incompatible with
this spine. Suggested ways to include them are given below.

### Coverage-complete 15-hour survey alternative

| Block                                                 |    Hours | Depth                                                                         |
| ----------------------------------------------------- | -------: | ----------------------------------------------------------------------------- |
| [Randomized algorithms](Randomized%20algorithms%20and%20probabilistic%20analysis.md), [Monte Carlo](Monte%20Carlo%20estimation.md), [concentration](Concentration%20inequalities.md)     |      2.5 | Prove Markov/Chernoff method/Hoeffding; state birthday and amplification      |
| **[Metropolis-Hastings and Gibbs](Markov-chain%20Monte%20Carlo.md)**                     |      1.0 | Derive detailed balance; state ergodic convergence; practical demonstration   |
| [Entropy](Shannon%20entropy%20and%20conditional%20entropy.md), [KL, mutual information](KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md), [statistical distance](Statistical%20distance%20and%20couplings.md) |      2.0 | Prove elementary identities and data processing selectively                   |
| [Fano and information-theoretic lower bounds](Fano's%20inequality%20and%20information-theoretic%20lower%20bounds.md)           |      0.5 | State and apply once                                                          |
| [Bayesian inference](Bayesian%20inference.md), [MAP, and regularization](MAP%20estimation%20and%20regularization.md) |      1.5 | One conjugate model and Gaussian-prior/ridge connection                       |
| [Causality](Causal%20inference.md)                                             |      0.5 | Association/intervention distinction and one confounding example              |
| [PAC, sample complexity](PAC%20learning%20and%20finite-class%20sample%20complexity.md), [VC dimension](VC%20dimension%20and%20growth%20functions.md), [compression](Sample%20compression%20and%20description%20length.md)     |      3.0 | Prove finite-class bound; state Sauer-Shelah/VC theorem and compression bound |
| [GP regression](Gaussian-process%20regression%20as%20Bayesian%20learning.md)                                         |      0.5 | Bayesian reinterpretation of LEPL1109, no repeated derivation                 |
| [Universal hashing](Universal%20hashing%20and%20randomized%20hash%20maps.md), [min-entropy, leftover hashing](Min-entropy%20and%20randomness%20extraction.md)      |      2.0 | Prove collision facts; state or sketch LHL                                    |
| [Applications](Integrated%20learning%20applications.md) and FoC handoff                          |      1.5 | Robustness, randomness hierarchy, and hybrid argument                         |
| **Total**                                             | **15.0** |                                                                               |

This alternative touches every prescribed heading but necessarily treats
several results as statement-only. Detailed derivations, coding experiments,
and applications should use the course's practical hours rather than being
added to the 15 lecture hours.

# Candidate topic catalogue

| Topic | Note |
|---:|---|
| 1 | [Randomized algorithms and probabilistic analysis](Randomized%20algorithms%20and%20probabilistic%20analysis.md) |
| 2 | [Monte Carlo estimation](Monte%20Carlo%20estimation.md) |
| 3 | [Concentration inequalities](Concentration%20inequalities.md) |
| 4 | [Direct and weighted sampling methods](Direct%20and%20weighted%20sampling%20methods.md) |
| 5 | [Markov-chain Monte Carlo](Markov-chain%20Monte%20Carlo.md) |
| 6 | [Shannon entropy and conditional entropy](Shannon%20entropy%20and%20conditional%20entropy.md) |
| 7 | [KL divergence, cross-entropy, and mutual information](KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md) |
| 8 | [Statistical distance and couplings](Statistical%20distance%20and%20couplings.md) |
| 9 | [Fano's inequality and information-theoretic lower bounds](Fano's%20inequality%20and%20information-theoretic%20lower%20bounds.md) |
| 10 | [Bayesian inference](Bayesian%20inference.md) |
| 11 | [MAP estimation and regularization](MAP%20estimation%20and%20regularization.md) |
| 12 | [Causal inference](Causal%20inference.md) |
| 13 | [PAC learning and finite-class sample complexity](PAC%20learning%20and%20finite-class%20sample%20complexity.md) |
| 14 | [VC dimension and growth functions](VC%20dimension%20and%20growth%20functions.md) |
| 15 | [Sample compression and description length](Sample%20compression%20and%20description%20length.md) |
| 16 | [Further generalization frameworks](Further%20generalization%20frameworks.md) |
| 17 | [Gaussian-process regression as Bayesian learning](Gaussian-process%20regression%20as%20Bayesian%20learning.md) |
| 18 | [Universal hashing and randomized hash maps](Universal%20hashing%20and%20randomized%20hash%20maps.md) |
| 19 | [Min-entropy and randomness extraction](Min-entropy%20and%20randomness%20extraction.md) |
| 20 | [Computational pseudorandomness and reductions](Computational%20pseudorandomness%20and%20reductions.md) |
| 21 | [Integrated learning applications](Integrated%20learning%20applications.md) |
| 22 | [Robustness, sensitivity, and distribution shift](Robustness,%20sensitivity,%20and%20distribution%20shift.md) |

# FoL-to-FoC interface

## Results that FoC should be able to reuse

| FoL result or notion | FoC use |
|---|---|
| [Union bound](Randomized%20algorithms%20and%20probabilistic%20analysis.md) | Compose failure events and security games |
| [Hoeffding/Chernoff bounds](Concentration%20inequalities.md) | Amplification and random-construction guarantees |
| [Birthday bound](Randomized%20algorithms%20and%20probabilistic%20analysis.md) | Hash collisions, random-oracle queries, PRF/PRP switching |
| [Total-variation distance](Statistical%20distance%20and%20couplings.md) | Information-theoretic real/ideal security |
| [Test characterization of total variation](Statistical%20distance%20and%20couplings.md) | Statistical distinguishing advantage |
| [Data processing and triangle inequality](Statistical%20distance%20and%20couplings.md) | Post-processing and hybrid games |
| [Coupling lemma](Statistical%20distance%20and%20couplings.md) | Compare ideal and real experiments |
| [Shannon/conditional entropy and chain rule](Shannon%20entropy%20and%20conditional%20entropy.md) | Perfect secrecy, leakage, impossibility |
| [Mutual information and data processing](KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md) | Zero leakage and limits of processing observations |
| [Pinsker inequality](KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md) | Convert KL/information bounds to distinguishing bounds |
| [Min-entropy](Min-entropy%20and%20randomness%20extraction.md) | Model weak keys and weak random sources |
| [Universal hashing](Universal%20hashing%20and%20randomized%20hash%20maps.md) | MACs, collision analysis, and extractors |
| [Leftover Hash Lemma](Min-entropy%20and%20randomness%20extraction.md) | Privacy amplification and nearly uniform key derivation |
| [Statistical versus finite computational indistinguishability](Computational%20pseudorandomness%20and%20reductions.md) | Handoff to PRGs, PRFs, encryption, MACs, signatures |
| [Generic hybrid argument](Computational%20pseudorandomness%20and%20reductions.md) | Handoff to game-based security proofs and reductions |
| [Bernoulli sample-complexity bound](PAC%20learning%20and%20finite-class%20sample%20complexity.md) | Interpret empirical attack experiments without confusing them with proofs |
| [Fano inequality, if included](Fano's%20inequality%20and%20information-theoretic%20lower%20bounds.md) | Information-theoretic impossibility arguments |

## Recommended ownership boundary

| FoL owns | FoC owns |
|---|---|
| [Statistical distance/indistinguishability, finite efficient-distinguisher advantage, generic hybrid lemma](Computational%20pseudorandomness%20and%20reductions.md) | Security parameters, negligible asymptotics, formal games, and concrete reductions |
| [Shannon entropy](Shannon%20entropy%20and%20conditional%20entropy.md), [mutual information](KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md), [min-entropy](Min-entropy%20and%20randomness%20extraction.md) | Perfect-secrecy theorems and cryptographic leakage definitions |
| [Universal/2-universal hashing](Universal%20hashing%20and%20randomized%20hash%20maps.md) | Collision resistance, preimage resistance, random oracles, hash constructions |
| [Leftover hashing and extraction](Min-entropy%20and%20randomness%20extraction.md) | Key-derivation use and protocol-level privacy amplification |
| [Concentration, union bounds](Concentration%20inequalities.md), [birthday bound](Randomized%20algorithms%20and%20probabilistic%20analysis.md) | Concrete cryptographic security bounds and query accounting |
| [Generic hybrid/telescoping argument](Computational%20pseudorandomness%20and%20reductions.md) | Game-based proofs and reductions to PRF/PRP/public-key assumptions |
| [Numerical PRNG versus true randomness distinction](Computational%20pseudorandomness%20and%20reductions.md) | Cryptographic PRGs, PRFs, and PRPs |
| [Statistical sample complexity](PAC%20learning%20and%20finite-class%20sample%20complexity.md) | Security parameter and adversarial query complexity |

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
| [Rigorous Markov-chain mixing times](Markov-chain%20Monte%20Carlo.md) | Defer | Requires a Markov-chain foundation absent from LEPL1109 |
| [Full MCMC convergence proofs](Markov-chain%20Monte%20Carlo.md) | Defer | Too large relative to direct FoC value |
| [Full source/channel coding theorems](Shannon%20entropy%20and%20conditional%20entropy.md) | Defer | Separate course-sized subject |
| Differential entropy in depth | Defer | Technical caveats distract from the discrete crypto interface |
| [Le Cam/Assouad/minimax theory](Fano's%20inequality%20and%20information-theoretic%20lower%20bounds.md) | Defer | Advanced statistical decision theory |
| [Full fundamental theorem proof for VC learning](VC%20dimension%20and%20growth%20functions.md) | State, do not fully prove | Requires substantial combinatorics and technical qualifications |
| [Rademacher complexity](Further%20generalization%20frameworks.md) | Defer | VC or compression is enough for the first course |
| [PAC-Bayes and stability](Further%20generalization%20frameworks.md) | Defer | Each needs an independent proof toolkit |
| [Bayesian asymptotics and variational inference](Bayesian%20inference.md) | Defer | Exceeds the intended Bayesian introduction |
| [Full causal inference/do-calculus](Causal%20inference.md) | Defer | Cannot be treated responsibly as a short add-on |
| Re-derivation of [GP regression](Gaussian-process%20regression%20as%20Bayesian%20learning.md) | Do not repeat | Already covered by LEPL1109 |
| RKHS and representer theorem | Defer | Functional-analysis overhead and little FoC reuse |
| [Cryptographic PRGs/PRFs/random oracles](Computational%20pseudorandomness%20and%20reductions.md) | FoC | Explicitly belongs to the cryptography syllabus |
| [Extractor theory beyond leftover hashing](Min-entropy%20and%20randomness%20extraction.md) | Defer | Specialized and unnecessary for the handoff |
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
theory, bias-variance, PCA, clustering, and [GP regression](Gaussian-process%20regression%20as%20Bayesian%20learning.md). FoL should instead be
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

[Bayesian inference](Bayesian%20inference.md) and [regularization](MAP%20estimation%20and%20regularization.md) form the second conceptual chain:

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
