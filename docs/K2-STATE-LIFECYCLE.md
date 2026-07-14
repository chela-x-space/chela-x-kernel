# K2-STATE-LIFECYCLE

## Status
Draft

## Version
0.1.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
K2 state and lifecycle implementation in `kernel-domain`.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Delivery Status
- Implementation Status: `COMPLETE`
- Architecture Review: `PASS`
- Host Validation: `PASS`
- Codex Sandbox Validation: `PARTIAL — native unit-test linking unavailable`
- Overall Status: `PASS`
- Ready for K3: `YES`

## Scope
K2 adds deterministic state snapshots, transition requests, transition validation, transition outcomes, lifecycle guards, terminal-state protection, failure-state representation, and transition references for the frozen K1 lifecycle types.

## CES Sources
- `CES-B0-012#12.2-lifecycle`
- `CES-B0-022.5`, `CES-B0-022.6`
- `CES-B0-025.1` to `CES-B0-025.5`
- `CES-B0-027.7`, `CES-B0-027.15`, `CES-B0-027.18`, `CES-B0-027.19`
- `CES-B0-029.4`, `CES-B0-029.9`, `CES-B0-029.11`, `CES-B0-029.12`, `CES-B0-029.13`, `CES-B0-029.20`
- `CES-B0-030.9`, `CES-B0-030.13`, `CES-B0-030.14`, `CES-B0-030.17`, `CES-B0-030.18`

## Lifecycle Types
- Enterprise, workspace, project, organizational-unit, ownership, human, agent, decision, delegation, and workflow lifecycle transitions are validated in `kernel-domain::state`.
- Agent snapshots preserve identity version separately from mutable lifecycle state.
- Workflow snapshots preserve definition version separately from mutable lifecycle state.

## Implemented Transition Rules
- Explicit workflow and delegation transition maps are enforced directly from CES.
- Human lifecycle progression is enforced as the ordered Chapter 12 sequence.
- Enterprise, workspace, project, organizational-unit, and ownership transitions are limited to the CES-defined command and terminal semantics without adding runtime behavior.
- Agent activation, suspension, recovery, retirement, and deletion require CES-backed lifecycle guards.
- Decision approval, rejection, execution, supersession, and archival require CES-backed rationale, authority, evidence, and successor guards.

## Deferred Semantics
- Enterprise reactivation from `Suspended` is preserved as deferred because CES-B0-025.1 defines the suspended state and commands but does not define a resume transition.
- K2 does not infer undocumented lifecycle edges beyond the explicit CES sequence or command-backed progression.
- K2 does not resolve policy evaluation, authorization evaluation, delegation execution, or workflow execution behavior.

## Out Of Scope
- workflow execution
- agent execution
- persistence
- schedulers
- event buses
- HTTP or runtime infrastructure

## K1 Relationship
- K1 public lifecycle enums remain frozen and unchanged.
- K2 adds a new `state` module and re-exports additive public state primitives from `kernel-domain`.
- No frozen K1 type was removed or renamed.

## K3 Consumers
- K3 authorization and decision enforcement may consume K2 transition outcomes and guard inputs.
- Later phases may consume K2 state snapshots and transition records for replay, audit, and runtime orchestration.
