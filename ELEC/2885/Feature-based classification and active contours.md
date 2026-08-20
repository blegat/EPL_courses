# Feature-based classification and active contours

## Topics and results

- A visual classifier maps feature vectors to semantic labels; training data,
  representation, decision rule, and evaluation form the common pipeline
  (`SEG`, pp. 71–79).
- Minimum-distance classification assigns a sample to the closest class centroid
  but cannot capture anisotropic, multimodal, or nonlinear class geometry
  (`SEG`, pp. 80–89).
- Contour descriptors and shape priors complement region features in recognition
  and medical-image analysis (`SEG`, pp. 90–99).
- Parametric curves represent closed object boundaries and permit geometric
  regularity terms such as elasticity and curvature (`SEG`, pp. 100–109).
- Active contours minimize an energy combining internal smoothness with external
  image forces. Boundary-based snakes use gradients, while region-based models
  exploit statistics inside and outside the curve (`SEG`, pp. 110–124).
- Level-set formulations represent a contour implicitly, naturally accommodate
  topology changes, and evolve the boundary through a PDE-like update (`SEG`,
  pp. 125–139).

## Related courses

- Region-based precursor: [Edges, watersheds, and feature-based segmentation](Edges,%20watersheds,%20and%20feature-based%20segmentation.md)
- Classification foundations: [LINFO2262 — learning problems and classification workflow](../../INFO/2262/Learning%20problems%20and%20classification%20workflow.md)
- Learned visual classifiers: [Convolutional neural networks for vision](Convolutional%20neural%20networks%20for%20vision.md)
