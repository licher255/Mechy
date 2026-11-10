# Rust rocket computational engineering model

Mechy is an original Rust computational engineering model project. Its
original core is available under **AGPL-3.0-only**, with a separate commercial
licensing option by written agreement. See [licensing](LICENSING.md) and
[commercial licensing and services](COMMERCIAL.md).

The first crate contains typed SI quantities
and elementary mechanics equations. The broader research workspace develops
the geometry kernel and an English HTML handbook alongside this project.

The business model centres on annual maintenance, deployment and engineering
verification services. Users who comply with AGPL may use Mechy commercially
without purchasing a commercial licence or support. Service scope, fees and
any alternative licensing rights are agreed separately.

```powershell
cargo test -p rust-rocket-cem --offline
```

Next work: equation provenance records, validity intervals, machine readable
inputs and outputs, analytical reference cases, and geometry integration.
Simulation and test results will remain separate from predictions.

The project currently makes no propulsion performance or manufacturability
claim. See [model conventions](docs/model-conventions.md) and the
[architecture](docs/architecture.md) for review rules and module boundaries.

For contributions and the rights needed to support both licensing routes,
see [CONTRIBUTING.md](CONTRIBUTING.md).
