# Classical clustering methods and evaluation

Partitioning methods optimize a within-cluster objective: $k$-means uses centroids, $k$-medoids uses representative observations, and $k$-modes adapts the idea to categorical data. Hierarchical clustering builds a dendrogram using linkage choices; density-based and grid-based methods can recover non-convex groups and treat sparse regions as noise.

Cluster evaluation uses internal cohesion/separation criteria, external labels when available, stability, and domain usefulness. Selecting the number of clusters is inseparable from the geometry and scale of the data.

Source: `S9`, pp. 2–127.

## Related courses

- [Instance-based learning, prototypes, and clustering](../../INFO/2262/Instance-based%20learning,%20prototypes,%20and%20clustering.md) introduces prototype and distance-based viewpoints.
- [Advanced clustering and biclustering](Advanced%20clustering%20and%20biclustering.md) treats soft, high-dimensional, and structured variants.
- [Outlier and anomaly detection](Outlier%20and%20anomaly%20detection.md) reuses distance and density concepts.
- [LELEC2885 — edges, watersheds, and feature-based segmentation](../../ELEC/2885/Edges,%20watersheds,%20and%20feature-based%20segmentation.md) applies centroid, density, and spectral clustering to visual features.
