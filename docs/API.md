# API

## Status
Draft

## Version
0.3.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
K1 public API review and consumption guidance for `kernel-domain`.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose And Scope
This document defines the frozen K1.1 public API baseline plus the additive K2 state API and additive K3 enforcement API for `kernel-domain`. It covers only pure domain types, constructors, invariants, lifecycle validation, deterministic authorization enforcement, and CES traceability. Runtime execution is out of scope.

## K1 API Stability Statement
The K1.1 domain API is frozen for K2 consumption. Breaking changes require either a CES-backed defect correction or an approved ADR.

## Module List
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

## Public Type Catalogue
- `identifier`: enterprise, workspace, project, organizational-unit, ownership, human, agent, decision, authority, principal, role, permission, scope, authorization request or decision, audit evidence, delegation, policy, workflow, namespace, version, and non-empty text types.
- `ownership`: `OwnershipScope`, `OwnershipSubject`, `OwnerReference`, `OwnershipPath`, `OrganizationalContext`.
- `identity`: `IdentityKind`, `HumanIdentity`, `AgentIdentity`, `IdentityReference`.
- `lifecycle`: enterprise, workspace, project, organizational-unit, ownership, human, agent, decision, delegation, and workflow lifecycle enums.
- `authorization`: subject, target, scope, role, permission, evaluation-order, decision, and audit-evidence references.
- `decision`: decision type, owner, subject, policy-set, rationale, context, and `DecisionRecord`.
- `agent`: agent definition, type, category, runtime, failure, and recovery references.
- `delegation`: delegator, delegate, beneficiary, scope, right, task, condition, authority-source, depth, and `DelegationReference`.
- `enforcement`: authorization evaluation context, grants, explicit denials, role-permission bindings, policy records, authority requirements, delegation bindings, deterministic trace results, and optional decision construction inputs.
- `policy`: `PolicyEffect`, evaluation-order, and audit-evidence references.
- `workflow`: retry, recovery, and audit-evidence references.
- `state`: lifecycle guard structs, state snapshots, transition request records, transition outcome records, reason or authority or evidence references, deterministic sequence values, workflow failure codes, and lifecycle validation functions.

## CES Source References
- `CES-B0-011#11.2-principle`
- `CES-B0-012#12.2-lifecycle`
- `CES-B0-022.1`, `CES-B0-022.5`, `CES-B0-022.6`
- `CES-B0-025.1` to `CES-B0-025.5`
- `CES-B0-026.1`, `CES-B0-026.3`, `CES-B0-026.5`, `CES-B0-026.6`, `CES-B0-026.8`
- `CES-B0-027.1`, `CES-B0-027.2`, `CES-B0-027.7`, `CES-B0-027.15`, `CES-B0-027.18`, `CES-B0-027.19`
- `CES-B0-028.7`, `CES-B0-028.9`, `CES-B0-028.12`
- `CES-B0-029.4`, `CES-B0-029.9`, `CES-B0-029.11`, `CES-B0-029.12`, `CES-B0-029.13`, `CES-B0-029.20`
- `CES-B0-030.9`, `CES-B0-030.13`, `CES-B0-030.14`, `CES-B0-030.17`, `CES-B0-030.18`

## Invariants
- Stable identifiers are immutable after construction.
- Ownership paths preserve the Chapter 25 hierarchy.
- Identity is distinct from operational state.
- Lifecycle types remain entity-specific and are not merged.
- Delegation remains bounded by policy and authorization.
- Workflow and policy references remain data-only and non-executable.

## Constructor And Parsing Rules
- Public constructors validate CES-required mandatory data.
- Identifier parsing rejects empty or malformed values.
- Constructors return `DomainError`; they do not panic for invalid input.
- Spec structs are used where constructor arity would otherwise exceed a safe public API.

## Trait Policy
- `Clone` is used where duplication is semantically safe.
- `Copy` is limited to scalar state and enum values.
- `Eq` and `Hash` are derived only for immutable value semantics.
- `Ord` is used only on identifier-like values where stable lexical ordering is acceptable.

## Mutation Policy
- Stable identifiers are private fields with accessor methods.
- No public setter mutates identity-bearing fields.
- Records and references contain no clocks, randomness, I/O, or execution behavior.
- Transition validation is exposed through pure functions and immutable outcome records.

## Runtime Consumers Expected In Later Phases
- K3 authorization and decision enforcement
- K4 agent and delegation runtime
- K5 workflow execution
- K6 audit, failure, and recovery controls

## Explicitly Out Of Scope
- persistence
- network APIs
- async runtime
- workflow execution
- policy evaluation
- external policy language parsing
- delegation resolution
- audit storage
- runtime execution

## Known Deferred Semantics
- delegation chain resolution beyond one supplied bound
- unsupported or higher-order exception and waiver publication workflows
- workflow retry or recovery execution
- enterprise reactivation semantics beyond explicit CES definition
- linker-dependent native test execution on this machine because `cc` is unavailable
