# Mechy licensing

Copyright (c) 2026 licher.

Starting with the licensing change dated 2026-11-10, the original Mechy core
in this revision is offered under **GNU Affero General Public License,
version 3 only** (`AGPL-3.0-only`). The complete, unmodified licence text is
in [LICENSE](LICENSE), obtained from the
[GNU official text](https://www.gnu.org/licenses/agpl-3.0.txt).

## Two licensing routes

1. **Open source:** use, modify and distribute the covered code under AGPL-3.0.
   Commercial use is allowed. No payment to the maintainer is required merely
   because the user is a business. Source provision and other obligations are
   determined by the licence, including section 13 for modified versions that
   support remote network interaction.
2. **Commercial:** obtain different rights for code the licensor is authorized
   to license, through a separate signed agreement. See [COMMERCIAL.md](COMMERCIAL.md).
   This document does not itself grant a commercial licence, an AGPL exception,
   or a right to keep covered modifications proprietary.

The Cargo SPDX field identifies the public open-source grant. It deliberately
does not list MIT, LGPL, or a freely selectable commercial alternative.
Buying support alone does not change the applicable software licence.

AGPL does not require submitting a pull request to this repository. Its source
provision duties concern recipients and qualifying remote users. We encourage
upstream contributions and address rights for commercial relicensing separately
in [CONTRIBUTING.md](CONTRIBUTING.md).

## Existing MIT grants

The licence change does not revoke rights already granted under MIT. The
previous development revision,
[`c1bb3877014fe95ea6253817bbf64ef60c942fbb`](https://github.com/licher255/Mechy/tree/c1bb3877014fe95ea6253817bbf64ef60c942fbb),
and earlier MIT-licensed revisions remain available under their original terms.
The historical text is preserved in
[docs/licenses/MIT-before-2026-11-10.txt](docs/licenses/MIT-before-2026-11-10.txt).
It is not an additional MIT grant for new contributions or modifications
published only under the current licensing policy.

## Third-party material and outputs

Third-party material retains its own copyright notices and licence terms.
Mechy's commercial agreement cannot waive another copyright holder's terms
unless that holder has separately authorized it. The current crate declares
no third-party dependencies; future dependencies require review for both
the AGPL distribution and any proposed commercial distribution.

In particular, the parent research project's Apache-2.0-derived geometry ports
retain their Apache notices. Third-party GPL/AGPL code must not be represented
as commercially relicensable merely because it is compatible with Mechy's
open-source distribution.

Using Mechy does not by itself place customer inputs, geometry, calculation
results or manufactured products under AGPL. Outputs containing protected
code, templates or other licensed material require assessment of that material.
See the [GNU FAQ on program output](https://www.gnu.org/licenses/gpl-faq.html#WhatCaseIsOutputGPL).
