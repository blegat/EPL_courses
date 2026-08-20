# LINMA2725: stochastic optimal control and reinforcement learning

This index summarizes the topics and principal results in the slide decks under
[`slides/`](slides/). References use **physical PDF pages**, counted from the
first page of each file, independently of printed chapter pagination.

## Source abbreviations

- `CH1`: [`CH1.pdf`](slides/CH1.pdf) — LQR and LQG.
- `CH2`: [`CH2.pdf`](slides/CH2.pdf) — model predictive control.
- `CH3A`: [`CH3-a.pdf`](slides/CH3-a.pdf) — value/policy iteration motivation.
- `CH3B`: [`CH3-b.pdf`](slides/CH3-b.pdf) — approximation architectures and reinforcement-learning algorithms.
- `S67`: [`Chap6and7.pdf`](slides/Chap6and7.pdf) — stochastic systems and stochastic optimal control.
- `S9`: [`Chap9.pdf`](slides/Chap9.pdf) — temporal-difference and Q-learning techniques.
- `S10`: [`Chap10.pdf`](slides/Chap10.pdf) — actor–critic methods.
- `ONLINE`: [`OnlineLearning.pdf`](slides/OnlineLearning.pdf) — bandits, adaptive control, and partial observation.

The chapter notes describe themselves as preliminary and warn of possible
notation inconsistencies or imprecisions (`CH1`, p. 1; `CH2`, p. 2; `CH3B`,
p. 2). The notes below record the mathematical scope represented by the supplied
material, not a claim that every displayed theorem is examined.

## Course map

### Part I — deterministic systems and approximation

| Topic | Note | Main sources |
|---:|---|---|
| 1 | [Bellman equations, value iteration, and policy iteration](Bellman%20equations,%20value%20iteration,%20and%20policy%20iteration.md) | CH1, pp. 3–5; CH3A, pp. 2–6 |
| 2 | [Linear quadratic regulation and Riccati equations](Linear%20quadratic%20regulation%20and%20Riccati%20equations.md) | CH1, pp. 2–12 |
| 3 | [Linear quadratic Gaussian control](Linear%20quadratic%20Gaussian%20control.md) | CH1, pp. 13–15 |
| 4 | [Model predictive control](Model%20predictive%20control.md) | CH2, pp. 3–18 |
| 5 | [Value-function approximation architectures](Value-function%20approximation%20architectures.md) | CH3B, pp. 7–15 |
| 6 | [Exploration and stochastic-approximation ODEs](Exploration%20and%20stochastic-approximation%20ODEs.md) | CH3B, pp. 16–19 |
| 7 | [Temporal-difference learning and projected Bellman equations](Temporal-difference%20learning%20and%20projected%20Bellman%20equations.md) | CH3B, pp. 20–34 |
| 8 | [Deep and convex Q-learning](Deep%20and%20convex%20Q-learning.md) | CH3B, pp. 35–49 |

### Part II — stochastic systems and online learning

| Topic | Note | Main sources |
|---:|---|---|
| 9 | [Markov systems, invariant measures, and ergodicity](Markov%20systems,%20invariant%20measures,%20and%20ergodicity.md) | S67, pp. 5–20 |
| 10 | [Poisson equations and stochastic cost criteria](Poisson%20equations%20and%20stochastic%20cost%20criteria.md) | S67, pp. 21–31 |
| 11 | [Stochastic optimal control and dynamic programming](Stochastic%20optimal%20control%20and%20dynamic%20programming.md) | S67, pp. 32–47 |
| 12 | [Fluid models, policy sensitivity, and score functions](Fluid%20models,%20policy%20sensitivity,%20and%20score%20functions.md) | S67, pp. 48–55 |
| 13 | [Stochastic TD and off-policy Q-learning](Stochastic%20TD%20and%20off-policy%20Q-learning.md) | S9, pp. 4–46 |
| 14 | [Advantage functions and actor-critic methods](Advantage%20functions%20and%20actor-critic%20methods.md) | S10, pp. 4–35 |
| 15 | [Multi-armed bandits and regret](Multi-armed%20bandits%20and%20regret.md) | ONLINE, pp. 4–31 |
| 16 | [Adaptive LQR and partially observable control](Adaptive%20LQR%20and%20partially%20observable%20control.md) | ONLINE, pp. 32–53 |

## Relationship to the other courses

LEPL1109 provides probability, estimation, linear regression, resampling, time
series, and introductory supervised learning. LELEC2870 supplies gradient-based
optimization, neural networks, regularization, and kernels. LDACS1110 adds
concentration and finite-sample learning language. LINMA2725 uses these tools in
sequential decision problems, adding state-space dynamics, Bellman and Poisson
equations, optimal control, reinforcement learning, regret, and partial
observation. Reciprocal links are recorded in the relevant topic notes.

