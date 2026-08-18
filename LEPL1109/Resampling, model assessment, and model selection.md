# 15. Resampling, model assessment, and model selection

- Generalization error is empirical risk on an independent dataset. Resampling
  simulates held-out data from the available sample to assess models and select
  model family, features, or hyperparameters [SL-2, pp. 3-7].
- The workflow separates a final test set, resamples the available data for
  training/validation, selects the model, refits on all available data, and
  evaluates on test data [SL-2, p. 6]. The slides place preprocessing before
  these splits and do not develop leakage-safe fitting of transformations
  inside folds, so a later course should not assume mastery of pipelines or
  leakage prevention.
- Validation-set approach: random permutation and split, fit on training data,
  estimate MSE/error on held-out validation data. Randomization avoids a
  structurally biased split when ordering induces dependence [SL-2, pp. 8-11].
- Validation drawbacks: estimate variability across random splits and possible
  test-error overestimation because the model sees fewer training observations
  [SL-2, p. 12].
- LOOCV: fit \(N\) models, each excluding one observation, and average its
  one-point prediction loss. It uses almost all data and helps parameter
  selection but is expensive [SL-2, pp. 13-16].
- K-fold CV: randomize, partition into \(K\) folds, fit \(K\) times while each
  fold serves once as validation, and average fold errors. Typical \(K\) is 5
  or 10; it is much cheaper than LOOCV [SL-2, pp. 17-19].
- Bootstrap is named as another resampling method but is not covered in the
  data-science lectures because it appears in the statistics part [SL-2, p. 7;
  STAT, pp. 111-117].
