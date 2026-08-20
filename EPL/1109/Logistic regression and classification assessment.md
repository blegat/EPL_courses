# 16. Logistic regression and classification assessment

### Logistic regression

- Linear classification partitions the feature space with hyperplane decision
  boundaries from discriminant functions [SL-2, p. 23].
- Least-squares classification has arbitrary category-ordering problems for
  more than two classes and lacks probability interpretation even for binary
  coding [SL-2, pp. 24-25].
- Logistic regression models
  $h_\beta(x)=P(Y=1\mid X=x)=S(\beta^Tx)$, where
  $S(t)=e^t/(1+e^t)$. Thresholding this probability gives a classifier
  [SL-2, pp. 26-30].
- Log-odds/logit is linear:
  $\log(P(Y=1\mid X)/P(Y=0\mid X))=\beta^TX$. At threshold 0.5 the
  equiprobability boundary is $\beta^Tx=0$; coefficient signs describe how
  features change positive-class probability [SL-2, pp. 28-32].
- Under independent Bernoulli conditional outcomes, MLE minimizes negative
  log-likelihood/binary cross-entropy
  $\sum_i[\log(1+e^{\beta^Tx_i})-y_i\beta^Tx_i]$. The objective is convex
  with gradient $\sum_i(S(\beta^Tx_i)-y_i)x_i$ [SL-2, pp. 33-35].
- Gradient descent, Newton-Raphson, and LBFGS are named solvers; scikit-learn
  fitting/prediction is demonstrated on digit `3` versus `not 3` [SL-2,
  pp. 35-38].

### Classification metrics and thresholds

- Accuracy = $1-$misclassification rate, estimated with CV; accuracy can be
  misleading for imbalanced classes because a constant majority classifier may
  score highly [SL-2, p. 39].
- True/false positives and negatives and the binary/multiclass confusion matrix;
  obtain out-of-fold predictions before computing it to avoid training-data
  optimism [SL-2, pp. 40-42].
- Precision $TP/(TP+FP)$, recall/sensitivity/TPR $TP/(TP+FN)$, their
  conditional-probability interpretations, and task-dependent tradeoff [SL-2,
  pp. 43-45].
- F1 is the harmonic mean of precision and recall [SL-2, p. 46].
- Changing the decision threshold trades precision against recall; PR curves
  visualize achievable pairs and threshold choice must reflect application
  costs [SL-2, pp. 47-49].
- ROC plots TPR versus FPR $=FP/(FP+TN)$; AUC near 1 indicates a strong
  ranking classifier, while random classification gives about 0.5. PR is
  preferred for rare positives/imbalanced classes or when false positives
  matter especially; ROC is suggested otherwise [SL-2, p. 50].

## Related courses

- Related LDACS1110 topic: [Integrated learning applications](../../DACS/1110/Integrated%20learning%20applications.md)
- Related LDACS1110 topic: [KL divergence, cross-entropy, and mutual information](../../DACS/1110/KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md)
- Related LDACS1110 topic: [Shannon entropy and conditional entropy](../../DACS/1110/Shannon%20entropy%20and%20conditional%20entropy.md)
