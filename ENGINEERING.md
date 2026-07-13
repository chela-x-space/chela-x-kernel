# ENGINEERING

## Status
Draft

## Version
0.2.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
Implementation work, validation, and commit discipline in CHELA-X Kernel.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose
Define repository-local engineering rules while the Kernel moves from K0 bootstrap into K1 domain modeling.

## Engineering Rules
- Work MUST begin with baseline inspection and traceability review.
- Changes MUST preserve the frozen dependency direction inherited from CHELA-X Program.
- K1 work MUST remain domain primitives, validation, and documentation updates only.
- Runtime behavior, network layers, database layers, schedulers, event buses, and agent runtimes MUST NOT be introduced in K0.
- Runtime behavior, network layers, database layers, schedulers, event buses, and agent runtimes MUST NOT be introduced in K1.
- External dependencies MUST NOT be added unless strictly required for workspace bootstrap.
- Commit scope MUST stay small and explainable.

## Validation Rules
- Run available format, build, lint, and test commands before completion.
- Record blocked validation honestly when the environment lacks required tooling.
- Repository clean checks MUST pass before final completion.

## Commit Rules
- Prefer small commits grouped by baseline, documentation, and bootstrap concerns.
- Do not bundle unrelated work.

## References
- [AGENTS.md](./AGENTS.md)
- [ARCHITECTURE.md](./ARCHITECTURE.md)
- [docs/TRACEABILITY.md](./docs/TRACEABILITY.md)
- [docs/VALIDATION.md](./docs/VALIDATION.md)
