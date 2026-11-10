# Rust rocket computational engineering model

Mechy is an MIT licensed, original Rust computational engineering model
project. The first crate contains typed SI quantities
and elementary mechanics equations. The broader research workspace develops
the geometry kernel and an English HTML handbook alongside this project.

```powershell
cargo test -p rust-rocket-cem --offline
```

Next work: equation provenance records, validity intervals, machine readable
inputs and outputs, analytical reference cases, and geometry integration.
Simulation and test results will remain separate from predictions.

The project currently makes no propulsion performance or manufacturability
claim. See [model conventions](docs/model-conventions.md) and the
[architecture](docs/architecture.md) for review rules and module boundaries.
