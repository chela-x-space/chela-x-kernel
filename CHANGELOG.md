# CHANGELOG

## Status
Draft

## Version
0.2.1

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

## 0.2.1
- Installed a stable Rust toolchain through `rustup` and verified direct toolchain execution.
- Completed K1.1 domain API corrections, including authorization evaluation-order, policy, workflow, and agent failure or recovery reference primitives.
- Replaced wide constructor signatures with validated spec structs for `AgentDefinition`, `DecisionRecord`, and `DelegationReference`.
- Froze the K1 public API baseline in `docs/API.md` and `docs/API-FREEZE.md`.
- Recorded real K1.1 validation evidence and the remaining unit-test linker blocker.

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
