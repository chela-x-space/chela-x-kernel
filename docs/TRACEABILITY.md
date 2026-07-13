# TRACEABILITY

## Status
Draft

## Version
0.2.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
Requirement traceability from CES and Program sources into CHELA-X Kernel.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

| Requirement Source | Requirement ID | Requirement Summary | Target Kernel Component | Implementation Status | Test Status | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| CES | `CES-B0-011#11.2-principle` | Identity persists across model, runtime, provider, and infrastructure changes. | `kernel-domain::identity::HumanIdentity`, `kernel-domain::identity::AgentIdentity` | PARTIAL | Defined, not run | `crates/kernel-domain/src/identity.rs`; tests: `identity_creates_valid_human_identity_ces_b0_011_2`; commit: pending |
| CES | `CES-B0-012#12.2-lifecycle` | Digital employees follow one governed lifecycle. | `kernel-domain::lifecycle::HumanLifecycle` | IMPLEMENTED | Defined, not run | `crates/kernel-domain/src/lifecycle.rs`; commit: pending |
| CES | `CES-B0-015#15.2-principle` | Authority exists only when granted by the organization. | `kernel-domain::authorization::AuthorityLevel` | PARTIAL | Not run | `crates/kernel-domain/src/authorization.rs`; commit: pending |
| CES | `CES-B0-022#normative-specification` | Decisions are canonical, auditable, and deterministically mappable. | `kernel-domain::decision::DecisionRecord` | PARTIAL | Defined, not run | `crates/kernel-domain/src/decision.rs`; tests: `decision_creates_valid_decision_record_ces_b0_022_1`; commit: pending |
| CES | `CES-B0-024.1#normative-specification` | High-risk actions must be authenticated, authorized, logged, and reviewable. | `kernel-domain::authorization::{AuthorizationSubject, AuthorizationTarget, PermissionReference}` | PARTIAL | Defined, not run | `crates/kernel-domain/src/authorization.rs`; commit: pending |
| CES | `CES-B0-024.6#normative-specification` | Security-relevant actions generate reconstructable audit records. | `kernel-domain::identifier::AuditEvidenceId` | PARTIAL | Not run | `crates/kernel-domain/src/identifier.rs`; commit: pending |
| CES | `CES-B0-025#normative-specification` | Enterprise, workspace, project, and OU hierarchy remain canonical. | `kernel-domain::ownership::{OwnershipPath, OrganizationalContext}` | IMPLEMENTED | Defined, not run | `crates/kernel-domain/src/ownership.rs`; tests: `ownership_accepts_valid_workspace_project_path_ces_b0_025_3`; commit: pending |
| CES | `CES-B0-026.3#normative-specification` | Tenant isolation denies scope widening across enterprise boundaries. | `kernel-domain::authorization::ScopeReference` | PARTIAL | Defined, not run | `crates/kernel-domain/src/authorization.rs`; tests: `authorization_scope_accepts_valid_project_scope_ces_b0_026_3`; commit: pending |
| CES | `CES-B0-026.5#normative-specification` | Authorization evaluation order is fixed and cannot be reordered locally. | TBD | NOT STARTED | Not run | `docs/TRACEABILITY.md` |
| CES | `CES-B0-026.6#normative-specification` | Authorization requests are immutable and decisions are deterministic. | `kernel-domain::request::AuthorizationRequestRecord` | IMPLEMENTED | Defined, not run | `crates/kernel-domain/src/request.rs`; tests: `request_creates_valid_request_record_ces_b0_026_6`; commit: pending |
| CES | `CES-B0-026.8#normative-specification` | Every authorization decision must produce stable audit evidence. | `kernel-domain::authorization::AuthorizationDecisionReference` | PARTIAL | Not run | `crates/kernel-domain/src/authorization.rs`; commit: pending |
| CES | `CES-B0-027.7#normative-specification` | Agent activation requires lifecycle, ownership, permission, and supervision prerequisites. | `kernel-domain::lifecycle::AgentLifecycle` | PARTIAL | Defined, not run | `crates/kernel-domain/src/lifecycle.rs`; tests: `agent_terminal_states_are_explicit_ces_b0_027_7`; commit: pending |
| CES | `CES-B0-027.15#normative-specification` | Agent state separates immutable identity from mutable operational state and supports replay. | `kernel-domain::identity::AgentIdentity`, `kernel-domain::agent::AgentDefinition` | PARTIAL | Defined, not run | `crates/kernel-domain/src/identity.rs`, `crates/kernel-domain/src/agent.rs`; tests: `identity_id_is_immutable_through_public_api_ces_b0_027_2`; commit: pending |
| CES | `CES-B0-027.18#normative-specification` | Critical agent failures trigger suspension or isolation before further privileged work. | TBD | NOT STARTED | Not run | `docs/TRACEABILITY.md` |
| CES | `CES-B0-027.19#normative-specification` | Recovery requires a plan, supervisor, and validation evidence. | TBD | NOT STARTED | Not run | `docs/TRACEABILITY.md` |
| CES | `CES-B0-028.7#normative-specification` | Explicit Deny overrides Permit and remains non-waivable where marked. | `kernel-domain::authorization::AuthorizationDecisionOutcome`, `kernel-domain::delegation::PolicyResultReference` | PARTIAL | Not run | `crates/kernel-domain/src/authorization.rs`, `crates/kernel-domain/src/delegation.rs`; commit: pending |
| CES | `CES-B0-028.9#normative-specification` | Policy evaluation order is deterministic and immutable per published set. | TBD | NOT STARTED | Not run | `docs/TRACEABILITY.md` |
| CES | `CES-B0-028.12#normative-specification` | Delegation and workflow must consume policy results rather than redefine policy authority. | `kernel-domain::delegation::AuthoritySourceReference` | PARTIAL | Not run | `crates/kernel-domain/src/delegation.rs`; commit: pending |
| CES | `CES-B0-029.4#normative-specification` | A delegator cannot delegate more authority than currently held. | `kernel-domain::delegation::DelegationReference` | PARTIAL | Defined, not run | `crates/kernel-domain/src/delegation.rs`; tests: `delegation_creates_valid_reference_ces_b0_029_1`; commit: pending |
| CES | `CES-B0-029.9#normative-specification` | Delegation chains are auditable, acyclic, and depth-bounded. | `kernel-domain::delegation::DelegationDepth` | PARTIAL | Defined, not run | `crates/kernel-domain/src/delegation.rs`; tests: `delegation_rejects_invalid_depth_ces_b0_029_9`; commit: pending |
| CES | `CES-B0-029.20#normative-specification` | Delegation remains policy-constrained and downstream workflow cannot redefine it. | `kernel-domain::delegation::{DelegationReference, AuthoritySourceReference}` | PARTIAL | Not run | `crates/kernel-domain/src/delegation.rs`; commit: pending |
| CES | `CES-B0-030.9#normative-specification` | Workflow transitions are deterministic and require valid upstream outcomes before running. | `kernel-domain::lifecycle::WorkflowState` | IMPLEMENTED | Defined, not run | `crates/kernel-domain/src/lifecycle.rs`; tests: `workflow_allows_documented_transition_ces_b0_030_9`; commit: pending |
| CES | `CES-B0-030.14#normative-specification` | Workflow retry and recovery are bounded and must revalidate upstream evidence. | TBD | NOT STARTED | Not run | `docs/TRACEABILITY.md` |
| CES | `CES-B0-030.17#normative-specification` | Decision-relevant workflow transitions must emit append-only audit evidence. | TBD | NOT STARTED | Not run | `docs/TRACEABILITY.md` |
| CES | `CES-B0-030.18#normative-specification` | Workflow must remain deterministic, tenant-isolated, and free of unbounded cycles. | `kernel-domain::lifecycle::WorkflowState` | PARTIAL | Defined, not run | `crates/kernel-domain/src/lifecycle.rs`; tests: `workflow_terminal_state_rejects_reactivation_ces_b0_030_9`; commit: pending |
| Program | `CX-PGM-008#repository-dependencies` | Kernel depends only on CES and AI Engineering OS and precedes Runtime. | `README.md`, `ARCHITECTURE.md`, `docs/BASELINE.md` | IMPLEMENTED | Not run | `ARCHITECTURE.md`, `docs/BASELINE.md`; commit: pending |
