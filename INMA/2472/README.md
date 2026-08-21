# LINMA2472: algorithms in data science

This index summarizes the topics and principal results in the locally available
kernel-method slides and the public Pluto lecture notebooks. PDF references use
physical pages, counted from the first page of each file.

## Source abbreviations

- `KERNELS`: `linma2472_2025_2026_kernels.pdf` (81 pages)
- `APPLICATIONS`: `Kernels_applications_2025_v1.pdf` (115 pages)
- `RFF`: `Kernel methods - 2.pdf` (52 pages)
- `AD`: `autodiff.jl`
- `IMPLICIT`: `implicit.jl`
- `SPARSE`: `sparse.jl`
- `TRANSFORMERS`: `transformers.jl`
- `DIFFUSION`: `diffusion.jl`

The PDFs are local teaching material and intentionally ignored by Git. The
Pluto notebooks are part of the lectures and can be viewed on the
[public course site](https://blegat.github.io/LINMA2472/).

## Course map

| Topic | Note | Main sources |
|---:|---|---|
| 1 | [Feature spaces and the kernel trick](Feature%20spaces%20and%20the%20kernel%20trick.md) | KERNELS, pp. 2–46 |
| 2 | [Reproducing-kernel Hilbert spaces and kernel construction](Reproducing-kernel%20Hilbert%20spaces%20and%20kernel%20construction.md) | KERNELS, pp. 47–81 |
| 3 | [Kernel ridge regression and the representer theorem](Kernel%20ridge%20regression%20and%20the%20representer%20theorem.md) | APPLICATIONS, pp. 11–23 |
| 4 | [Support-vector machines and kernel classification](Support-vector%20machines%20and%20kernel%20classification.md) | APPLICATIONS, pp. 24–83 |
| 5 | [Kernel PCA and nonlinear structure discovery](Kernel%20PCA%20and%20nonlinear%20structure%20discovery.md) | APPLICATIONS, pp. 84–107 |
| 6 | [Bochner's theorem and random Fourier features](Bochner's%20theorem%20and%20random%20Fourier%20features.md) | RFF, pp. 1–52 |
| 7 | [Forward and reverse automatic differentiation](Forward%20and%20reverse%20automatic%20differentiation.md) | AD |
| 8 | [Higher-order and sparse automatic differentiation](Higher-order%20and%20sparse%20automatic%20differentiation.md) | AD; SPARSE |
| 9 | [Implicit differentiation and optimization sensitivity](Implicit%20differentiation%20and%20optimization%20sensitivity.md) | IMPLICIT |
| 10 | [Autoregressive language models and token embeddings](Autoregressive%20language%20models%20and%20token%20embeddings.md) | TRANSFORMERS |
| 11 | [Attention and transformer architectures](Attention%20and%20transformer%20architectures.md) | TRANSFORMERS |
| 12 | [Variational autoencoders and latent-variable learning](Variational%20autoencoders%20and%20latent-variable%20learning.md) | DIFFUSION |
| 13 | [Score matching and diffusion models](Score%20matching%20and%20diffusion%20models.md) | DIFFUSION |

## Relationship to the other courses

LEPL1109 supplies linear algebra, probability, regression, PCA, introductory
machine learning, and Gaussian processes. LELEC2870 and LINFO2262 develop many
classical learning algorithms and neural architectures. LINMA2472 emphasizes
the mathematical and computational mechanisms behind modern data-science
algorithms: implicit feature spaces, scalable kernel approximations,
program differentiation, differentiable optimization, transformers, and
generative models. LINMA2725 later reuses kernels and differentiable function
approximators in control and reinforcement learning.
