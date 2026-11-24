//! Agent-ready CLI for **Refractor UI for Rust** (`refractor`).
//!
//! Commands:
//! - `help`
//! - `component --list`
//! - `component <Name> [--dense | --detail full]`
//! - `docs theme [--dense]`
//! - `theme list`
//! - `theme inspect <id>`
//! - `swizzle <Name> --out <dir>`
//! - `check [--strict]`

use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use refractor_components::{
    all_component_groups, all_component_specs, find_component_spec,
};
use refractor_security::validate_swizzle_target;
use refractor_themes::{all_themes, find_theme};
use refractor_tokens::{ThemeMaterial, ThemeOverlayFx};

fn print_help() {
    println!(
        "\
Refractor UI for Rust CLI (v{})

USAGE:
  refractor help
  refractor component --list
  refractor component <Name> [--dense | --detail full]
  refractor docs theme [--dense]
  refractor theme list
  refractor theme inspect <id>
  refractor swizzle <Name> --out <dir>
  refractor check [--strict]",
        env!("CARGO_PKG_VERSION")
    );
}

fn cmd_component(args: &[String]) -> ExitCode {
    if args.is_empty() || args[0] == "--list" {
        let specs = all_component_specs();
        println!(
            "Refractor Rust Component Catalog ({} components across {} categories):",
            specs.len(),
            all_component_groups().len()
        );
        for spec in specs {
            println!("  - {:<30} [{}] {}", spec.name, spec.group, spec.display_name);
        }
        return ExitCode::SUCCESS;
    }

    let name = &args[0];
    let dense = args.iter().any(|a| a == "--dense");
    let full = args.windows(2).any(|w| w[0] == "--detail" && w[1] == "full");

    let Some(spec) = find_component_spec(name) else {
        eprintln!("Component '{name}' not found in the 314-component catalog.");
        return ExitCode::FAILURE;
    };

    if dense {
        println!(
            "{} | {} | group={} | props={} | keywords={}",
            spec.name,
            spec.display_name,
            spec.group,
            spec.props.len(),
            spec.keywords.join(",")
        );
        return ExitCode::SUCCESS;
    }

    println!("# {} ({})", spec.display_name, spec.name);
    println!("Group: {} ({})", spec.group, spec.category);
    println!("Description: {}", spec.description);
    println!("Keywords: {}", spec.keywords.join(", "));
    println!("\nProps ({}):", spec.props.len());
    for p in &spec.props {
        println!("  - {}: {} (default: {}) — {}", p.name, p.prop_type, p.default_value, p.description);
    }

    if full {
        println!("\nBest Practices:");
        for bp in &spec.best_practices {
            let tag = if bp.guidance { "DO" } else { "DON'T" };
            println!("  [{tag}] {}", bp.description);
        }
        println!("\nAccessibility:");
        for a in &spec.accessibility {
            println!("  - {} ({}, {}): {}", a.name, a.criterion, a.requirement, a.description);
        }
    }

    ExitCode::SUCCESS
}

fn cmd_theme(args: &[String]) -> ExitCode {
    if args.is_empty() || args[0] == "list" {
        let themes = all_themes();
        println!("Refractor Rust Theme Catalog ({} themes):", themes.len());
        for t in themes {
            println!(
                "  - {:<22} [{:<16}] material={:<10} overlay={:<14} mode={:?}",
                t.id,
                t.category.as_str(),
                t.material.as_str(),
                t.wallpaper_overlay.as_str(),
                t.mode
            );
        }
        return ExitCode::SUCCESS;
    }

    if args[0] == "inspect" {
        let Some(id) = args.get(1) else {
            eprintln!("Usage: refractor theme inspect <id>");
            return ExitCode::FAILURE;
        };
        let Some(entry) = find_theme(id) else {
            eprintln!("Theme '{id}' not found in the 67-theme catalog.");
            return ExitCode::FAILURE;
        };
        let resolved = entry.resolve();
        println!("# {} ({})", entry.name, entry.id);
        println!("Description: {}", entry.description);
        println!(
            "Category: {} | Material: {} | Overlay: {} | Mode: {:?}",
            entry.category.as_str(),
            entry.material.as_str(),
            entry.wallpaper_overlay.as_str(),
            entry.mode
        );
        println!(
            "Primary: {} (contrast: {}) | Secondary: {} | Success: {}",
            resolved.palette.primary.main,
            resolved.palette.primary.contrast_text,
            resolved.palette.secondary.main,
            resolved.palette.success.main
        );
        return ExitCode::SUCCESS;
    }

    eprintln!("Unknown theme subcommand: {}", args[0]);
    ExitCode::FAILURE
}

fn cmd_docs(args: &[String]) -> ExitCode {
    if args.first().map(String::as_str) == Some("theme") {
        let dense = args.iter().any(|a| a == "--dense");
        if dense {
            println!(
                "Refractor Theme Engine | themes={} | materials={} | overlays={}",
                all_themes().len(),
                ThemeMaterial::ALL.len(),
                ThemeOverlayFx::ALL.len()
            );
        } else {
            println!("# Refractor Multi-Material & Optical Glass Theme Engine (ADR-0003)");
            println!("Shipped themes: {}", all_themes().len());
            println!(
                "Materials ({}): {}",
                ThemeMaterial::ALL.len(),
                ThemeMaterial::ALL
                    .iter()
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            println!(
                "Overlays ({}): {}",
                ThemeOverlayFx::ALL.len(),
                ThemeOverlayFx::ALL
                    .iter()
                    .map(|o| o.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        return ExitCode::SUCCESS;
    }
    eprintln!("Usage: refractor docs theme [--dense]");
    ExitCode::FAILURE
}

fn cmd_swizzle(args: &[String]) -> ExitCode {
    let Some(name) = args.first() else {
        eprintln!("Usage: refractor swizzle <ComponentName> --out <dir>");
        return ExitCode::FAILURE;
    };
    let out_dir = args
        .windows(2)
        .find(|w| w[0] == "--out")
        .map_or("src/ui", |w| w[1].as_str());

    let Some(spec) = find_component_spec(name) else {
        eprintln!("Refusing swizzle: '{name}' is not in the allowlisted 314-component catalog.");
        return ExitCode::FAILURE;
    };

    let target_path = match validate_swizzle_target(&spec.name, Path::new(out_dir)) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Security error: {err}");
            return ExitCode::FAILURE;
        }
    };

    if let Some(parent) = target_path.parent()
        && let Err(err) = fs::create_dir_all(parent)
    {
        eprintln!("Failed to create output directory: {err}");
        return ExitCode::FAILURE;
    }

    let template = format!(
        "//! Swizzled local copy of `{name}` ({display}) from Refractor UI for Rust.\n\
         //! Group: {group}\n\n\
         use gpui::{{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px}};\n\
         use refractor_core::{{BaseProps, SurfaceVariant, active_theme, optical_surface}};\n\n\
         #[derive(Debug, Clone, IntoElement)]\n\
         pub struct {name} {{\n\
             pub label: SharedString,\n\
             pub base: BaseProps,\n\
         }}\n\n\
         impl {name} {{\n\
             pub fn new(label: impl Into<SharedString>) -> Self {{\n\
                 Self {{ label: label.into(), base: BaseProps::new() }}\n\
             }}\n\
         }}\n\n\
         impl RenderOnce for {name} {{\n\
             fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {{\n\
                 let theme = active_theme(cx);\n\
                 optical_surface(&theme, SurfaceVariant::Crystal)\n\
                     .p(px(14.0))\n\
                     .child(div().child(self.label))\n\
             }}\n\
         }}\n",
        name = spec.name,
        display = spec.display_name,
        group = spec.group,
    );

    if let Err(err) = fs::write(&target_path, template) {
        eprintln!("Failed to write swizzled file: {err}");
        return ExitCode::FAILURE;
    }

    println!("Swizzled {} -> {}", spec.name, target_path.display());
    ExitCode::SUCCESS
}

fn cmd_check(args: &[String]) -> ExitCode {
    let strict = args.iter().any(|a| a == "--strict");
    let comp_count = all_component_specs().len();
    let group_count = all_component_groups().len();
    let theme_count = all_themes().len();

    let ok = comp_count == 314 && group_count == 55 && theme_count == 67;
    println!(
        "Refractor check (strict={strict}): components={comp_count}/314, categories={group_count}/55, themes={theme_count}/67"
    );
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_help();
        return ExitCode::SUCCESS;
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => {
            print_help();
            ExitCode::SUCCESS
        }
        "component" => cmd_component(&args[1..]),
        "theme" => cmd_theme(&args[1..]),
        "docs" => cmd_docs(&args[1..]),
        "swizzle" => cmd_swizzle(&args[1..]),
        "check" => cmd_check(&args[1..]),
        other => {
            eprintln!("Unknown command: {other}");
            print_help();
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_commands_succeed_on_valid_inputs() {
        assert_eq!(cmd_component(&["--list".to_owned()]), ExitCode::SUCCESS);
        assert_eq!(
            cmd_component(&["PrimaryButton".to_owned(), "--dense".to_owned()]),
            ExitCode::SUCCESS
        );
        assert_eq!(
            cmd_component(&[
                "PrimaryButton".to_owned(),
                "--detail".to_owned(),
                "full".to_owned()
            ]),
            ExitCode::SUCCESS
        );
        assert_eq!(cmd_theme(&["list".to_owned()]), ExitCode::SUCCESS);
        assert_eq!(
            cmd_theme(&["inspect".to_owned(), "cyberpunk".to_owned()]),
            ExitCode::SUCCESS
        );
        assert_eq!(
            cmd_docs(&["theme".to_owned(), "--dense".to_owned()]),
            ExitCode::SUCCESS
        );
        assert_eq!(cmd_check(&["--strict".to_owned()]), ExitCode::SUCCESS);
    }

    #[test]
    fn cli_rejects_unknown_and_adversarial_swizzle_inputs() {
        assert_eq!(cmd_component(&["DoesNotExist".to_owned()]), ExitCode::FAILURE);
        assert_eq!(
            cmd_theme(&["inspect".to_owned(), "unknown-theme".to_owned()]),
            ExitCode::FAILURE
        );
        assert_eq!(
            cmd_swizzle(&[
                "PrimaryButton".to_owned(),
                "--out".to_owned(),
                "../../etc".to_owned()
            ]),
            ExitCode::FAILURE
        );
        assert_eq!(
            cmd_swizzle(&["NotAComponent".to_owned(), "--out".to_owned(), "out".to_owned()]),
            ExitCode::FAILURE
        );
    }
}
