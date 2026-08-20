# Convolutional neural networks for vision

## Topics and results

- Deep vision replaces hand-crafted features with representations trained from
  raw pixels; cross-entropy and backpropagation fit multilayer networks (`AI2`,
  pp. 2–13).
- Initialization, normalization, regularization, and stochastic optimization
  control exploding/vanishing gradients and generalization (`AI2`, pp. 14–16).
- CNNs encode locality and translation sharing through learned convolution
  kernels and feature maps, avoiding the parameter count and structural blindness
  of fully connected image models (`AI3`, pp. 1–4).
- Pooling, stride, nonlinear activation, and upsampling control resolution and
  receptive fields. A backbone produces representations consumed by a
  task-specific head (`AI3`, pp. 3–8).
- LeNet, AlexNet, VGG, residual networks, and encoder–decoder/U-Net designs
  illustrate increasing depth, skip connections, and dense prediction (`AI3`,
  pp. 5–18).
- Heads specialize shared features for classification, detection, segmentation,
  pose estimation, retrieval, or image generation (`AI3`, pp. 19–29).
- Overparameterization, shortcut learning, domain shift, adversarial behavior,
  calibration, data requirements, and limited theory qualify empirical success
  (`AI3`, pp. 30–37).

## Related courses

- General neural-network treatment: [LELEC2870 — deep learning architectures and training](../2870/Deep%20learning%20architectures%20and%20training.md)
- Classical pipeline: [Hand-crafted features and classical vision learning](Hand-crafted%20features%20and%20classical%20vision%20learning.md)
- Dense prediction: [Feature-based classification and active contours](Feature-based%20classification%20and%20active%20contours.md)
- Detection input to tracking: [Detection-based multi-object tracking](Detection-based%20multi-object%20tracking.md)
- Geometric alternative to learned correspondence: [Camera calibration, homographies, and image stitching](Camera%20calibration,%20homographies,%20and%20image%20stitching.md)
