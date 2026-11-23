## Summary

<!-- Describe what this PR changes and which ADR(s) or components it touches. -->

## Verification Checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes (0 errors, 0 warnings)
- [ ] `cargo test --workspace` passes (positive, negative, and adversarial tests)
- [ ] `cargo run -p refractor-xtask -- check-repo` passes
- [ ] `CHANGELOG.md` updated if public API changed
