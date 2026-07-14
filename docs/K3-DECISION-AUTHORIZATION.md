# K3 DECISION AUTHORIZATION

## Status
Draft

## Version
0.3.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Scope
K3 adds pure deterministic authorization enforcement to `kernel-domain` without adding infrastructure, persistence, networking, or runtime execution.

## CES Sources
- `CES-B0-015`
- `CES-B0-022.1`
- `CES-B0-022.3`
- `CES-B0-022.5`
- `CES-B0-022.6`
- `CES-B0-022.7`
- `CES-B0-022.11`
- `CES-B0-022.12`
- `CES-B0-022.13`
- `CES-B0-024.1`
- `CES-B0-024.2`
- `CES-B0-024.6`
- `CES-B0-026.1` to `CES-B0-026.8`
- `CES-B0-028.1`
- `CES-B0-028.3`
- `CES-B0-028.4`
- `CES-B0-028.5`
- `CES-B0-028.7`
- `CES-B0-028.8`
- `CES-B0-028.9`
- `CES-B0-028.12`
- `CES-B0-029.4`
- `CES-B0-029.5`
- `CES-B0-029.6`
- `CES-B0-029.9`
- `CES-B0-029.11`
- `CES-B0-029.13`
- `CES-B0-029.15`
- `CES-ADR-0005`

## Enforcement Boundary
- Pure evaluation only
- No database
- No SQLx
- No HTTP
- No Tokio
- No async runtime
- No external API
- No authentication provider
- No policy language parser
- No command execution
- No runtime delegation chain resolution

## Evaluation Order
The evaluator encodes the CES Chapter 26 order explicitly:
1. Verify principal identity and lifecycle
2. Verify tenant isolation and scope lineage
3. Resolve explicit denials
4. Resolve direct grants
5. Resolve inherited grants
6. Resolve requested permission match
7. Apply separation-of-duties conflicts
8. Emit final decision and evidence

Chapter 28 policy precedence and Chapter 22 authority requirements are consumed inside the permission-resolution stage rather than reordering Chapter 26.

## Explicit Deny
- Explicit deny is evaluated before direct or inherited grants.
- Non-waivable explicit deny is terminal for the current evaluation.
- `Indeterminate` never becomes permit.
- Default behavior remains deny-by-default.

## Scope Isolation
- Principal enterprise, principal operating scope, target scope, grant scope, policy scope, and delegation scope remain enterprise-bounded.
- Cross-tenant widening is denied before permission matching.
- Resource-level targets must remain consistent with resource-scoped references.

## Role Permission Distinction
- Roles remain reusable permission containers.
- Permissions remain atomic action or resource rights.
- Authority is evaluated separately and is never inferred from a role name alone.

## Authority
- K3 evaluates required authority level against supplied authority evidence.
- Authority checks stay explicit through `AuthorizationAuthorityRequirement`.
- Decision construction preserves authority ID, authority level, policy references, and rationale.

## Separation Of Duties
- K3 evaluates supplied conflict evidence deterministically.
- Unresolved requester or approver, executor or reviewer, or similar conflict evidence denies the request.
- No organizational graph lookup is performed.

## Delegation Bounds
- K3 consumes a supplied `AuthorizationDelegationBinding` only when the request principal is a delegated agent.
- The evaluator checks lifecycle, depth, scope preservation, delegated right compatibility, and local SoD evidence.
- Dynamic chain traversal remains deferred to K4.

## Decision Output
- `AuthorizationDecisionReference`
- `AuthorizationAuditEvidenceReference`
- optional `DecisionRecord`
- `AuthorizationEvaluationTrace`

All timestamps and stable identifiers are supplied by the caller. The evaluator does not create clocks or random identifiers internally.

When the decisive denial is not itself a matched policy effect, K3 still requires a supplied governing policy reference and policy-backed evidence set from the evaluation context. Non-policy denials therefore remain auditable without manufacturing a false permit or explicit-deny match.

Permit policies alone do not authorize access. A request still requires a matching grant or role-derived permission binding whose scope contains the target.

## Evaluation Trace
Every evaluation returns an ordered trace with:
- step
- pass or fail result
- decisive-step marker
- rejection reason where applicable
- matched evidence references

## Deferred Semantics
- dynamic delegation chain resolution
- full exception and waiver publication workflow
- classification graph resolution beyond supplied evaluation input
- infrastructure-backed audit persistence
- runtime execution and remote enforcement points

## K4 Relationship
K4 may consume K3 outputs but must not redefine policy precedence, explicit deny semantics, tenant boundaries, or authority ordering.
