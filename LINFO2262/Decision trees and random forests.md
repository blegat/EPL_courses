# Decision trees and random forests

- A decision tree recursively tests attributes and predicts from leaves; paths
  correspond to interpretable rules (`TREE`, pp. 3–8).
- ID3 chooses splits by entropy reduction/information gain and carries an
  inductive bias toward short trees and high-gain attributes (`TREE`, pp. 9–19).
- C4.5 adds continuous thresholds, gain ratio, missing-value handling, and
  pruning to control overfitting (`TREE`, pp. 20–31).
- CART uses binary splits and impurity criteria and supports classification or
  regression trees (`TREE`, pp. 32–35).
- Random forests combine bootstrap samples and randomized feature subsets;
  averaging decorrelates trees and reduces variance (`TREE`, pp. 36–42).

## Related courses

- Pattern/rule view: [LINFO2364 — rule-based and pattern-based classification](../LINFO2364/Rule-based%20and%20pattern-based%20classification.md)
- Evaluation: [Classifier performance and model comparison](Classifier%20performance%20and%20model%20comparison.md)

