# LDACS1110: foundations of machine learning and cryptography

This index maps the Foundations of Learning topics and their principal results.
The learning part develops probability and information tools for randomized
algorithms, statistical learning, imperfect randomness, and the transition to
the Foundations of Cryptography part of the course.

## Course map

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

## Main connections

The topics form three connected strands:

1. Monte Carlo estimation, concentration, PAC learning, and generalization.
2. Entropy, divergence, statistical distance, Bayesian uncertainty, and
   regularization.
3. Universal hashing, min-entropy, extraction, and the transition from
   statistical to computational indistinguishability.

Integrated applications connect these strands through estimation, model
selection, robustness, and randomized computation.

## Relationship to the other courses

LEPL1109 supplies the prerequisite probability, statistics, simulation,
estimation, decision theory, introductory machine learning, and
Gaussian-process regression. LDACS1110 adds nonasymptotic guarantees,
information-theoretic tools, Bayesian parameter inference, learning theory,
weak-randomness tools, and the interface with cryptographic reasoning.

Later courses reuse these foundations in model selection and representation
learning (LELEC2870), sparse imaging and particle tracking (LELEC2885),
classification (LINFO2262), and stochastic modelling, bandits, reinforcement
learning, and control (LINMA2470 and LINMA2725).

## References

### B1

Thomas H. Cormen et al., *Introduction to Algorithms*, 4th ed., MIT Press, 2022.

### B2

Michael Mitzenmacher and Eli Upfal, *Probability and Computing*, 2nd ed.,
Cambridge University Press, 2017.

### B3

Christian P. Robert and George Casella, *Monte Carlo Statistical Methods*, 2nd
ed., Springer, 2004.

### B4

Andrew Gelman et al., *Bayesian Data Analysis*, 3rd ed., CRC Press, 2013.

### B5

Thomas M. Cover and Joy A. Thomas, *Elements of Information Theory*, 2nd ed.,
Wiley, 2006.

### B6

David J. C. MacKay, *Information Theory, Inference, and Learning Algorithms*,
Cambridge University Press, 2003.

### B7

Francis Bach, *Learning Theory from First Principles*, MIT Press, 2024.

### B8

Shai Shalev-Shwartz and Shai Ben-David, *Understanding Machine Learning*,
Cambridge University Press, 2014.

### B9

Stephen J. Wright and Benjamin Recht, *Optimization for Data Analysis*,
Cambridge University Press, 2022.

### B10

Moritz Hardt and Benjamin Recht, *Patterns, Predictions, and Actions*,
Princeton University Press, 2022.

### B11

Salil P. Vadhan, *Pseudorandomness*, Foundations and Trends in Theoretical
Computer Science 7(1–3), 2012.

### B12

Stéphane Boucheron, Gábor Lugosi, and Pascal Massart, *Concentration
Inequalities*, Oxford University Press, 2013.

### B13

Miguel A. Hernán and James M. Robins, *Causal Inference: What If*, Chapman &
Hall/CRC, 2020.

### B14

Jonas Peters, Dominik Janzing, and Bernhard Schölkopf, *Elements of Causal
Inference*, MIT Press, 2017.

### B15

Nick Littlestone and Manfred Warmuth, “Relating Data Compression and
Learnability,” 1986.

### B16

Carl Edward Rasmussen and Christopher K. I. Williams, *Gaussian Processes for
Machine Learning*, MIT Press, 2006.

### B17

Dan Boneh and Victor Shoup, *A Graduate Course in Applied Cryptography*.

### B18

Jonathan Katz and Yehuda Lindell, *Introduction to Modern Cryptography*, 3rd
ed., CRC Press, 2020.

### B19

Nigel P. Smart, *Cryptography Made Simple*, Springer, 2016.

### B20

Christof Paar, Jan Pelzl, and Tim Güneysu, *Understanding Cryptography*, 2nd
ed., Springer, 2024.
