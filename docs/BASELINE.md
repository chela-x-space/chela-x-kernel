# BASELINE

## Status
Draft

## Version
0.1.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-13

## Applies To
Frozen upstream baselines consumed by CHELA-X Kernel K0.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Frozen Baselines
- AI Engineering OS: `v1.0` `Freeze`
- CHELA-X CES: `Book 0 RC1 Frozen`
- CES Tag: `book0-rc1`
- CES Commit: `6f131072b0ef0e871b929a67ab558409acca4ed6`
- CHELA-X Program: initialized and validated baseline
- CHELA-X Library Commit: `c7a9600`

## Mandatory Governance Files
- `/home/chela-x/ai-engineering-os/ENGINEER.md`
- `/home/chela-x/ai-engineering-os/ARCHITECTURE.md`
- `/home/chela-x/ai-engineering-os/DECISION.md`
- `/home/chela-x/ai-engineering-os/WORKFLOW.md`
- `/home/chela-x/ai-engineering-os/STANDARDS.md`
- `/home/chela-x/ai-engineering-os/DOCUMENT-METADATA.md`
- `/home/chela-x/ai-engineering-os/REPOSITORY-MANIFEST.md`
- `/home/chela-x/chela-x-ces/AGENTS.md`
- `/home/chela-x/chela-x-ces/ENGINEERING.md`
- `/home/chela-x/chela-x-ces/BOOK0-RC1.md`
- `/home/chela-x/chela-x-program/DEPENDENCY-GRAPH.md`
- `/home/chela-x/chela-x-library/docs/AGENT-USAGE.md`

## Applicable ADRs
- `CES-ADR-0001` Book 0 chapter reconciliation
- `CES-ADR-0002` repository governance
- `CES-ADR-0003` policy before delegation
- `CES-ADR-0004` Book 0 completion policy
- `CES-ADR-0005` Enterprise Core architecture
- `CES-ADR-0006` legacy Book normalization policy
- `CES-ADR-0007` legacy standards normalization policy
- `CES-ADR-0008` metadata versioning policy

## Applicable Standards
- `CES-STD-0001` Enterprise Entity Standard
- `CES-STD-0002` Enterprise Identity Standard
- `CES-STD-0003` Enterprise Object Model
- `CES-STD-0004` Enterprise Laws
- `CES-STD-0005` Enterprise Layer Architecture
- `CES-STD-0006` Document Standard
- `CES-STD-0007` Architecture Standard
- `CES-STD-0008` Coding Standard

## Factory Jobs
- `FACTORY-001`: historical CES implementation authority
- `FACTORY-002`: historical Enterprise Core implementation authority
- These factory jobs do not authorize Kernel architecture changes.

## Dependency Direction
- `AI Engineering OS -> CHELA-X CES -> CHELA-X Kernel -> CHELA-X Runtime -> CHELA-X SDK -> CHELA-X Media`
- Kernel MUST NOT depend on Runtime, SDK, or Media.

## Library Verification
- Validation command: `python3 scripts/validate-library.py`
- Query surface verified: `search`, `get`, `dependencies`, `release`
