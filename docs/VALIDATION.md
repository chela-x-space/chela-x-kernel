# VALIDATION

## Status
Draft

## Version
0.2.1

## Owner
Kernel Platform Team

## Last Updated
2026-07-14

## Applies To
Validation commands and K1.1 verification for CHELA-X Kernel.

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
- `cargo fmt --all --check`: initial `FORMAT` failure, then pass after applying `cargo fmt --all`.
- `cargo check --workspace --all-targets`: initial `COMPILE` failure from moved `DelegationDepth`, then pass.
- `cargo test --workspace --all-targets`: `ENVIRONMENT` failure; no `cc` linker is present on this machine.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo doc --workspace --no-deps`: pass.
- `cargo test --workspace --doc`: pass.
- `git diff --check`: pass.
- `git status --short`: clean required before final completion.
