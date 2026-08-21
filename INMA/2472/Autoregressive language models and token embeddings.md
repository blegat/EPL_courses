# Autoregressive language models and token embeddings

## Topics and results

- An autoregressive model factorizes a sequence probability into conditional
  next-token probabilities and generates by repeated conditional sampling
  (`TRANSFORMERS`, “Autoregressive Models”).
- Tokenization balances vocabulary size against sequence length. Byte-pair
  encoding repeatedly merges frequent adjacent units to build a subword
  vocabulary (`TRANSFORMERS`, “Text to vectors”).
- A learned embedding maps token identifiers to dense vectors; shared embeddings
  let statistical strength transfer across positions and contexts.
- Recurrent neural networks process arbitrary-length histories through a hidden
  state, but long products of Jacobians lead to vanishing or exploding
  gradients. Gated recurrent units mitigate rather than eliminate this issue
  (`TRANSFORMERS`, “Pre-transformers approaches”).
- Next-token cross-entropy training connects the probabilistic factorization to
  gradient-based representation learning.

## Connections

- Long-range alternative: [Attention and transformer architectures](Attention%20and%20transformer%20architectures.md)
- Information measure: [LDACS1110 — KL divergence, cross-entropy, and mutual information](../../DACS/1110/KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md)
