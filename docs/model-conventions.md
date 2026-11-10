# Equation review contract

Use SI at all public interfaces. The naming and unit framework is informed by
[ISO 80000-1:2022](https://www.iso.org/standard/76921.html),
[ISO 80000-4:2019](https://www.iso.org/standard/64975.html), and
[ISO 80000-5:2019](https://www.iso.org/standard/64976.html). The ISO 80000
series standardizes quantities and units; it does not validate a specific
engineering approximation.

Every model equation must record its source, symbol definitions and SI units,
assumptions, validity range, singularities, expected uncertainty, revision,
and at least one check against an independent reference. Mark each statement
as a conservation law, constitutive relation, empirical correlation, or design
rule. Do not merge predictions with simulation or measurement records.

The present crate implements only three elementary relations. Their use is
limited to the assumptions stated in the Rust documentation. No result from
this crate is a validated rocket engine prediction.
