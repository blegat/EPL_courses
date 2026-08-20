# Edges, watersheds, and feature-based segmentation

## Topics and results

- Edges are locations of strong directional intensity change. Gradients estimate
  orientation and strength; derivative filters require smoothing because
  differentiation amplifies noise (`SEG`, pp. 3–16).
- Watershed segmentation interprets an intensity or gradient image as topography.
  Flooding catchment basins produces separating dams, but raw minima often cause
  oversegmentation and motivate markers or preprocessing (`SEG`, pp. 17–28).
- Feature-based segmentation maps pixels or patches into a feature space built
  from color, texture, filter responses, position, or learned descriptors
  (`SEG`, pp. 29–40).
- K-means alternates assignment to nearest centroids and centroid updates;
  initialization affects the local optimum, with K-means++ providing a more
  robust randomized initialization (`SEG`, pp. 41–52).
- Mean shift performs nonparametric mode seeking, allowing clusters to arise from
  density rather than a fixed number of centroids (`SEG`, pp. 53–59).
- Graph-based segmentation represents samples as weighted vertices. The graph
  Laplacian and its eigenvectors expose connected structure and enable spectral
  clustering in non-Euclidean geometries (`SEG`, pp. 60–70).

## Related courses

- Binary and grayscale cleanup: [Mathematical morphology](Mathematical%20morphology.md)
- Broader clustering survey: [LINFO2364 — classical clustering methods and evaluation](../../INFO/2364/Classical%20clustering%20methods%20and%20evaluation.md)
- Contour-based continuation: [Feature-based classification and active contours](Feature-based%20classification%20and%20active%20contours.md)
