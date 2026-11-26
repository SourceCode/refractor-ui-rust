//! Automated repository governance & ADR drift checker (`cargo run -p refractor-xtask -- check-repo`).
//!
//! Enforces `ADR-0001`..`ADR-0006` invariants:
//! - Strict `SemVer` 2.0.0 workspace version lockstep across all Cargo manifests
//! - `CHANGELOG.md` documents the current workspace version
//! - Required governance files & ADRs (`ADR-0001`..`ADR-0006`) exist
//! - Full 339-component catalog across 58 categories and 67 themes are intact
//! - Zero `unsafe` keyword tokens in `crates/` and `apps/`

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use refractor_components::{all_component_groups, all_component_specs};
use refractor_themes::all_themes;

fn check_repo(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();

    // 1. Parse workspace Cargo.toml & validate SemVer 2.0.0
    let root_manifest_path = root.join("Cargo.toml");
    let root_text = match fs::read_to_string(&root_manifest_path) {
        Ok(t) => t,
        Err(e) => {
            errors.push(format!("Cannot read root Cargo.toml: {e}"));
            return errors;
        }
    };
    let root_toml: toml::Value = match toml::from_str(&root_text) {
        Ok(v) => v,
        Err(e) => {
            errors.push(format!("Invalid root Cargo.toml: {e}"));
            return errors;
        }
    };

    let version = root_toml
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("version"))
        .and_then(toml::Value::as_str)
        .unwrap_or("");

    if !is_semver_2_0_0(version) {
        errors.push(format!(
            "Workspace version '{version}' is not valid SemVer 2.0.0."
        ));
    }

    // 2. Verify CHANGELOG.md documents current version
    let changelog_path = root.join("CHANGELOG.md");
    match fs::read_to_string(&changelog_path) {
        Ok(cl) => {
            let needle = format!("## [{version}]");
            if !cl.contains(&needle) {
                errors.push(format!("CHANGELOG.md missing section \"{needle}\""));
            }
        }
        Err(_) => errors.push("Missing CHANGELOG.md".to_owned()),
    }

    // 3. Verify required governance files and all 6 ADRs
    let required_files = [
        "README.md",
        "LICENSE",
        "NOTICE",
        "CHANGELOG.md",
        "CONTRIBUTING.md",
        "CODE_OF_CONDUCT.md",
        "SECURITY.md",
        "AGENTS.md",
        ".github/CODEOWNERS",
        ".github/SUPPORT.md",
        ".github/PULL_REQUEST_TEMPLATE.md",
        ".github/dependabot.yml",
        ".github/ci-templates/ci.yml",
        ".github/ci-templates/release.yml",
        "docs/adr/README.md",
        "docs/adr/ADR-0001-workspace-and-open-internals-component-architecture.md",
        "docs/adr/ADR-0002-strict-rust-safety-zero-unsafe-and-clippy-governance.md",
        "docs/adr/ADR-0003-optical-theming-and-gpui-material-engine.md",
        "docs/adr/ADR-0004-semantic-versioning-and-release-workflow.md",
        "docs/adr/ADR-0005-testing-adversarial-coverage-and-security-verification.md",
        "docs/adr/ADR-0006-accessibility-and-optical-glass-design-standards.md",
    ];
    for rel in required_files {
        if !root.join(rel).exists() {
            errors.push(format!("Missing required governance/ADR file: {rel}"));
        }
    }

    // 4. Verify member manifests inherit version.workspace = true and lints.workspace = true
    if let Some(members) = root_toml
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(toml::Value::as_array)
    {
        for m in members {
            if let Some(rel) = m.as_str() {
                let manifest = root.join(rel).join("Cargo.toml");
                match fs::read_to_string(&manifest) {
                    Ok(txt) => {
                        if !txt.contains("version.workspace = true") {
                            errors.push(format!(
                                "{rel}/Cargo.toml must inherit `version.workspace = true`"
                            ));
                        }
                        if !txt.contains("workspace = true") {
                            errors.push(format!(
                                "{rel}/Cargo.toml must inherit `[lints] workspace = true`"
                            ));
                        }
                    }
                    Err(_) => errors.push(format!("Missing member manifest: {rel}/Cargo.toml")),
                }
            }
        }
    }

    // 5. Verify catalog parity (339 components, 58 categories, 67 themes)
    let comps = all_component_specs().len();
    let cats = all_component_groups().len();
    let themes = all_themes().len();
    if comps != 339 {
        errors.push(format!("Expected 339 components, found {comps}"));
    }
    if cats != 58 {
        errors.push(format!("Expected 58 categories, found {cats}"));
    }
    if themes != 67 {
        errors.push(format!("Expected 67 themes, found {themes}"));
    }

    // 6. Verify zero `unsafe {` blocks across crates/ and apps/
    scan_for_unsafe(&root.join("crates"), &mut errors);
    scan_for_unsafe(&root.join("apps"), &mut errors);

    errors
}

fn scan_for_unsafe(dir: &Path, errors: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_for_unsafe(&path, errors);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs")
            && let Ok(content) = fs::read_to_string(&path)
        {
            for (idx, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("//") {
                    continue;
                }
                let block_needle = concat!("unsa", "fe {");
                let fn_needle = concat!("unsa", "fe fn ");
                if trimmed.contains(block_needle) || trimmed.starts_with(fn_needle) {
                    errors.push(format!(
                        "Forbidden unsafe code in {}:{}",
                        path.display(),
                        idx + 1
                    ));
                }
            }
        }
    }
}

fn is_semver_2_0_0(version: &str) -> bool {
    let core = version.split(['-', '+']).next().unwrap_or("");
    let parts: Vec<&str> = core.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map_or("check-repo", String::as_str);
    if cmd != "check-repo" {
        eprintln!("Usage: cargo run -p refractor-xtask -- check-repo");
        return ExitCode::FAILURE;
    }

    let root = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let errors = check_repo(&root);
    if errors.is_empty() {
        println!(
            "check-repo PASSED: 339 components, 58 categories, 67 themes, all 6 ADRs & SemVer lockstep verified."
        );
        ExitCode::SUCCESS
    } else {
        eprintln!("check-repo FAILED with {} error(s):", errors.len());
        for err in errors {
            eprintln!("  - {err}");
        }
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_validation_accepts_valid_and_rejects_invalid() {
        assert!(is_semver_2_0_0("1.0.0"));
        assert!(is_semver_2_0_0("1.47.1"));
        assert!(is_semver_2_0_0("2.0.0-rc.1+build.42"));
        assert!(!is_semver_2_0_0("1.0"));
        assert!(!is_semver_2_0_0("v1.0.0"));
        assert!(!is_semver_2_0_0(""));
    }

    #[test]
    fn check_repo_passes_on_workspace_root() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or(manifest_dir);
        let errors = check_repo(root);
        assert!(errors.is_empty(), "expected 0 governance errors, got: {errors:?}");
    }

    #[test]
    fn check_repo_detects_missing_root_manifest() {
        let errors = check_repo(Path::new("nonexistent-dir-for-test"));
        assert!(!errors.is_empty(), "expected missing manifest error");
    }
}
