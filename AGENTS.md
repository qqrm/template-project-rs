# Repository Agent Instructions

## Core Principles
- Keep crates small and composable.
- Prefer stable APIs and explicit errors.
- Keep CI strict and reproducible.

## Validation Loop
- `cargo fmt --all -- --check`
- `cargo check --tests --benches`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

## Workflow Rules
- Protect `main` with required CI checks.
- Never merge code with failing lint or tests.
