# API-FREEZE

## Status
Current

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-19

## Applies To
Frozen public API governance for `kernel-domain`, including the K6 workflow-engine surface.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Freeze Name

`K6 Workflow Engine Domain API`

## Status Statement

`FROZEN FOR DOWNSTREAM CONSUMPTION`

## Validation State

- Host validation status: `PASS`
- Validation source: accepted host verification for `/home/chela-x/chela-x-kernel`
- Unit-test baseline: `595 passed`, `0 failed`, `0 ignored`
- Doc-test baseline: `0 passed`, `0 failed`
- Architecture Freeze: `PRESERVED`

## Scope Of The Freeze

Frozen public K6 workflow types exported from `crates/kernel-domain/src/lib.rs`:

- workflow foundation types
- workflow definition and instance types
- workflow transition-control types
- workflow step-coordination types
- workflow authorization-integration types
- workflow event-integration types
- workflow failure-and-recovery types
- workflow-related `DomainError` variants

Private helpers and internal validation functions that are not publicly re-exported are not frozen by this document.

## Compatibility Guarantees

- Additive compatibility with K1 is preserved.
- K2 lifecycle semantics are unchanged.
- K3 authorization semantics are reused, not duplicated.
- K5 event-envelope semantics are reused, not duplicated.
- Existing public K1-K5 exports remain usable.

## Explicit Non-Features

- No runtime scheduler
- No executor
- No persistence
- No event bus
- No async runtime
- No network
- No workflow mutation performed by step coordination, authorization integration, event integration, or recovery decision layers

## Change Policy

Any breaking K6 public API or semantic change requires an approved ADR.

Allowed non-breaking changes:

- documentation corrections
- additive public getters
- additive non-breaking workflow reference types
- stronger validation only when it enforces already-approved CES or frozen-K2 semantics without changing accepted valid states

Prohibited changes without approved ADR:

- renaming or removing frozen K6 public types
- changing K2 lifecycle semantics
- duplicating K3 authorization semantics
- duplicating K5 event-envelope semantics
- introducing runtime infrastructure behavior into `kernel-domain`

## K7 Task Domain API

### Status Statement

`FROZEN FOR NEXT-MILESTONE CONSUMPTION`

### K7 Review State

- Implementation status: `COMPLETE`
- Architecture review: `PASSED`
- Public API inventory: `RECORDED`
- Compatibility status: `PRESERVED`
- Native verification status: `PASSED`
- Architecture Freeze: `PRESERVED`

### K7 Scope

The K7 task-domain API covers:

- identity and references
- definition and instance modeling
- ownership and assignment
- priority and readiness
- lifecycle and state transitions
- dependency coordination
- completion, failure, evidence, and outcome decisions
- K7-009 integration and conformance coverage

### K7 Freeze Conditions

- K7 implementation is complete and ready for downstream consumption in the next milestone.
- Public API inventory is recorded in `docs/API.md`.
- Architecture review passed without redesign, dependency-direction change, or ADR requirement.
- K1-K6 compatibility and K7 additive compatibility are preserved.
- Native verification passed on the primary machine on Saturday, July 18, 2026.
- `cargo test --workspace --all-targets` result: `765 passed`, `0 failed`, `0 ignored`, `0 measured`, `0 filtered out`, exit code `0`.

### K7 Non-Features

- No scheduler
- No executor
- No worker dispatch
- No persistence
- No repository
- No task runtime facade
- No event publication
- No retry or timeout engine

## K8 Execution Engine Domain API

### Status Statement

`FROZEN FOR NEXT-MILESTONE CONSUMPTION`

### K8 Review State

- Implementation status: `COMPLETE`
- Architecture review: `PASSED`
- Compile validation status: `PASSED`
- Native verification status: `PASSED`
- Public API inventory: `RECORDED`
- Compatibility status: `K1-K7 PRESERVED; K8 ADDITIVE`
- Architecture Freeze: `PRESERVED`

### K8 Scope

The frozen additive K8 execution-domain API covers:

- `ExecutionSessionId`
- `ExecutionRequest`
- `ExecutionContext`
- `ExecutionSession`
- `ExecutionOutcome`
- `ExecutionTermination`
- `ExecutionEvidenceBinding`
- `ExecutionRetryEligibilityDecision`
- `ExecutionAuditReference`

### K8 Freeze Conditions

- K8 implementation is complete and ready for downstream consumption in the next milestone.
- Public API inventory is recorded in `docs/API.md`.
- Architecture review passed without redesign, dependency-direction change, or ADR requirement.
- K1-K7 API compatibility is preserved and K8 public API is additive only.
- Native verification passed on the primary machine on Saturday, July 18, 2026.
- `cargo test --workspace --all-targets` result: `790 passed`, `0 failed`, `0 ignored`, `0 measured`, `0 filtered out`, exit code `0`.

### K8 Non-Features

- No scheduler
- No worker dispatch
- No queue
- No process spawning
- No network transport
- No filesystem or database persistence
- No event publication
- No memory persistence
- No automatic retry execution
- No automatic timeout execution
- No task lifecycle mutation

## K9 Enterprise Memory Domain API

### Status Statement

`FROZEN FOR K10 CONSUMPTION`

### K9 Review State

- Implementation status: `COMPLETE`
- Architecture review: `PASSED`
- Compile validation status: `PASSED`
- Native verification status: `PASSED`
- Public API inventory: `RECORDED`
- Compatibility status: `K1-K8 PRESERVED; K9 ADDITIVE`
- Architecture Freeze: `PRESERVED`

### K9 Scope

The additive K9 memory-domain API currently covers:

- `MemoryRecordId`
- `MemoryRecordReference`
- `MemoryRecord`
- `MemoryProvenance`
- `MemoryClassification`
- `MemoryRelationship`
- `MemoryRetentionPolicyReference`
- `MemoryCaptureRequest`
- `MemoryCaptureDecision`
- `MemoryRetentionDecision`
- `MemoryRelationshipRequest`
- `MemoryRetrievalRequest`
- `MemoryRetrievalResult`
- `MemoryQuery`
- `MemoryQueryResult`
- `MemoryProjection`
- `WorkflowMemoryProjection`
- `TaskMemoryProjection`
- `ExecutionMemoryProjection`
- `RuntimeMemoryProjection`
- `MemoryAuditReference`
- `MemoryRejectionReason`

### K9 Freeze Conditions

- K9 implementation is complete in `kernel-domain`.
- Public API inventory is recorded in `docs/API.md`.
- Architecture review passed without redesign, dependency-direction change, or ADR requirement.
- K1-K8 API compatibility is preserved and K9 public API is additive only.
- Native verification passed on the primary host on Saturday, July 18, 2026.
- `cargo test --workspace --all-targets` result: `827 passed`, `0 failed`, `0 ignored`, `0 measured`, `0 filtered out`, exit code `0`.

### K9 Change Policy

Breaking K9 public API or semantic changes after freeze require:

- Approved ADR
- Compatibility Review

### K9 Non-Features

- No application service
- No runtime orchestration
- No scheduler, worker dispatch, or queue
- No network transport
- No filesystem or database persistence
- No search, vector, or embedding infrastructure
- No API Gateway or Studio implementation

## K10 API Gateway

### Status Statement

`FROZEN FOR K11 CONSUMPTION`

### K10 Review State

- Planning status: `COMPLETE`
- Architecture review: `PASSED`
- Implementation status: `COMPLETE`
- Compile validation status: `PASSED`
- Native verification status: `PASSED`
- Public API inventory: `RECORDED`
- Compatibility status: `K1-K9 PRESERVED; K10 ADDITIVE`
- Architecture Freeze: `PRESERVED`

### K10 Scope

The additive K10 gateway API currently covers:

- `GatewayApiVersion`
- `GatewayOperationKind`
- `GatewayOperationReference`
- `GatewayAuthenticationContext`
- `GatewayAuthorizationBinding`
- `GatewayRequestContext`
- `GatewayRequestEnvelope`
- `GatewayCommandPayload`
- `GatewayCommandRequest`
- `GatewayCommandResponse`
- `GatewayQueryPayload`
- `GatewayQueryRequest`
- `GatewayQueryResponse`
- `GatewayResponseEnvelope`
- `GatewayError`
- `GatewayErrorCode`
- `GatewayResult`
- `GatewayProtocol`
- `GatewayRateGovernanceReference`
- `GatewayStatusSnapshot`
- `GatewayAuditReference`

## K11 Studio Integration

### Status Statement

`FROZEN FOR K12 CONSUMPTION`

### K11 Review State

- Planning status: `COMPLETE`
- Architecture review: `PASSED`
- Implementation status: `COMPLETE`
- Compile validation status: `PASSED`
- Native verification status: `PASSED`
- Public API inventory: `RECORDED`
- Compatibility status: `K1-K10 PRESERVED; K11 ADDITIVE`
- Architecture Freeze: `PRESERVED`

### K11 Scope

The additive K11 Studio API currently covers:

- `StudioApiVersion`
- `StudioViewKind`
- `StudioViewReference`
- `StudioNavigationReference`
- `StudioFilterReference`
- `StudioSortReference`
- `StudioTimeRange`
- `StudioFilterContext`
- `StudioSelectionContext`
- `StudioAuditReference`
- `StudioStatusSnapshot`
- `StudioError`
- `StudioErrorCode`
- `StudioResult`
- `StudioViewProjection`
- `StudioViewRequest`
- `StudioViewResponse`
- `StudioRequestEnvelope`
- `StudioResponseEnvelope`
- `StudioTopViewProjection`
- `StudioAttentionState`
- `StudioDigitalTwinProjection`
- `StudioRuntimeProjection`
- `StudioWorkflowProjection`
- `StudioTaskProjection`
- `StudioEventTimelineProjection`
- `StudioMemoryProjection`
- `StudioAuditProjection`
- `StudioRevenueReferenceProjection`

## K12 Application Integration

### Status Statement

`FROZEN FOR K13 CONSUMPTION`

### K12 Review State

- Planning status: `COMPLETE`
- Architecture review: `PASSED`
- Implementation status: `COMPLETE`
- Compile validation status: `PASSED`
- Native verification status: `PASSED`
- Public API inventory: `RECORDED`
- Compatibility status: `K1-K11 PRESERVED; K12 ADDITIVE`
- Architecture Freeze: `PRESERVED`

### K12 Scope

The additive K12 Application Integration API currently covers:

- `ApplicationApiVersion`
- `ApplicationIdentity`
- `ApplicationIdentityKind`
- `ApplicationCapabilityReference`
- `ApplicationCapabilityDeclaration`
- `ApplicationRequestId`
- `ApplicationAuditReference`
- `ApplicationRequestContext`
- `ApplicationSessionReference`
- `ApplicationSessionStatusReference`
- `ApplicationViewIntent`
- `ApplicationCommandIntent`
- `ApplicationQueryIntent`
- `ApplicationIntentKind`
- `ApplicationRequestEnvelope`
- `ApplicationResponseKind`
- `ApplicationResponseStatusReference`
- `ApplicationResponsePayload`
- `ApplicationResponseEnvelope`
- `ApplicationError`
- `ApplicationErrorCode`
- `ApplicationResult`
- `ApplicationDependencyCompatibilityReference`
- `ApplicationStatusSnapshot`
- `ApplicationValidationStatus`
- `StudioCommandRequest`
- `StudioCommandResponse`

### K12 Freeze Conditions

- K12 implementation is complete in `kernel-application`.
- Public API inventory is recorded in `docs/API.md`.
- Architecture review passed under accepted `ADR-0001`.
- K1-K11 compatibility is preserved and K12 public API is additive only.
- Primary-host native verification passed on Sunday, July 19, 2026.
- Current authoritative workspace-native result includes `kernel-application: 23 passed` and `TOTAL: 917 passed`, `FAILED: 0`.

### K12 Change Policy

Breaking K12 public API or semantic changes after freeze require:

- approved ADR
- compatibility review
- explicit human authorization

## K13 Service Integration

### Status Statement

`FROZEN FOR K14 CONSUMPTION`

### K13 Review State

- Planning status: `COMPLETE`
- ADR status: `ACCEPTED`
- Architecture review: `PASSED`
- Workspace integration status: `PASSED`
- Implementation status: `COMPLETE`
- Compile validation status: `PASSED`
- Native verification status: `PASSED`
- Public API inventory: `RECORDED`
- Compatibility status: `K1-K12 PRESERVED; K13 ADDITIVE`
- Architecture Freeze: `PRESERVED`

### K13 Scope

The additive K13 Service Integration API currently covers:

- `ServiceApiVersion`
- `ServiceIdentity`
- `ServiceIdentityKind`
- `ServiceCapabilityReference`
- `ServiceCapabilityDeclaration`
- `ServiceIntentKind`
- `ServiceRequestId`
- `ServiceRequestContext`
- `ServiceCommandIntent`
- `ServiceQueryIntent`
- `ServiceResponseKind`
- `ServiceResponseStatusReference`
- `ServiceResponseEnvelope`
- `ServiceError`
- `ServiceErrorCode`
- `ServiceResult`
- `ServiceDependencyCompatibilityReference`
- `ServiceStatusSnapshot`
- `ServiceValidationStatus`
- `SERVICE_COMMAND_CAPABILITY`
- `SERVICE_QUERY_CAPABILITY`

### K13 Freeze Conditions

- K13 implementation is complete in `kernel-service`.
- Public API inventory is recorded in `docs/API.md`.
- Architecture review passed under accepted `ADR-0002`.
- K1-K12 compatibility is preserved and K13 public API is additive only.
- Primary-host native verification passed on Sunday, July 19, 2026.
- `cargo test --workspace --all-targets` result: `kernel-domain: 827 passed`, `kernel-gateway: 34 passed`, `kernel-studio: 16 passed`, `kernel-application: 23 passed`, `kernel-service: 17 passed`, `TOTAL: 917 passed`, `FAILED: 0`.
- Root workspace is the single workspace authority; `crates/kernel-service` no longer declares a crate-local `[workspace]` and no crate-local `Cargo.lock` remains.

### K13 Change Policy

Breaking K13 public API or semantic changes after freeze require:

- approved ADR
- compatibility review
- explicit human authorization
