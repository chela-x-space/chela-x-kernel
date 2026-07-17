# API-FREEZE

## Status
Current

## Version
0.5.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-17

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
