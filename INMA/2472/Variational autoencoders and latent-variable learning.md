# Variational autoencoders and latent-variable learning

## Topics and results

- A latent-variable model introduces $z$ and marginalizes
  $p_\theta(x)=\int p_\theta(x\mid z)p(z)\,dz$; exact posterior inference is
  generally intractable (`DIFFUSION`, “Variational Auto-Encoder”).
- An encoder $q_\phi(z\mid x)$ approximates the posterior and a decoder models
  $p_\theta(x\mid z)$.
- Jensen's inequality yields the evidence lower bound (ELBO): a reconstruction
  term minus $D_{\mathrm{KL}}(q_\phi(z\mid x)\|p(z))$.
- For Gaussian encoders, the reparameterization trick writes a latent sample as
  a differentiable transformation of parameter-free noise, enabling Monte Carlo
  gradient estimates (`DIFFUSION`, “Gaussian ELBO” and “Monte-Carlo sampling”).
- The KL term regularizes the latent representation toward the prior, enabling
  generation by sampling and decoding.

## Connections

- Alternative generative mechanism: [Score matching and diffusion models](Score%20matching%20and%20diffusion%20models.md)
- Architecture background: [LELEC2870 — deep learning architectures and training](../../ELEC/2870/Deep%20learning%20architectures%20and%20training.md)
