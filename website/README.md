# Website

The published site is generated without modifying the source course notes.

The `epl-sitegen` Rust program copies the course Markdown into temporary Zola
content, adds page metadata, rewrites local links, and produces `graph.json`.
Zola renders the note pages, while the `epl-graph` crate builds the interactive
graph as WebAssembly using `egui_graphs`.

Courses are discovered from `FAMILY/NUMBER/README.md`; for example,
`INMA/2470` is published as `LINMA2470`. This directory hierarchy is mirrored
by the graph's family and course controls, so adding a family does not require
changing the generator.

## Local build

Install Zola 0.22.1, Trunk 0.21.14, and the Rust WebAssembly target, then run:

```bash
cargo run --manifest-path website/Cargo.toml -p epl-sitegen -- \
  . website/content website/static/graph.json

(
  cd website/graph
  trunk build --release --public-url ./
)

mkdir -p website/static/graph
cp -R website/graph/dist/. website/static/graph/
zola --root website build
```

The result is written to `website/public`. Generated content, graph data,
WebAssembly bundles, and site output are ignored by Git.

## Visibility states

- **Enabled:** every topic in the course is visible.
- **Context:** only topics adjacent to an enabled topic are visible, in grey.
- **Hidden:** no topic from the course is visible.

Family controls apply the same state to all courses in that family. Hovering a
course or one of its topics highlights the other visible topics in that course.
