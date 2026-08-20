# 19. Unsupervised learning

### Scope and limitations

- Unsupervised learning has observations $x_i$ but no labels. Its aims are to
  discover arrangements, clusters, patterns, and hierarchies rather than make
  supervised predictions [UL, pp. 4-5].
- The course focuses on PCA for dimensionality reduction/visualization and
  preprocessing, and K-means for subgroup discovery [UL, p. 5].
- UL objectives and performance are less universally defined; analysis is often
  explanatory and lacks the simple validation target available in supervised
  learning [UL, p. 6].

### Principal component analysis

- Dimensionality-reduction objective: map data from $R^p$ to $R^{p'}$,
  $p'\ll p$, while retaining essential information. Merely selecting pairs
  of original axes is combinatorial and restricts the representation [UL,
  pp. 8-9].
- PCA finds axes maximizing projected variance for centered data [UL, p. 10].
- For unit direction $\phi$, scores are $z_i=x_i^T\phi$ and directional
  variance is $V(\phi)=N^{-1}\|X\phi\|^2$ [UL, pp. 11-12].
- The first PC maximizes the Rayleigh quotient and is the leading eigenvector of
  empirical covariance $C=N^{-1}X^TX$, with eigenvalue equal to explained
  variance. Later PCs maximize variance subject to orthogonality to prior PCs
  [UL, pp. 13-15].
- All PCs form an orthonormal basis adapted to the data; an observation can be
  expanded in canonical coordinates or PC scores [UL, p. 16].
- For centered $X\in R^{N\times p}$, $rank(X)\le\min(p,N-1)$, so at most
  $\min(p,N-1)$ PCs have nonzero explained variance [UL, p. 17].
- The first $k$ PCs span the $k$-dimensional subspace of maximum variance,
  equivalently the subspace minimizing total squared orthogonal reconstruction
  distance [UL, pp. 17-18].
- Forward transform stores scores
  $z_i=(\phi_1^Tx_i,\ldots,\phi_k^Tx_i)$; inverse transform reconstructs
  $x_i' =\sum_{j=1}^kz_{ij}\phi_j$ [UL, p. 19].
- Total variance is invariant under the orthonormal change of basis. Proportion
  of variance explained is $PVE(j)=V(\phi_j)/V_X$, decreases with PC index,
  and cumulative PVE supports choosing $k$ through scree/cumulative plots
  [UL, pp. 20-22].
- Data must be centered. Features should often be standardized to unit variance
  for meaningful PCA, but this can be inappropriate when physical scaling is
  meaningful [UL, p. 23].
- In practice PCA uses SVD $X=U\Sigma V^T$; PCs are columns of $V$, their
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
- A clustering $\mathcal C=\{C_1,\ldots,C_K\}$ partitions observation
  indices. K-means minimizes within-cluster pairwise squared variation [UL,
  pp. 41-43].
- Identity relating pairwise variation to centroid distances gives the standard
  objective $\sum_j\sum_{i\in C_j}\|x_i-c_j\|^2$, where
  $c_j=|C_j|^{-1}\sum_{i\in C_j}x_i$. Global optimization is NP-hard [UL,
  p. 43].
- Lloyd-Max algorithm alternates nearest-centroid assignment and centroid
  recomputation until within-cluster variation stops decreasing [UL,
  pp. 44-50].
- Number of clusters $K$ is a hyperparameter with no universal selection
  rule. Silhouette score uses mean within-cluster distance $a(x)$, nearest
  other-cluster distance $b(x)$, and
  $s(x)=(b-a)/\max(a,b)\in[-1,1]$ [UL, pp. 51-53].
- Good solutions seek high average silhouette, avoid clusters below that
  average, and avoid wide within-cluster score fluctuations; silhouette plots
  compare candidate $K$ [UL, pp. 53-57].
- A 64-dimensional digit example shows centroids, arbitrary cluster-label
  permutation, and post-hoc comparison to true labels [UL, pp. 58-60].
- Lloyd iterations monotonically decrease the objective but the problem is
  nonconvex and can end in local minima. Mitigation: multiple starts or
  K-means++ [UL, p. 61].
- Complexity is $O(NKpJ)$ for $J$ iterations; mini-batch and sparse
  variants address large $N$ or $p$ [UL, p. 62].
- K-means is sensitive to outliers and Euclidean cluster geometry. Preprocessing,
  K-medians, feature mappings, or spectral clustering are named alternatives
  [UL, pp. 62-63].

## Related courses

- Follow-on PCA treatment: [LELEC2870 — principal component analysis](../LELEC2870/Principal%20component%20analysis.md)
- Follow-on embeddings: [LELEC2870 — nonlinear dimensionality reduction and quality assessment](../LELEC2870/Nonlinear%20dimensionality%20reduction%20and%20quality%20assessment.md)
- Follow-on clustering: [LELEC2870 — vector quantization and prototype clustering](../LELEC2870/Vector%20quantization%20and%20prototype%20clustering.md)
- Follow-on topology-preserving learning: [LELEC2870 — self-organizing maps](../LELEC2870/Self-organizing%20maps%20and%20topology-preserving%20learning.md)
- Related LDACS1110 topic: [Sample compression and description length](../LDACS1110/Sample%20compression%20and%20description%20length.md)
