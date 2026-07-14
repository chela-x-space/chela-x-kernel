# API-FREEZE

## Status
Draft

## Version
0.3.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
K1 public API compatibility governance for `kernel-domain`.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Freeze Name
`K1 Domain API Baseline`

## Status Statement
`FROZEN FOR K2/K3 CONSUMPTION`

## Validation State
- K1.1 validation status: `PASS`
- Validation source: accepted host verification for `/home/chela-x/chela-x-kernel`
- Unit-test result: `38 passed`, `0 failed`, `0 ignored`
- Ready for K2: `YES`

## Crate
- name: `kernel-domain`
- version: `0.1.0`

## Public Modules
- `agent`
- `authorization`
- `decision`
- `delegation`
- `enforcement`
- `errors`
- `identifier`
- `identity`
- `lifecycle`
- `ownership`
- `policy`
- `request`
- `state`
- `workflow`

## Public Types
- stable identifiers and validated text or version values
- ownership path and organizational context values
- identity and lifecycle enums or references
- authorization, decision, agent, delegation, policy, and workflow pure reference types
- deterministic enforcement input, output, trace, and spec types
- state snapshots, transition request or outcome records, lifecycle guard structs, workflow failure codes, and lifecycle validation functions
- spec structs for `AgentDefinition`, `DecisionRecord`, `DelegationReference`, `AuthorizationGrantRecord`, and `AuthorizationPolicyRecord`

## Intentionally Private Types
- internal validation helpers inside `identifier.rs`
- private record fields across all public structs

## Known Partial Requirements
- `CES-B0-011#11.2-principle`
- `CES-B0-015#15.2-principle`
- `CES-B0-022#normative-specification`
- `CES-B0-024.1#normative-specification`
- `CES-B0-024.6#normative-specification`
- `CES-B0-026.3#normative-specification`
- `CES-B0-027.7#normative-specification`
- `CES-B0-027.15#normative-specification`
- `CES-B0-029.4#normative-specification`
- `CES-B0-029.11#normative-specification`
- `CES-B0-029.12#normative-specification`
- `CES-B0-029.13#normative-specification`
- `CES-B0-030.13#normative-specification`
- `CES-B0-029.20#normative-specification`
- `CES-B0-030.17#normative-specification`
- `CES-B0-030.18#normative-specification`

## Compatibility Rules
- Breaking public API changes after this freeze require a documented CES-backed defect correction or an approved ADR.
- Non-breaking additions remain allowed only when traced to an existing CES or Program requirement.
- Constructor strengthening that closes an invalid state is allowed only when CES already prohibits that state.

## Allowed Change Categories
- additive getters
- additive non-breaking reference types
- stronger validation that enforces existing CES rules
- documentation and traceability corrections

## Prohibited Breaking Changes
- renaming or removing frozen public types without approved authority
- widening mutable access to stable identity fields
- replacing strong domain types with raw `String`
- introducing runtime execution behavior into `kernel-domain`
