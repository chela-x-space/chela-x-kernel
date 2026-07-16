# K6 Implementation Plan

## Status
Draft

## Objective
Implement K6 Workflow Engine additively in small sequential phases under the frozen architecture.

## Phase 1
- Confirm workflow identity, retry, recovery, and audit-evidence references remain the current K6 seed surface in `kernel-domain`.
- Preserve all K1-K5 public APIs and reuse existing workflow, state, and event types.

## Phase 2
- Add immutable workflow-definition structures and pure validators.
- Bind definitions to stable identifiers, stable versions, ownership paths, and approved lifecycle maps.

## Phase 3
- Add immutable workflow-instance structures derived from approved definitions.
- Reuse `WorkflowStateSnapshot`, `StateSequence`, and additive consumed-evidence references.

## Phase 4
- Extend pure workflow transition-control validation over the approved `WorkflowState` map.
- Preserve existing guard ordering for approval, start, resume, failure, and recovery semantics.

## Phase 5
- Add workflow-stage coordination structures and validators.
- Keep stage coordination declarative, bounded, and distinct from K7 task semantics.

## Phase 6
- Add workflow authorization and policy-consumption validation APIs.
- Consume K3 policy, authorization, delegation, decision, and SoD outcomes by reference only.

## Phase 7
- Add workflow event-integration validators using completed K5 event, stream, and replay APIs.
- Preserve immutable facts, append-only ordering, and deterministic replay.

## Phase 8
- Add workflow failure, retry, and recovery validators using bounded retry and explicit recovery-path references.
- Preserve stable failure-code mapping and fresh revalidation requirements.

## Phase 9
- Add focused deterministic tests for each K6 slice.
- Run repository validation gates only after each additive phase compiles cleanly.

## Phase 10
- Update K6 traceability, validation evidence, and backlog state only after host validation passes.
- Do not proceed into K7 Task Engine work during K6 implementation.

## Final Determination
K6 implementation should proceed as sequential additive validation work over existing kernel-domain contracts without introducing infrastructure or architectural change.
