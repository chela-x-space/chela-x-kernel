# AGENTS

## Status
Draft

## Version
0.1.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-13

## Applies To
All AI agents and automated engineering assistants operating in CHELA-X Kernel.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Purpose
Define the operating contract for implementation work in CHELA-X Kernel under inherited AI Engineering OS governance.

## Active Role
- Agents SHALL operate as an implementation engineer under AI Engineering OS.
- Agents SHALL preserve CES Book 0 RC1, Program dependency direction, and Library traceability evidence.

## Governing Order
1. Explicit human instruction
2. Approved ADR
3. Frozen dependency direction and approved architecture baseline
4. Repository rules in this file and `ENGINEERING.md`
5. AI Engineering OS v1.0

## Repository Rules
- Kernel implementation is implementation-only until authorized otherwise.
- Agents MUST NOT redesign architecture, change dependency direction, or infer missing architectural components.
- If implementation requires architectural change, agents MUST stop and report `ADR REQUIRED`.
- Agents MUST NOT modify frozen upstream repositories.
- Agents MUST keep changes small, traceable, and reviewable.
- Agents MUST maintain requirement traceability to CES and Program sources.

## Validation Gate
- Minimum validation before commit includes repository inspection, `git diff --check`, and every available local build, format, lint, and test command.
- If an environment prerequisite is missing, the blocked command and reason MUST be reported explicitly.

## References
- `ENGINEERING.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`
- `/home/chela-x/ai-engineering-os/ENGINEER.md`
- `/home/chela-x/ai-engineering-os/ARCHITECTURE.md`
- `/home/chela-x/ai-engineering-os/DECISION.md`
- `/home/chela-x/ai-engineering-os/WORKFLOW.md`
- `/home/chela-x/ai-engineering-os/STANDARDS.md`
