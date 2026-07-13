# VALIDATION

## Status
Draft

## Version
0.2.0

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
Validation commands and baseline verification for CHELA-X Kernel K0.

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
- `git diff --check`

## Build Commands
- `cargo check --workspace --all-targets`

## Formatting Commands
- `cargo fmt --all --check`

## Lint Commands
- `cargo clippy --workspace --all-targets -- -D warnings`

## Test Commands
- `cargo test --workspace --all-targets`
- `cargo doc --workspace --no-deps`

## Repository Clean Check
- `git status --short`

## Baseline Verification Results
- Frozen upstream repository status checks were clean at baseline verification time.
- CES tag `book0-rc1` resolved to `6f131072b0ef0e871b929a67ab558409acca4ed6`.
- Library validation passed and retrieval queries succeeded for Kernel-relevant topics.
- Rust toolchain discovery results:
  - `rustc`: not found
  - `cargo`: not found
  - `rustup`: not found
- Rust installation attempts:
  - `snap install rustup --classic`: blocked by environment permission on `/usr/bin/snap`
  - `curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal`: blocked because `curl` is not installed
- Code validation remains an environment blocker until a usable Rust toolchain is available.
