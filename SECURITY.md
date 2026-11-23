# Security Policy

## Supported Versions

| Version | Supported |
| --- | --- |
| `1.x` | Yes |

## Security Invariants

Refractor UI for Rust enforces defense-in-depth invariants (see `docs/adr/ADR-0005`):
- `#![forbid(unsafe_code)]` across all workspace crates.
- Strict URL protocol allowlisting (`sanitize_url`), HTML allowlist sanitization (`sanitize_html`), and OWASP CSV formula neutralization (`sanitize_csv_cell`).
- ReDoS-free literal text highlighting (`split_literal_highlight`).
- Strict component-name allowlisting and path-traversal rejection in `refractor-cli swizzle`.

## Reporting a Vulnerability

Please report security issues privately via GitHub Security Advisories rather than opening a public issue.
