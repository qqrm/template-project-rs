# Rust Project Template

Template repository for general-purpose Rust projects.

## Included
- CI workflow with fmt/check/clippy/nextest and dependency checks (`.github/workflows/ci.yml`)
- GitHub Actions security workflow with actionlint and Zizmor (`.github/workflows/workflow-security.yml`)
- Release workflow for version tags (`.github/workflows/release.yml`)
- Codex cleanup workflow (`.github/workflows/codex-cleanup.yml`)

## Usage
1. Create a repository from this template.
2. Replace crate name in `Cargo.toml`.
3. Implement project code in `src/`.
4. Push a tag like `v0.1.0` to trigger release workflow.

## Local Commands
- `cargo fmt --all -- --check`
- `cargo check --tests --benches`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo nextest run`
- `cargo audit`
- `cargo deny check advisories bans sources`
- `cargo machete`

Before publishing a derived project, choose and declare its license. License
enforcement is intentionally not enabled by the template's `cargo-deny` policy.
