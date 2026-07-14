# CHANGELOG

## Status
Draft

## Version
0.4.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-15

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

## 0.4.0
- Added additive `kernel-domain::runtime` primitives for runtime identity, capability descriptors, lease records, heartbeat records, presence states, runtime health, immutable agent registrations, and deterministic in-memory agent registry indexing.
- Preserved the frozen K1-K3 surface while adding non-breaking runtime identifiers and additive agent getters needed by the registry layer.
- Added K4.1 CES-traceable tests for deterministic registration, duplicate detection, runtime and capability lookup, presence transitions, deregistration, lease validation, and heartbeat updates.
- Recorded K4.1 sandbox validation evidence: `cargo fmt`, `cargo check`, `cargo clippy`, `cargo doc`, and `cargo test --doc` pass; native `cargo test --all-targets` remains blocked in this environment because linker `cc` is unavailable.

## 0.3.0
- Added additive `kernel-domain::enforcement` primitives for deterministic authorization evaluation, policy or grant evidence resolution, authority checks, separation-of-duties checks, delegation bounds, evaluation traces, and decision construction.
- Preserved the frozen K1/K2 public surface while adding non-breaking getters and enforcement-specific input spec types.
- Added K3 CES-traceable tests for identity, scope, permission, explicit deny, authority, separation of duties, delegation bounds, deterministic outcomes, and trace order.
- Recorded K3 sandbox validation evidence: `cargo fmt`, `cargo check`, `cargo clippy`, `cargo doc`, and `cargo test --doc` passed; native `cargo test --all-targets` remains blocked in this environment because linker `cc` is unavailable.

## 0.2.5
- Finalized K2 canonical host validation with `cargo fmt`, `cargo check`, `cargo test`, `cargo clippy`, `cargo doc`, `cargo test --doc`, and `git diff --check` all passing.
- Recorded canonical host unit-test evidence as `58 passed`, `0 failed`, `0 ignored`.
- Promoted supported K2 traceability entries from `IMPLEMENTED` to `VERIFIED`.
- Marked K2 complete while preserving that K3 has not started.

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
