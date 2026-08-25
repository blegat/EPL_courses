# Complexity classes, reductions, and NP-completeness

## Topics and results

- Time complexity is measured on a specified machine model. Multitape and
  single-tape Turing machines simulate one another with polynomial overhead
  (`S12`, pp. 2–8).
- $P$ contains languages decidable in deterministic polynomial time. $NP$ has
  equivalent definitions through nondeterministic polynomial time and
  polynomially verifiable certificates (`S12`, pp. 9–17).
- A polynomial-time many-one reduction $A\leq_pB$ transforms instances while
  preserving yes/no answers; an algorithm for $B$ therefore gives one for $A$.
- A language is NP-hard if every language in $NP$ reduces to it, and
  NP-complete if it is also in $NP$ (`S12`, pp. 19–22).
- Cook–Levin encodes an accepting computation tableau as SAT, proving SAT
  NP-complete; reductions from SAT to 3SAT and from 3SAT to CLIQUE propagate
  NP-completeness (`S12`, pp. 20–31).
- The older notes also situate $P$, $NP$, $coNP$, $PSPACE$, $EXPTIME$, $RP$,
  $coRP$, $BPP$ and $ZPP$ (`NOTES`, pp. 87–105).
