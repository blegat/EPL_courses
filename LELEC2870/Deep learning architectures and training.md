# Deep learning architectures and training

## Topics and results

- Deep learning combines multilayer representations, large data sets, and
  computation to learn hierarchical features; depth can express compositional
  structure efficiently but makes optimization harder (`DL`, pp. 2–10).
- The historical path covers threshold neurons, perceptrons, multilayer
  backpropagation, convolutional networks, and the universal approximation
  theorem (`DL`, pp. 13–47).
- Convolution uses local connectivity and shared weights to form feature maps;
  receptive fields grow through depth/dilation, while pooling subsamples and
  adds limited invariance (`DL`, pp. 30–47, 66–67).
- Multiclass networks use one-hot targets and softmax outputs with an appropriate
  classification loss (`DL`, pp. 48–51).
- Vanishing gradients motivate activation, initialization, normalization,
  residual/skip, pretraining, and optimization techniques (`DL`, pp. 55–78,
  88–106).
- Autoencoders learn a bottleneck representation through reconstruction;
  transfer learning reuses representations across source and target domains
  (`DL`, pp. 57–65, 75–78).
- The survey introduces CNNs, GANs, ResNets, U-Nets, DenseNets, transformers,
  and vision transformers, emphasizing their architectural ideas rather than a
  full derivation of every model (`DL`, pp. 79–95, 107–108).
- Limitations include high data/compute demand, empirical sensitivity,
  interpretability and theory gaps, robustness concerns, and environmental or
  social costs (`DL`, pp. 101–116).

## Related courses

- Foundation: [Multilayer perceptrons and backpropagation](Multilayer%20perceptrons%20and%20backpropagation.md)
- Autoencoder/embedding connection: [Nonlinear dimensionality reduction and quality assessment](Nonlinear%20dimensionality%20reduction%20and%20quality%20assessment.md)
- Prerequisite boundary: [LEPL1109 — explicitly extra or not covered](../LEPL1109/Explicitly%20extra,%20deferred,%20or%20not%20covered.md)
- Robustness context: [LDACS1110 — robustness and distribution shift](../LDACS1110/Robustness,%20sensitivity,%20and%20distribution%20shift.md)

