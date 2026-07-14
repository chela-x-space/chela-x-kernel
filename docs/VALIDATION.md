# VALIDATION

## Status
Draft

## Version
0.4.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-15

## Applies To
Validation commands and K1.1, K2, K3, or K4.1 verification for CHELA-X Kernel.

## Review Cycle
Quarterly

## Review Owner
Kernel Platform Team

## Next Review
2026-10-13

## Classification
INTERNAL

## Validation Commands
- `git -C /home/chela-x/ai-engineering-os status --short`
- `git -C /home/chela-x/chela-x-ces status --short`
- `git -C /home/chela-x/chela-x-program status --short`
- `git -C /home/chela-x/chela-x-library status --short`
- `git -C /home/chela-x/chela-x-ces rev-parse book0-rc1`
- `git -C /home/chela-x/chela-x-ces show --stat --oneline book0-rc1`
- `python3 /home/chela-x/chela-x-library/scripts/validate-library.py`
- `python3 /home/chela-x/chela-x-library/scripts/query-library.py search "<term>" --limit 5`
- `command -v rustc`
- `command -v cargo`
- `command -v rustup`
- `rustc --version`
- `cargo --version`
- `rustup show`
- `cargo fmt --all --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo doc --workspace --no-deps`
- `cargo test --workspace --doc`
- `git diff --check`

## Build Commands
- `cargo check --workspace --all-targets`

## Formatting Commands
- `cargo fmt --all --check`

## Lint Commands
- `cargo clippy --workspace --all-targets -- -D warnings`

## Test Commands
- `cargo test --workspace --all-targets`
- `cargo test --workspace --doc`
- `cargo doc --workspace --no-deps`

## Repository Clean Check
- `git status --short`

## Validation Authority
Host Validation
    ↓
Approved CI Validation
    ↓
Codex Sandbox Validation

This precedence applies only when the higher-authority validation was actually executed and evidence exists.

## Validation Result Categories
- `HOST`
- `CI`
- `CODEX SANDBOX`
- `ENVIRONMENT LIMITATION`
- `CODE DEFECT`
- `TEST DEFECT`

## Validation Authority Policy
- Host validation is canonical when it has actually been executed and evidence is supplied.
- CI validation is canonical when an approved CI pipeline exists and completes successfully.
- Codex sandbox validation is supplementary and environment-limited.
- A Codex sandbox failure caused only by missing tools, linker access, network isolation, or filesystem isolation must not override a successful host or CI validation result.
- Host validation must not be claimed unless the commands were actually executed and their results were provided.
- When host and sandbox results differ, record both.
- Host or CI result determines the project validation status.
- Sandbox result is recorded as environment evidence only.
- Code failures discovered in any environment remain real defects and must not be ignored.
- Environment limitations must never be mislabeled as implementation defects.

## Toolchain
- `rustc 1.97.0 (2d8144b78 2026-07-07)`
- `cargo 1.97.0 (c980f4866 2026-06-30)`
- `stable-x86_64-unknown-linux-gnu`
- `rustfmt 1.9.0-stable (2d8144b788 2026-07-07)`
- `clippy 0.1.97 (2d8144b788 2026-07-07)`

## Baseline Verification Results
- Frozen upstream repository status checks were clean at baseline verification time.
- CES tag `book0-rc1` resolved to `6f131072b0ef0e871b929a67ab558409acca4ed6`.
- Library validation passed and retrieval queries succeeded for Kernel-relevant topics.
- Rust installation attempts:
  - `snap install rustup --classic`: blocked by environment permission on `/usr/bin/snap`
  - `curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal`: blocked because `curl` is not installed
  - `python3` download of `rustup-init`: succeeded
  - `/tmp/rustup-init -y --default-toolchain stable --profile minimal`: succeeded
- Direct toolchain path used for K1.1 validation because the rustup proxy attempted redundant component sync against the repository override.

## K1.1 Validation Results
- Codex sandbox absolute-path checks for `/usr/bin/cc`, `/usr/bin/gcc`, and `/usr/bin/cargo` are unavailable in this environment and MUST NOT override verified host evidence.
- Accepted host validation evidence for `/home/chela-x/chela-x-kernel`:
  - `cargo fmt --all --check`: PASS
  - `cargo check --workspace --all-targets`: PASS
  - `cargo test --workspace --all-targets`: PASS
  - unit tests: `38 passed`, `0 failed`, `0 ignored`
  - `cargo clippy --workspace --all-targets -- -D warnings`: PASS
  - `cargo doc --workspace --no-deps`: PASS
  - `cargo test --workspace --doc`: PASS
  - `git diff --check`: PASS
- K1.1 validation status: `PASS`
- Ready for K2: `YES`

## K2 Validation Results
- K2 implementation status: `COMPLETE`
- K2 architecture review: `PASS`
- Canonical host validation evidence:
  - `cargo fmt --all --check`: `PASS`
  - `cargo check --workspace --all-targets`: `PASS`
  - `cargo test --workspace --all-targets`: `PASS`
  - unit tests: `58 passed`, `0 failed`, `0 ignored`
  - `cargo clippy --workspace --all-targets -- -D warnings`: `PASS`
  - `cargo doc --workspace --no-deps`: `PASS`
  - `cargo test --workspace --doc`: `PASS`
  - `git diff --check`: `PASS`
- Codex sandbox validation evidence:
  - `cargo fmt --all --check`: `PASS`
  - `cargo check --workspace --all-targets`: `PASS`
  - `cargo clippy --workspace --all-targets -- -D warnings`: `PASS`
  - `cargo doc --workspace --no-deps`: `PASS`
  - `cargo test --workspace --doc`: `PASS`
  - `cargo test --workspace --all-targets`: `ENVIRONMENT LIMITATION`
- Codex sandbox environment evidence:
  - `command -v cc`: not found
  - `command -v gcc`: not found
  - `command -v clang`: not found
  - direct `cargo test --workspace --all-targets` fails with `linker 'cc' not found`
  - forcing `rust-lld` also fails because system libraries `-lc`, `-lm`, `-lpthread`, `-ldl`, `-lrt`, and `-lutil` are unavailable to the linker
- Host validation status: `PASS`
- K2 validation status: `PASS`
- Overall K2 status: `PASS`

## K3 Validation Results
- K3 implementation status: `COMPLETE`
- K3 architecture review: `PASS`
- Codex sandbox validation evidence:
  - `cargo fmt --all --check`: `PASS`
  - `cargo check --workspace --all-targets`: `PASS`
  - `cargo clippy --workspace --all-targets -- -D warnings`: `PASS`
  - `cargo doc --workspace --no-deps`: `PASS`
  - `cargo test --workspace --doc`: `PASS`
  - `cargo test --workspace --all-targets`: `ENVIRONMENT LIMITATION`
- Codex sandbox environment evidence:
  - direct `cargo test --workspace --all-targets` fails with `linker 'cc' not found`
- K3 host validation status: `PENDING`
- K3 validation status: `PASS WITH HOST TEST VALIDATION PENDING`

## K4.1 Validation Results
- K4.1 implementation status: `COMPLETE`
- K4.1 architecture review: `PASS`
- Codex sandbox validation evidence:
  - `cargo fmt --all --check`: `PASS`
  - `cargo check --workspace --all-targets`: `PASS`
  - `cargo clippy --workspace --all-targets -- -D warnings`: `PASS`
  - `cargo doc --workspace --no-deps`: `PASS`
  - `cargo test --workspace --doc`: `PASS`
  - `cargo test --workspace --all-targets`: `ENVIRONMENT LIMITATION`
- Codex sandbox environment evidence:
  - direct `cargo test --workspace --all-targets` fails with `linker 'cc' not found`
- K4.1 host validation status: `PENDING`
- K4.1 validation status: `PASS WITH HOST TEST VALIDATION PENDING`

## Required Canonical Host Validation Commands
- `cd /home/chela-x/chela-x-kernel`
- `cargo fmt --all --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo doc --workspace --no-deps`
- `cargo test --workspace --doc`
- `git diff --check`
- `git status --short`

## Expected K2 Test Baseline
- Previous K1 tests: `38`
- New K2 tests: `20`
- Expected total: `58`
- Canonical host-verified K2 total: `58`

## Expected K3 Test Baseline
- Previous tests after K2: `58`
- New K3 tests: `28`
- Expected total: `86`
- Sandbox compile baseline: `86` tests discovered in source

## Expected K4.1 Test Baseline
- Previous tests after K3: `86`
- New K4.1 tests: `16`
- Expected total: `102`
- Sandbox compile baseline: `102` tests discovered in source
