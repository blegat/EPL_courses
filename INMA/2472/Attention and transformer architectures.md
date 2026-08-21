# Attention and transformer architectures

## Topics and results

- An attention head forms queries, keys, and values; scaled dot products score
  query–key compatibility and softmax produces convex weights for combining
  values (`TRANSFORMERS`, “Attention is all you need”).
- Matrix-form attention evaluates all token interactions in parallel. Causal
  masking prevents an autoregressive decoder from accessing future tokens.
- Multi-head attention learns several interaction patterns in parallel and
  concatenates their value mixtures.
- A decoder-only transformer combines masked self-attention, positional
  information, feed-forward blocks, residual connections, and layer
  normalization (`TRANSFORMERS`, “Decoder-only transformer”).
- A key–value cache avoids recomputing previous keys and values during
  autoregressive generation but grows with context length.
- Encoder–decoder transformers add cross-attention from decoder queries to
  encoder keys and values, as used in sequence-to-sequence translation.

## Connections

- Sequence modelling foundation: [Autoregressive language models and token embeddings](Autoregressive%20language%20models%20and%20token%20embeddings.md)
- Architectural survey: [LELEC2870 — deep learning architectures and training](../../ELEC/2870/Deep%20learning%20architectures%20and%20training.md)
