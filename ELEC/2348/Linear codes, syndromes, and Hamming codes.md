# Linear codes, syndromes, and Hamming codes

## Topics and results

- A binary linear $[n,k,d]$ code is a $k$-dimensional subspace of
  $\mathbb F_2^n$. A generator matrix encodes $u$ as $uG$ (`LINEAR`, pp. 3–9).
- A parity-check matrix satisfies $GH^T=0$ and defines the code by $Hx^T=0$.
- The minimum distance of a linear code is the minimum weight of a nonzero
  codeword.
- Cosets partition the ambient space. The syndrome $s=Hr^T$ identifies the
  coset of the error and supports table-based minimum-weight decoding
  (`LINEAR`, pp. 10–17).
- Binary Hamming codes have parameters $[2^m-1,2^m-m-1,3]$, correct one error
  and meet the Hamming bound (`LINEAR`, pp. 18–21).
- The dual code is the row space of $H$; simplex codes arise as duals of Hamming
  codes (`LINEAR`, pp. 22–23).
