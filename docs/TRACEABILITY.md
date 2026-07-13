# TRACEABILITY

## Status
Draft

## Version
0.2.2

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
| CES | `CES-B0-011#11.2-principle` | Identity persists across model, runtime, provider, and infrastructure changes. | `kernel-domain::identity::{HumanIdentity, AgentIdentity}` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/identity.rs`; tests: `identity_creates_valid_human_identity_ces_b0_011_2`, `identity_id_is_immutable_through_public_api_ces_b0_027_2`; commits: `cb67e70..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-012#12.2-lifecycle` | Digital employees follow one governed lifecycle. | `kernel-domain::lifecycle::HumanLifecycle` | IMPLEMENTED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/lifecycle.rs`; tests: lifecycle suite in `crates/kernel-domain/src/lifecycle.rs`; commits: `cb67e70..5c94d2a`, host validation accepted 2026-07-14 |
| CES | `CES-B0-015#15.2-principle` | Authority exists only when granted by the organization. | `kernel-domain::authorization::AuthorityLevel` | PARTIAL | Passed (host verified; 38/0/0 suite where applicable) | `crates/kernel-domain/src/authorization.rs`; commits: `5c94d2a..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-022#normative-specification` | Decisions are canonical, auditable, and deterministically mappable. | `kernel-domain::decision::{DecisionRecord, DecisionRecordSpec}` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/decision.rs`; tests: `decision_creates_valid_decision_record_ces_b0_022_1`; commits: `5c94d2a..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-024.1#normative-specification` | High-risk actions must be authenticated, authorized, logged, and reviewable. | `kernel-domain::authorization::{AuthorizationSubject, AuthorizationTarget, PermissionReference}` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/authorization.rs`; tests: `authorization_request_principal_and_target_share_enterprise_traceability_k1`; commits: `5c94d2a..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-024.6#normative-specification` | Security-relevant actions generate reconstructable audit records. | `kernel-domain::authorization::AuthorizationAuditEvidenceReference` | PARTIAL | Covered by accepted host validation evidence | `crates/kernel-domain/src/authorization.rs`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-025#normative-specification` | Enterprise, workspace, project, and OU hierarchy remain canonical. | `kernel-domain::ownership::{OwnershipPath, OrganizationalContext}` | IMPLEMENTED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/ownership.rs`; tests: `ownership_accepts_valid_workspace_project_path_ces_b0_025_3`; commits: `cb67e70..5c94d2a`, host validation accepted 2026-07-14 |
| CES | `CES-B0-026.3#normative-specification` | Tenant isolation denies scope widening across enterprise boundaries. | `kernel-domain::authorization::ScopeReference` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/authorization.rs`; tests: `authorization_scope_accepts_valid_project_scope_ces_b0_026_3`; commits: `5c94d2a..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-026.5#normative-specification` | Authorization evaluation order is fixed and cannot be reordered locally. | `kernel-domain::authorization::{AuthorizationEvaluationStep, AuthorizationEvaluationOrderVersion}` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/authorization.rs`; tests: `authorization_evaluation_order_matches_ces_b0_026_5`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-026.6#normative-specification` | Authorization requests are immutable and decisions are deterministic. | `kernel-domain::request::AuthorizationRequestRecord`, `kernel-domain::authorization::AuthorizationDecisionReference` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/request.rs`, `crates/kernel-domain/src/authorization.rs`; tests: `request_creates_valid_request_record_ces_b0_026_6`; commits: `5c94d2a..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-026.8#normative-specification` | Every authorization decision must produce stable audit evidence. | `kernel-domain::authorization::{AuthorizationDecisionReference, AuthorizationAuditEvidenceReference}` | IMPLEMENTED | Covered by accepted host validation evidence | `crates/kernel-domain/src/authorization.rs`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-027.7#normative-specification` | Agent activation requires lifecycle, ownership, permission, and supervision prerequisites. | `kernel-domain::lifecycle::AgentLifecycle` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/lifecycle.rs`; tests: `agent_terminal_states_are_explicit_ces_b0_027_7`; commits: `5c94d2a`, host validation accepted 2026-07-14 |
| CES | `CES-B0-027.15#normative-specification` | Agent state separates immutable identity from mutable operational state and supports replay. | `kernel-domain::identity::AgentIdentity`, `kernel-domain::agent::{AgentDefinition, AgentDefinitionSpec}` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/identity.rs`, `crates/kernel-domain/src/agent.rs`; tests: `identity_id_is_immutable_through_public_api_ces_b0_027_2`; commits: `cb67e70..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-027.18#normative-specification` | Critical agent failures trigger suspension or isolation before further privileged work. | `kernel-domain::agent::AgentFailureReference` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/agent.rs`; tests: `agent_failure_reference_rejects_premature_recovery_eligibility_ces_b0_027_18`; commits: `f9c0330..f2d0e77`, host validation accepted 2026-07-14 |
| CES | `CES-B0-027.19#normative-specification` | Recovery requires a plan, supervisor, and validation evidence. | `kernel-domain::agent::AgentRecoveryReference` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/agent.rs`; tests: `agent_recovery_reference_requires_plan_and_evidence_ces_b0_027_19`; commits: `f9c0330..f2d0e77`, host validation accepted 2026-07-14 |
| CES | `CES-B0-028.7#normative-specification` | Explicit Deny overrides Permit and remains non-waivable where marked. | `kernel-domain::policy::PolicyEffect`, `kernel-domain::delegation::PolicyResultReference` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/policy.rs`, `crates/kernel-domain/src/delegation.rs`; tests: `policy_effect_distinguishes_permit_and_deny_ces_b0_028_7`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-028.9#normative-specification` | Policy evaluation order is deterministic and immutable per published set. | `kernel-domain::policy::{PolicyEvaluationStep, PolicyEvaluationOrderVersion}` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/policy.rs`; tests: `policy_evaluation_order_is_total_and_stable_ces_b0_028_9`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-028.12#normative-specification` | Delegation and workflow must consume policy results rather than redefine policy authority. | `kernel-domain::delegation::AuthoritySourceReference` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/delegation.rs`; tests: `delegation_creates_valid_reference_ces_b0_029_1`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-029.4#normative-specification` | A delegator cannot delegate more authority than currently held. | `kernel-domain::delegation::{AuthoritySourceReference, DelegationReference}` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/delegation.rs`; tests: `delegation_creates_valid_reference_ces_b0_029_1`; commits: `5c94d2a..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-029.9#normative-specification` | Delegation chains are auditable, acyclic, and depth-bounded. | `kernel-domain::delegation::{DelegationDepth, DelegationReferenceSpec}` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/delegation.rs`; tests: `delegation_rejects_invalid_depth_ces_b0_029_9`, `delegation_requires_explicit_policy_evidence_for_redelegation_ces_b0_029_9`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-029.20#normative-specification` | Delegation remains policy-constrained and downstream workflow cannot redefine it. | `kernel-domain::delegation::{DelegationReference, AuthoritySourceReference}` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/delegation.rs`; tests: `delegation_creates_valid_reference_ces_b0_029_1`; commits: `5c94d2a..f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-030.9#normative-specification` | Workflow transitions are deterministic and require valid upstream outcomes before running. | `kernel-domain::lifecycle::WorkflowState` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/lifecycle.rs`; tests: `workflow_allows_documented_transition_ces_b0_030_9`; commits: `5c94d2a`, host validation accepted 2026-07-14 |
| CES | `CES-B0-030.14#normative-specification` | Workflow retry and recovery are bounded and must revalidate upstream evidence. | `kernel-domain::workflow::{WorkflowRetryPolicyReference, WorkflowRecoveryReference}` | VERIFIED | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/workflow.rs`; tests: `workflow_retry_limit_rejects_zero_ces_b0_030_14`, `workflow_recovery_requires_path_reference_ces_b0_030_14`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-030.17#normative-specification` | Decision-relevant workflow transitions must emit append-only audit evidence. | `kernel-domain::workflow::WorkflowAuditEvidenceReference` | PARTIAL | Covered by accepted host validation evidence | `crates/kernel-domain/src/workflow.rs`; commits: `f9c0330`, host validation accepted 2026-07-14 |
| CES | `CES-B0-030.18#normative-specification` | Workflow must remain deterministic, tenant-isolated, and free of unbounded cycles. | `kernel-domain::lifecycle::WorkflowState` | PARTIAL | Passed (host verified; 38/0/0 suite) | `crates/kernel-domain/src/lifecycle.rs`; tests: `workflow_terminal_state_rejects_reactivation_ces_b0_030_9`; commits: `5c94d2a`, host validation accepted 2026-07-14 |
| Program | `CX-PGM-008#repository-dependencies` | Kernel depends only on CES and AI Engineering OS and precedes Runtime. | `README.md`, `ARCHITECTURE.md`, `docs/BASELINE.md` | IMPLEMENTED | Covered by repository validation evidence | `README.md`, `ARCHITECTURE.md`, `docs/BASELINE.md`; commits: `500286b..900041e`, host validation accepted 2026-07-14 |
