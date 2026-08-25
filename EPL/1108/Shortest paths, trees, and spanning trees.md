# Shortest paths, trees, and spanning trees

## Topics and results

- A tree is a connected acyclic graph; equivalently, it has $|V|-1$ edges and
  a unique simple path between every vertex pair (`NOTES`, pp. 99–102).
- Dijkstra's greedy algorithm computes single-source shortest paths when edge
  weights are nonnegative (`GRAPH`, pp. 17–24).
- Bellman–Ford handles negative weights by repeated relaxation and can detect
  a reachable negative cycle (`GRAPH`, pp. 25–30).
- The cut property justifies greedy minimum-spanning-tree algorithms
  (`GRAPH`, pp. 31–38).
- Running time depends on representation and data structures; adjacency lists
  suit sparse graphs (`GRAPH`, pp. 47–54).
