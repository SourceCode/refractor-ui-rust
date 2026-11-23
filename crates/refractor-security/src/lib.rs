//! Defense-in-depth security and sanitization primitives for **Refractor UI for Rust**.
//!
//! Implements ADR-0005 invariants:
//! - URL protocol allowlisting (`sanitize_url`)
//! - Strict allowlist HTML sanitization (`sanitize_html`)
//! - OWASP CSV / Excel formula neutralization (`sanitize_csv_cell`, `build_safe_csv`)
//! - NaN/Infinity-safe numeric clamping (`clamp_f32`, `clamp_f64`)
//! - Prototype-pollution-safe key validation (`is_safe_object_key`)
//! - ReDoS-free literal highlight splitting (`split_literal_highlight`)
//! - Path-traversal-hardened swizzle validation (`validate_swizzle_target`)

use std::path::{Component, Path, PathBuf};

/// Forbidden object property keys guarded against prototype pollution patterns.
const FORBIDDEN_KEYS: [&str; 3] = ["__proto__", "constructor", "prototype"];

/// Dangerous HTML block tags whose entire subtree must be stripped.
const DANGEROUS_BLOCK_TAGS: [&str; 11] = [
    "script", "style", "iframe", "object", "embed", "svg", "math", "form", "meta", "link", "base",
];

/// Allowlisted safe HTML tags for `sanitize_html`.
const ALLOWED_HTML_TAGS: [&str; 19] = [
    "p",
    "br",
    "strong",
    "b",
    "em",
    "i",
    "u",
    "h1",
    "h2",
    "h3",
    "ul",
    "ol",
    "li",
    "blockquote",
    "code",
    "pre",
    "a",
    "img",
    "span",
];

/// Returns `false` for prototype-polluting object property keys.
pub fn is_safe_object_key(key: &str) -> bool {
    !FORBIDDEN_KEYS.contains(&key)
}

/// Clamps an `f32` value into `[min, max]` while defending against `NaN` and `±Infinity`.
pub fn clamp_f32(value: f32, min: f32, max: f32, fallback: f32) -> f32 {
    if !value.is_finite() {
        return if fallback.is_finite() {
            fallback.clamp(min.min(max), min.max(max))
        } else {
            min.min(max)
        };
    }
    let safe_min = min.min(max);
    let safe_max = min.max(max);
    value.clamp(safe_min, safe_max)
}

/// Clamps an `f64` value into `[min, max]` while defending against `NaN` and `±Infinity`.
pub fn clamp_f64(value: f64, min: f64, max: f64, fallback: f64) -> f64 {
    if !value.is_finite() {
        return if fallback.is_finite() {
            fallback.clamp(min.min(max), min.max(max))
        } else {
            min.min(max)
        };
    }
    let safe_min = min.min(max);
    let safe_max = min.max(max);
    value.clamp(safe_min, safe_max)
}

/// Strips ASCII control characters (0x00..0x1F and 0x7F).
fn strip_control_chars(input: &str) -> String {
    input
        .chars()
        .filter(|&c| {
            let code = c as u32;
            code >= 32 && code != 127
        })
        .collect()
}

/// Validates and sanitizes user-provided URLs against XSS and protocol injection.
///
/// Allows `http:`, `https:`, `mailto:`, hash fragments (`#`), and relative paths (`/`, `./`, `../`).
/// Rejects protocol-relative URLs (`//evil.com`), `javascript:`, `vbscript:`, and `data:text/html`.
/// Optionally permits safe base64 raster `data:image/(png|jpeg|jpg|webp|gif);base64,...` URIs.
pub fn sanitize_url(raw_url: &str, allow_raster_data_uri: bool) -> String {
    let cleaned = strip_control_chars(raw_url);
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.starts_with('/') && !trimmed.starts_with("//") {
        return trimmed.to_owned();
    }
    if trimmed.starts_with('#') || trimmed.starts_with("./") || trimmed.starts_with("../") {
        return trimmed.to_owned();
    }

    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("mailto:")
    {
        return trimmed.to_owned();
    }

    if allow_raster_data_uri && is_valid_raster_data_uri(&lower, trimmed) {
        return trimmed.to_owned();
    }

    String::new()
}

fn is_valid_raster_data_uri(lower: &str, original: &str) -> bool {
    const PREFIXES: [&str; 5] = [
        "data:image/png;base64,",
        "data:image/jpeg;base64,",
        "data:image/jpg;base64,",
        "data:image/webp;base64,",
        "data:image/gif;base64,",
    ];
    for prefix in PREFIXES {
        if lower.starts_with(prefix) {
            let payload = &original[prefix.len()..];
            return !payload.is_empty()
                && payload
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=');
        }
    }
    false
}

/// Extracts an HTML attribute value (double-quoted, single-quoted, or unquoted) case-insensitively.
fn extract_html_attr(attrs: &str, target: &str) -> Option<String> {
    let lower = attrs.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let target_bytes = target.as_bytes();
    let mut idx = 0;

    while idx + target_bytes.len() <= bytes.len() {
        if &bytes[idx..idx + target_bytes.len()] == target_bytes {
            let boundary_before = idx == 0 || !bytes[idx - 1].is_ascii_alphanumeric();
            let mut cursor = idx + target_bytes.len();
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            if boundary_before && cursor < bytes.len() && bytes[cursor] == b'=' {
                cursor += 1;
                while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                    cursor += 1;
                }
                if cursor >= attrs.len() {
                    return None;
                }
                let rest = &attrs[cursor..];
                if let Some(stripped) = rest.strip_prefix('"') {
                    let end = stripped.find('"').unwrap_or(stripped.len());
                    return Some(stripped[..end].to_owned());
                }
                if let Some(stripped) = rest.strip_prefix('\'') {
                    let end = stripped.find('\'').unwrap_or(stripped.len());
                    return Some(stripped[..end].to_owned());
                }
                let end = rest
                    .find(|c: char| c.is_ascii_whitespace() || c == '>')
                    .unwrap_or(rest.len());
                return Some(rest[..end].to_owned());
            }
        }
        idx += 1;
    }
    None
}

/// Strict allowlist HTML sanitizer removing `<script>`, `<iframe>`, `<object>`, `<embed>`,
/// `<style>`, `<svg>`, `<math>`, `<form>`, `<meta>`, `<link>`, `<base>`, HTML comments,
/// inline `on*` event handlers, and unsafe `javascript:`/`vbscript:`/`data:text/html` URIs.
pub fn sanitize_html(raw_html: &str) -> String {
    let mut output = String::with_capacity(raw_html.len());
    let mut rest = raw_html;

    while let Some(lt) = rest.find('<') {
        output.push_str(&rest[..lt]);
        let after_lt = &rest[lt..];

        // Strip HTML comments
        if let Some(comment_body) = after_lt.strip_prefix("<!--") {
            if let Some(end_comment) = comment_body.find("-->") {
                rest = &comment_body[end_comment + 3..];
                continue;
            }
            // Unclosed comment: drop remainder safely
            return output;
        }

        let Some(gt) = after_lt.find('>') else {
            // Unclosed tag: drop remainder safely
            return output;
        };

        let tag_inner = &after_lt[1..gt];
        let trimmed = tag_inner.trim();
        let (is_closing, tag_body) = if let Some(stripped) = trimmed.strip_prefix('/') {
            (true, stripped.trim_start())
        } else {
            (false, trimmed)
        };

        let name_end = tag_body
            .find(|c: char| !c.is_ascii_alphanumeric())
            .unwrap_or(tag_body.len());
        let tag_name = tag_body[..name_end].to_ascii_lowercase();
        let raw_attrs = &tag_body[name_end..];

        if !is_closing && DANGEROUS_BLOCK_TAGS.contains(&tag_name.as_str()) {
            let after_open = &after_lt[gt + 1..];
            let lower_after = after_open.to_ascii_lowercase();
            let close_needle = format!("</{tag_name}");
            if let Some(close_pos) = lower_after.find(&close_needle) {
                let after_close = &after_open[close_pos..];
                if let Some(close_gt) = after_close.find('>') {
                    rest = &after_close[close_gt + 1..];
                    continue;
                }
            }
            rest = after_open;
            continue;
        }

        if ALLOWED_HTML_TAGS.contains(&tag_name.as_str()) {
            if is_closing {
                if tag_name != "br" && tag_name != "img" {
                    output.push_str("</");
                    output.push_str(&tag_name);
                    output.push('>');
                }
            } else if tag_name == "a" {
                let raw_href = extract_html_attr(raw_attrs, "href").unwrap_or_default();
                let safe_href = sanitize_url(&raw_href, false);
                if safe_href.is_empty() {
                    output.push_str("<a>");
                } else {
                    let escaped = safe_href.replace('"', "&quot;");
                    output.push_str("<a href=\"");
                    output.push_str(&escaped);
                    output.push_str("\" rel=\"noopener noreferrer\">");
                }
            } else if tag_name == "img" {
                let raw_src = extract_html_attr(raw_attrs, "src").unwrap_or_default();
                let safe_src = sanitize_url(&raw_src, true);
                if !safe_src.is_empty() {
                    let safe_alt = extract_html_attr(raw_attrs, "alt")
                        .unwrap_or_default()
                        .replace(['<', '>', '"', '\''], "");
                    let escaped_src = safe_src.replace('"', "&quot;");
                    output.push_str("<img src=\"");
                    output.push_str(&escaped_src);
                    output.push_str("\" alt=\"");
                    output.push_str(&safe_alt);
                    output.push_str("\" />");
                }
            } else if tag_name == "br" {
                output.push_str("<br />");
            } else {
                output.push('<');
                output.push_str(&tag_name);
                output.push('>');
            }
        }

        rest = &after_lt[gt + 1..];
    }

    output.push_str(rest);
    output
}

/// Neutralizes CSV / Excel formula injection (OWASP CSV Injection defense)
/// and applies strict RFC 4180 quoting.
pub fn sanitize_csv_cell(value: &str) -> String {
    let trimmed_leading = value.trim_start_matches([' ', '\t', '\r']);
    let starts_with_formula = trimmed_leading.starts_with(['=', '+', '-', '@'])
        || value.starts_with(['\t', '\r']);

    let mut text = if starts_with_formula {
        let mut prefixed = String::with_capacity(value.len() + 1);
        prefixed.push('\'');
        prefixed.push_str(value);
        prefixed
    } else {
        value.to_owned()
    };

    if text.contains([',', '"', '\r', '\n']) {
        text = text.replace('"', "\"\"");
        let mut quoted = String::with_capacity(text.len() + 2);
        quoted.push('"');
        quoted.push_str(&text);
        quoted.push('"');
        quoted
    } else {
        text
    }
}

/// Builds an RFC 4180 compliant, formula-safe CSV document from headers and rows.
pub fn build_safe_csv<H, R, C>(headers: &[H], rows: &[R]) -> String
where
    H: AsRef<str>,
    R: AsRef<[C]>,
    C: AsRef<str>,
{
    let mut lines = Vec::with_capacity(rows.len() + 1);
    let header_line = headers
        .iter()
        .map(|h| sanitize_csv_cell(h.as_ref()))
        .collect::<Vec<_>>()
        .join(",");
    lines.push(header_line);

    for row in rows {
        let line = row
            .as_ref()
            .iter()
            .map(|c| sanitize_csv_cell(c.as_ref()))
            .collect::<Vec<_>>()
            .join(",");
        lines.push(line);
    }

    lines.join("\r\n")
}

/// A highlighted literal segment returned by [`split_literal_highlight`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightSegment {
    /// Text slice for this segment.
    pub text: String,
    /// Whether this segment matched the query literally.
    pub highlighted: bool,
}

/// Safely splits text into highlighted literal fragments without constructing regular
/// expressions from untrusted user input (ReDoS-immune).
pub fn split_literal_highlight(source: &str, query: &str) -> Vec<HighlightSegment> {
    let q = query.trim();
    if q.is_empty() || source.is_empty() {
        return vec![HighlightSegment {
            text: source.to_owned(),
            highlighted: false,
        }];
    }

    let lower_source = source.to_ascii_lowercase();
    let lower_query = q.to_ascii_lowercase();
    if lower_source.len() != source.len() {
        // Fallback for non-ASCII case-folding length changes: exact substring scan
        return split_exact(source, q);
    }

    let mut segments = Vec::new();
    let mut cursor = 0;

    while cursor < source.len() {
        if let Some(rel_idx) = lower_source[cursor..].find(&lower_query) {
            let idx = cursor + rel_idx;
            if idx > cursor {
                segments.push(HighlightSegment {
                    text: source[cursor..idx].to_owned(),
                    highlighted: false,
                });
            }
            let end = idx + lower_query.len();
            segments.push(HighlightSegment {
                text: source[idx..end].to_owned(),
                highlighted: true,
            });
            cursor = end;
        } else {
            segments.push(HighlightSegment {
                text: source[cursor..].to_owned(),
                highlighted: false,
            });
            break;
        }
    }

    if segments.is_empty() {
        segments.push(HighlightSegment {
            text: source.to_owned(),
            highlighted: false,
        });
    }

    segments
}

fn split_exact(source: &str, query: &str) -> Vec<HighlightSegment> {
    let mut segments = Vec::new();
    let mut cursor = 0;
    while cursor < source.len() {
        if let Some(rel_idx) = source[cursor..].find(query) {
            let idx = cursor + rel_idx;
            if idx > cursor {
                segments.push(HighlightSegment {
                    text: source[cursor..idx].to_owned(),
                    highlighted: false,
                });
            }
            let end = idx + query.len();
            segments.push(HighlightSegment {
                text: source[idx..end].to_owned(),
                highlighted: true,
            });
            cursor = end;
        } else {
            segments.push(HighlightSegment {
                text: source[cursor..].to_owned(),
                highlighted: false,
            });
            break;
        }
    }
    if segments.is_empty() {
        segments.push(HighlightSegment {
            text: source.to_owned(),
            highlighted: false,
        });
    }
    segments
}

/// Errors produced when validating CLI swizzle targets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwizzleValidationError {
    /// Component name is empty or contains non-PascalCase characters.
    InvalidComponentName(String),
    /// Output directory contains parent traversal (`..`) or is empty.
    UnsafeOutputPath(String),
}

impl std::fmt::Display for SwizzleValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidComponentName(name) => {
                write!(f, "invalid component name for swizzle: '{name}'")
            }
            Self::UnsafeOutputPath(path) => {
                write!(f, "unsafe output path (path traversal forbidden): '{path}'")
            }
        }
    }
}

impl std::error::Error for SwizzleValidationError {}

/// Validates a component name and output directory for CLI swizzling, forbidding
/// `..` path traversal and arbitrary filesystem injection (ADR-0005).
pub fn validate_swizzle_target(
    component_name: &str,
    out_dir: &Path,
) -> Result<PathBuf, SwizzleValidationError> {
    let mut chars = component_name.chars();
    let valid_name = match chars.next() {
        Some(first) if first.is_ascii_uppercase() => {
            chars.all(|c| c.is_ascii_alphanumeric())
        }
        _ => false,
    };
    if !valid_name {
        return Err(SwizzleValidationError::InvalidComponentName(
            component_name.to_owned(),
        ));
    }

    if out_dir.as_os_str().is_empty() {
        return Err(SwizzleValidationError::UnsafeOutputPath(String::new()));
    }

    for comp in out_dir.components() {
        if matches!(comp, Component::ParentDir) {
            return Err(SwizzleValidationError::UnsafeOutputPath(
                out_dir.display().to_string(),
            ));
        }
    }

    let file_name = format!("{}.rs", to_snake_case(component_name));
    Ok(out_dir.join(file_name))
}

fn to_snake_case(pascal: &str) -> String {
    let mut out = String::with_capacity(pascal.len() + 4);
    for (idx, ch) in pascal.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if idx > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_keys_reject_prototype_pollution() {
        assert!(is_safe_object_key("primary"));
        assert!(is_safe_object_key("windowBg"));
        assert!(!is_safe_object_key("__proto__"));
        assert!(!is_safe_object_key("constructor"));
        assert!(!is_safe_object_key("prototype"));
    }

    #[test]
    fn clamp_defends_against_nan_and_infinity() {
        assert!((clamp_f32(5.0, 0.0, 10.0, 1.0) - 5.0).abs() < f32::EPSILON);
        assert!((clamp_f32(f32::NAN, 0.0, 10.0, 3.0) - 3.0).abs() < f32::EPSILON);
        assert!((clamp_f32(f32::INFINITY, 0.0, 10.0, 2.0) - 2.0).abs() < f32::EPSILON);
        assert!((clamp_f32(f32::NEG_INFINITY, 10.0, 0.0, 15.0) - 10.0).abs() < f32::EPSILON);
        assert!((clamp_f64(f64::NAN, 0.0, 1.0, 0.5) - 0.5).abs() < f64::EPSILON);
        assert!((clamp_f64(2.5, 0.0, 1.0, 0.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sanitize_url_allows_safe_and_blocks_adversarial() {
        assert_eq!(sanitize_url("https://example.com/path", false), "https://example.com/path");
        assert_eq!(sanitize_url("http://localhost:3000", false), "http://localhost:3000");
        assert_eq!(sanitize_url("mailto:hi@example.com", false), "mailto:hi@example.com");
        assert_eq!(sanitize_url("/docs/start", false), "/docs/start");
        assert_eq!(sanitize_url("#section", false), "#section");

        // Adversarial injections
        assert_eq!(sanitize_url("javascript:alert(1)", false), "");
        assert_eq!(sanitize_url("  JaVaScRiPt:alert(1)", false), "");
        assert_eq!(sanitize_url("java\x00script:alert(1)", false), "");
        assert_eq!(sanitize_url("vbscript:msgbox(1)", false), "");
        assert_eq!(sanitize_url("//evil.example/payload", false), "");
        assert_eq!(sanitize_url("data:text/html,<script>alert(1)</script>", true), "");

        // Raster data URI opt-in
        let png = "data:image/png;base64,iVBORw0KGgo=";
        assert_eq!(sanitize_url(png, false), "");
        assert_eq!(sanitize_url(png, true), png);
    }

    #[test]
    fn sanitize_html_strips_xss_and_preserves_safe_markup() {
        let dirty = r#"<p>Hello</p><script>alert(1)</script><img src="javascript:alert(1)" onerror="alert(2)" /><a href="https://example.com" onclick="evil()">Link</a><!-- secret -->"#;
        let clean = sanitize_html(dirty);
        assert!(!clean.contains("script"));
        assert!(!clean.contains("onerror"));
        assert!(!clean.contains("onclick"));
        assert!(!clean.contains("javascript:"));
        assert!(!clean.contains("secret"));
        assert!(clean.contains("<p>Hello</p>"));
        assert!(clean.contains(r#"<a href="https://example.com" rel="noopener noreferrer">Link</a>"#));
    }

    #[test]
    fn sanitize_csv_neutralizes_owasp_formulas() {
        assert_eq!(sanitize_csv_cell("normal"), "normal");
        assert_eq!(sanitize_csv_cell("=CMD|' /C calc'!A0"), "'=CMD|' /C calc'!A0");
        assert_eq!(sanitize_csv_cell("+1+1"), "'+1+1");
        assert_eq!(sanitize_csv_cell("-2+3"), "'-2+3");
        assert_eq!(sanitize_csv_cell("@SUM(1+1)"), "'@SUM(1+1)");
        assert_eq!(sanitize_csv_cell("a, \"b\""), "\"a, \"\"b\"\"\"");

        let csv = build_safe_csv(&["Name", "Formula"], &[vec!["Alice", "=1+1"]]);
        assert_eq!(csv, "Name,Formula\r\nAlice,'=1+1");
    }

    #[test]
    fn split_literal_highlight_is_redos_safe() {
        let segs = split_literal_highlight("Refractor Optical Glass", "optical");
        assert_eq!(segs.len(), 3);
        assert!(!segs[0].highlighted);
        assert!(segs[1].highlighted);
        assert_eq!(segs[1].text, "Optical");

        // Adversarial regex metacharacters treated strictly literally
        let evil = "(a+)+$";
        let segs_evil = split_literal_highlight("test (a+)+$ value", evil);
        assert_eq!(segs_evil.len(), 3);
        assert!(segs_evil[1].highlighted);
        assert_eq!(segs_evil[1].text, evil);
    }

    #[test]
    fn swizzle_validation_blocks_path_traversal() {
        let ok = validate_swizzle_target("PrimaryButton", Path::new("src/ui"));
        assert!(ok.is_ok());

        let traversal = validate_swizzle_target("PrimaryButton", Path::new("../../etc"));
        assert!(matches!(traversal, Err(SwizzleValidationError::UnsafeOutputPath(_))));
        if let Err(err) = traversal {
            assert!(err.to_string().contains("unsafe output path"));
        }

        let empty_out = validate_swizzle_target("PrimaryButton", Path::new(""));
        assert!(matches!(empty_out, Err(SwizzleValidationError::UnsafeOutputPath(_))));

        let bad_name = validate_swizzle_target("../Evil", Path::new("src/ui"));
        assert!(matches!(bad_name, Err(SwizzleValidationError::InvalidComponentName(_))));
        if let Err(err) = bad_name {
            assert!(err.to_string().contains("invalid component name"));
        }
        assert!(validate_swizzle_target("", Path::new("src/ui")).is_err());
    }

    #[test]
    fn covers_html_edge_cases_and_unicode_highlighting() {
        // Safe img with alt, br, single-quoted attrs, unquoted attrs, unclosed tag/comment
        let html = r#"<img src='https://example.com/a.png' alt='Logo' /><br><a href=https://example.com>Ok</a><a href="javascript:evil">Bad</a><svg><script>x</script></svg><unknown>Drop</unknown>"#;
        let clean = sanitize_html(html);
        assert!(clean.contains(r#"<img src="https://example.com/a.png" alt="Logo" />"#));
        assert!(clean.contains("<br />"));
        assert!(clean.contains("<a>Bad</a>"));
        assert!(!clean.contains("svg"));
        assert!(!clean.contains("unknown"));

        assert_eq!(sanitize_html("hello <!-- unclosed"), "hello ");
        assert_eq!(sanitize_html("hello <unclosed"), "hello ");
        assert_eq!(sanitize_url("", false), "");
        assert_eq!(sanitize_url("./local/path", false), "./local/path");
        assert_eq!(sanitize_url("../parent/path", false), "../parent/path");
        assert_eq!(sanitize_url("data:image/png;base64,!!!invalid!!!", true), "");

        // Non-finite fallback in clamp
        assert!((clamp_f32(f32::NAN, 1.0, 5.0, f32::NAN) - 1.0).abs() < f32::EPSILON);
        assert!((clamp_f64(f64::NAN, 2.0, 5.0, f64::NAN) - 2.0).abs() < f64::EPSILON);

        // Empty query / source & exact split fallback
        assert_eq!(split_literal_highlight("abc", "").len(), 1);
        assert_eq!(split_literal_highlight("", "a").len(), 1);
        let exact = split_exact("alpha beta alpha", "alpha");
        assert_eq!(exact.len(), 3);
        assert!(exact[0].highlighted);
        assert!(!exact[1].highlighted);
        assert!(exact[2].highlighted);
        assert_eq!(split_exact("no-match", "zzz").len(), 1);
    }
}
