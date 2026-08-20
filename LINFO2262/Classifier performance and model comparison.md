# Classifier performance and model comparison

- Test accuracy estimates future performance only when test observations are
  representative and untouched by training/model choice (`EVAL`, pp. 3–12).
- A binomial model supplies uncertainty intervals for a fixed classifier's error
  rate (`EVAL`, pp. 9–15; `PROB`, pp. 2–19).
- Paired predictions support tests comparing two classifiers on the same cases
  rather than treating errors as independent (`EVAL`, pp. 16–23).
- Comparing learning algorithms requires repeated train/test sampling or
  $K$-fold cross-validation (`EVAL`, pp. 24–32).
- Hyperparameter tuning must be nested inside evaluation; reusing validation
  performance as final test performance creates optimistic bias (`EVAL`,
  pp. 30–37).

## Related courses

- Model family: [Decision trees and random forests](Decision%20trees%20and%20random%20forests.md)
- Regression counterpart: [LSTAT2120 — variable and model selection](../LSTAT2120/Variable%20and%20model%20selection.md)
- Prerequisite: [LEPL1109 — resampling, model assessment, and model selection](../LEPL1109/Resampling,%20model%20assessment,%20and%20model%20selection.md)
- Mining evaluation: [LINFO2364 — probabilistic classification and model evaluation](../LINFO2364/Probabilistic%20classification%20and%20model%20evaluation.md)
