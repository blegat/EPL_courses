# Hand-crafted features and classical vision learning

## Topics and results

- Vision tasks range from classification and detection to semantic/instance
  segmentation, pose estimation, and retrieval; each moves from pixels toward a
  different semantic output (`AI1`, pp. 2–7).
- Classical pipelines first construct pixel-, patch-, geometry-, or graph-based
  features and then apply a separate inference or learning method (`AI1`,
  pp. 8–22).
- Random ferns combine simple randomized binary feature tests into efficient
  class-conditional evidence (`AI1`, pp. 23–27).
- AdaBoost iteratively emphasizes errors and combines weak classifiers; the
  Viola–Jones cascade illustrates how cheap rejection stages enable fast object
  detection (`AI1`, pp. 24–32).
- Linear SVMs maximize a margin in feature space. Histograms of oriented
  gradients paired with an SVM provide the canonical pedestrian-detection
  example (`AI1`, pp. 33–35).
- These pipelines offer interpretable design choices but depend on task-specific
  feature engineering, motivating end-to-end representation learning.

## Related courses

- SVM foundation: [LELEC2870 — support-vector machines and kernels](../2870/Support-vector%20machines%20and%20kernels.md)
- Classification survey: [LINFO2262 — linear discriminants and perceptron learning](../../INFO/2262/Linear%20discriminants%20and%20perceptron%20learning.md)
- Learned-feature alternative: [Convolutional neural networks for vision](Convolutional%20neural%20networks%20for%20vision.md)
