# CHANGELOG

## Status
Draft

## Version
0.2.4

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
CHELA-X Kernel repository history.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## 0.2.4
- Clarified the repository-local validation authority policy for host, approved CI, and Codex sandbox evidence.
- Corrected K2 status wording to `PASS WITH HOST VALIDATION PENDING` instead of treating the Codex sandbox linker limitation as a project blocker.
- Recorded the expected K2 test baseline of `58` tests as pending canonical host execution evidence.

## 0.2.3
- Added the additive K2 `kernel-domain::state` module for deterministic state snapshots, transition requests, transition outcomes, lifecycle guards, and workflow failure codes.
- Enforced CES-traced lifecycle validation for enterprise, workspace, project, organizational-unit, ownership, human, agent, decision, delegation, and workflow state changes.
- Added K2 traceability, API, implementation-plan, and validation documentation, including deferred-semantics notes where CES does not define a resume path.

## 0.2.2
- Accepted verified host validation evidence for K1.1 after the Codex sandbox could not access `/usr/bin/cc`, `/usr/bin/gcc`, or `/usr/bin/cargo` by absolute path.
- Recorded K1.1 validation as `PASS` with `38 passed`, `0 failed`, and `0 ignored` unit tests.
- Updated traceability evidence to use validated host results instead of the earlier sandbox linker blocker wording.
- Finalized the K1 domain API freeze as `FROZEN FOR K2 CONSUMPTION` and marked Kernel as ready for K2.

## 0.2.1
- Installed a stable Rust toolchain through `rustup` and verified direct toolchain execution.
- Completed K1.1 domain API corrections, including authorization evaluation-order, policy, workflow, and agent failure or recovery reference primitives.
- Replaced wide constructor signatures with validated spec structs for `AgentDefinition`, `DecisionRecord`, and `DelegationReference`.
- Froze the K1 public API baseline in `docs/API.md` and `docs/API-FREEZE.md`.
- Recorded initial K1.1 validation evidence before final host-verified gate closure.

## 0.2.0
- Replaced `kernel-bootstrap` with std-only `kernel-domain`.
- Added strongly typed identifiers for enterprise, ownership, authorization, decision, policy, workflow, and delegation records.
- Added ownership, identity, lifecycle, request, decision, authorization, agent, and delegation domain primitives.
- Added unit tests for identifier, ownership, lifecycle, request, decision, and delegation validation rules.
- Recorded K1 traceability and validation state, including the current Rust toolchain environment blocker.

## 0.1.0
- Initialized CHELA-X Kernel repository baseline.
- Added bootstrap documentation, traceability framework, and validation guidance.
- Added minimal Rust workspace scaffolding without business logic.
