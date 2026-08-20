# Mathematical morphology

## Topics and results

- A binary image is equivalently a subset of the integer grid. Complement,
  translation, reflection, union, and intersection provide the set-theoretic
  language for morphology (`MORPH`, pp. 1–14).
- A structuring element defines the neighborhood and geometry probed by an
  operation; changing its size or shape changes the prior imposed on objects
  (`MORPH`, pp. 5–14).
- Dilation expands foreground through translated structuring elements, fills
  small gaps, and combines nearby components (`MORPH`, pp. 15–26).
- Erosion retains only locations where the structuring element fits inside the
  foreground, shrinking objects and removing small components (`MORPH`,
  pp. 27–32).
- Opening (erosion then dilation) removes small protrusions and disconnects thin
  bridges; closing (dilation then erosion) fills small holes and joins narrow
  gaps. Duality relates these operators (`MORPH`, pp. 33–42).
- For grayscale images, min/max neighborhood filters generalize erosion and
  dilation; their compositions yield grayscale openings, closings, and
  morphology-based feature extraction (`MORPH`, pp. 43–47).

## Related courses

- Region construction: [Edges, watersheds, and feature-based segmentation](Edges,%20watersheds,%20and%20feature-based%20segmentation.md)
