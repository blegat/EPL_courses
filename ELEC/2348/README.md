# LELEC2348: information theory and coding

This index summarizes the topics and principal results in the locally available
course material. References use physical PDF pages, counted from the first page.

## Source abbreviations

- `SOURCE`: `LINGI2348-TIC-syllabus-en.pdf` (48 pages)
- `LEMPEL`: `Lempel.pdf` (16 pages)
- `CAP`: `Lecture4_Chap1.pdf` (43 pages)
- `NOISY`: `Lecture5_Chap2.pdf` (26 pages)
- `BLOCK`: `Lecture1-Chap3.pdf` (29 pages)
- `LINEAR`: `Lecture2_Chap4-5.pdf` (40 pages)
- `RS`: `Lecture3_Chap6.pdf` (23 pages)
- `CONV`: `Lecture3_Chap7.pdf` (11 pages)
- `LDPC`: `INGI2348_LDPC.pdf` (29 pages)
- `DESIGN`: `Lecture6_Design.pdf` (9 pages)
- `CRYPTO`: `LELEC2348 - Slides - Information theoretic cryptography.pdf`
  (50 pages)
- `MPC`: `MPCbook.pdf` (167 pages)

The PDFs are local teaching material and intentionally ignored by Git. The
abbreviations identify them without creating links that would be broken for
other clones. Exercises and their solutions support the same material but are
not treated as separate topics.

## Part A: source coding

| Topic | Note | Main source |
|---:|---|---|
| 1 | [Entropy, conditional entropy, and mutual information](Entropy,%20conditional%20entropy,%20and%20mutual%20information.md) | SOURCE, pp. 13–25 |
| 2 | [Prefix codes, Kraft inequality, and Huffman coding](Prefix%20codes,%20Kraft%20inequality,%20and%20Huffman%20coding.md) | SOURCE, pp. 26–31 |
| 3 | [Arithmetic, dictionary, and universal coding](Arithmetic,%20dictionary,%20and%20universal%20coding.md) | SOURCE, pp. 32–36; LEMPEL |
| 4 | [Scalar quantization and quantization noise](Scalar%20quantization%20and%20quantization%20noise.md) | SOURCE, pp. 37–43 |
| 5 | [Rate-distortion, transform, and predictive coding](Rate-distortion,%20transform,%20and%20predictive%20coding.md) | SOURCE, pp. 44–48 |

## Part B: channel coding

| Topic | Note | Main source |
|---:|---|---|
| 6 | [Discrete memoryless channels and capacity](Discrete%20memoryless%20channels%20and%20capacity.md) | CAP |
| 7 | [Typical sequences and the noisy-channel coding theorem](Typical%20sequences%20and%20the%20noisy-channel%20coding%20theorem.md) | NOISY |
| 8 | [Block codes, Hamming distance, and bounds](Block%20codes,%20Hamming%20distance,%20and%20bounds.md) | BLOCK |
| 9 | [Linear codes, syndromes, and Hamming codes](Linear%20codes,%20syndromes,%20and%20Hamming%20codes.md) | LINEAR, pp. 2–23 |
| 10 | [Finite fields and Reed-Solomon codes](Finite%20fields%20and%20Reed-Solomon%20codes.md) | LINEAR, pp. 24–40; RS |
| 11 | [Convolutional codes and trellis decoding](Convolutional%20codes%20and%20trellis%20decoding.md) | CONV |
| 12 | [LDPC codes and message-passing decoding](LDPC%20codes%20and%20message-passing%20decoding.md) | LDPC |
| 13 | [Code selection and parameter design](Code%20selection%20and%20parameter%20design.md) | DESIGN |

## Part C: information-theoretic cryptography

| Topic | Note | Main source |
|---:|---|---|
| 14 | [Perfect secrecy, one-time pads, and wiretap channels](Perfect%20secrecy,%20one-time%20pads,%20and%20wiretap%20channels.md) | CRYPTO, pp. 3–20 |
| 15 | [Secret-key agreement and information-theoretic authentication](Secret-key%20agreement%20and%20information-theoretic%20authentication.md) | CRYPTO, pp. 21–31 |
| 16 | [Secret sharing and secure function evaluation](Secret%20sharing%20and%20secure%20function%20evaluation.md) | CRYPTO, pp. 32–49; MPC |

## Scope

The course follows information from compression through reliable transmission
to unconditional security. Entropy supplies a common language, while source
codes, error-correcting codes and cryptographic protocols give it distinct
operational meanings.
