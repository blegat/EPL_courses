# Nonlinear dimensionality reduction and quality assessment

## Topics and results

- Dimensionality reduction seeks a lower-dimensional representation revealing
  structure in high-dimensional data; the manifold hypothesis motivates
  nonlinear embeddings (`NLDR`, pp. 8–13).
- Stress-based multidimensional scaling, Sammon mapping, nonmetric MDS, and
  curvilinear component analysis optimize different distance/rank preservation
  objectives (`NLDR`, pp. 14–20).
- SOMs and autoencoders provide prototype/grid-based and neural reconstruction
  approaches (`NLDR`, pp. 22–28).
- SNE, t-SNE, and related neighbor embeddings convert distances into
  neighborhood similarities and minimize distribution mismatch; their scale
  parameters and stochastic optimization materially affect results (`NLDR`,
  pp. 30–45).
- Concentration of norms and distances in high dimension makes literal global
  distance preservation impossible or misleading, motivating shift/scale-aware
  similarities (`NLDR`, pp. 35–45).
- Rank- and neighborhood-based quality criteria measure trustworthiness and
  continuity across neighborhood sizes; an area-under-curve summary aggregates
  multiscale quality (`NLDR`, pp. 47–64).
- Practical conclusions stress preprocessing, metric choice, parameter
  sensitivity, scalability, multiple runs, and the non-identifiability of a
  single visually attractive embedding (`NLDR`, pp. 65–70).

## Related courses

- Linear baseline: [Principal component analysis](Principal%20component%20analysis.md)
- Neural approach: [Deep learning architectures and training](Deep%20learning%20architectures%20and%20training.md)
- Prerequisite: [LEPL1109 — unsupervised learning](../../EPL/1109/Unsupervised%20learning.md)
- Topology-preserving method: [Self-organizing maps and topology-preserving learning](Self-organizing%20maps%20and%20topology-preserving%20learning.md)
- Kernel spectral method: [LINMA2472 — kernel PCA and nonlinear structure discovery](../../INMA/2472/Kernel%20PCA%20and%20nonlinear%20structure%20discovery.md)
