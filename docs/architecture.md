# Mechy: computational engineering model architecture

Mechy is an original Rust project for encoding engineering requirements,
explicit rules, simplified physical models, and evidence. Its long-term
research objective is parameter-driven rocket engine design, but no current
output is a validated engine design. Noyron RP is proprietary and is a
conceptual comparator, not source code used here.

The original core is offered under AGPL-3.0-only, with commercial licensing
available by separate written agreement. The commercial model centres on annual
maintenance, deployment and engineering verification services. See
[licensing](../LICENSING.md) and [commercial services](../COMMERCIAL.md).

## Intended data flow

```text
Requirements + materials + process constraints
                    ↓
Typed SI inputs and provenance
                    ↓
Analytical models + explicit design rules
                    ↓
Predicted quantities + geometry parameters
                    ↓
Rust geometry kernel and manufacturing checks
                    ↓
External simulation results (separate evidence)
                    ↓
Measured test records and model calibration (separate evidence)
```

Prediction, simulation, and measurement are separate versioned records.
Each record will carry the input revision, code revision, source of properties,
units, assumptions, numerical settings, validity range and uncertainty.
Calibration will create a new model revision rather than overwrite an earlier
prediction.

## Current implementation

`src/lib.rs` contains typed SI wrappers and three elementary relations:
constant-mass net force, average normal stress, and steady one-dimensional
mass flow. They are teaching and software-boundary examples, not a propulsion
performance model. The [equation contract](model-conventions.md) governs
future formulas. Integration with the pure Rust geometry workspace will occur
through an explicit crate dependency once both APIs are stable.

## Planned modules and gates

| Module | Output | Acceptance gate |
| --- | --- | --- |
| Requirements | typed input and constraints | rejected invalid units and impossible ranges |
| Thermochemistry | property and performance baselines | independent reference cases and source provenance |
| Thermal and flow | stated one-dimensional approximations | conservation and uncertainty checks |
| Structure and manufacture | limit states and geometry constraints | benchmark cases and process-specific review |
| Geometry | parameterized solid and export metadata | topology, dimensions, wall-thickness and resolution checks |
| External solvers | immutable job and result records | reproducible configuration and result import |
| Test feedback | calibrated model revision | traceable instruments, uncertainty and held-out validation |

No module is accepted from plausible-looking geometry or a single passing
example. The result must state its assumptions and evidence level.
