# Score matching and diffusion models

## Topics and results

- Tweedie's formula relates the posterior denoising mean under additive Gaussian
  noise to the score $\nabla_x\log p_\sigma(x)$ of the noise-smoothed density
  (`DIFFUSION`, “Tweedie's formula”).
- Denoising score matching trains a network from clean samples and injected
  noise without requiring the unknown data-density score.
- A range of noise levels bridges the difficult data distribution and an
  approximately Gaussian distribution; a noise-conditional score model learns
  the reverse denoising directions (`DIFFUSION`, “Variance-dependent score”).
- Sampling alternates score-directed denoising with calibrated randomness.
  Deterministic or reduced-step samplers trade exactness against speed
  (`DIFFUSION`, “Sampling” and “Acceleration”).
- Conditioning guides generation toward labels or text. Classifier-free
  guidance combines conditional and unconditional scores and exposes a
  fidelity–diversity trade-off.
- Latent diffusion performs the diffusion process in an autoencoder latent
  space to reduce computation while retaining a learned decoder.

## Connections

- Latent generative models: [Variational autoencoders and latent-variable learning](Variational%20autoencoders%20and%20latent-variable%20learning.md)
- Score gradients: [Forward and reverse automatic differentiation](Forward%20and%20reverse%20automatic%20differentiation.md)
