//! Machine-readable 339-component documentation registry ported verbatim from `refractor-ui` v1.52.0
//! with O(1) name lookup and precomputed lowercased search index.

use std::collections::{BTreeSet, HashMap};
use std::sync::{LazyLock, OnceLock};

use serde::{Deserialize, Serialize};

const CATALOG_339_JSON: &str = include_str!("catalog_339.json");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentBestPractice {
    pub guidance: bool,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentA11yItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub criterion: String,
    #[serde(default)]
    pub requirement: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentPropSpec {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub prop_type: String,
    #[serde(rename = "defaultValue", default)]
    pub default_value: String,
    #[serde(default)]
    pub description: String,
}

/// Complete machine-readable metadata for one component in the 339-component catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSpec {
    pub name: String,
    pub display_name: String,
    pub group: String,
    pub category: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub best_practices: Vec<ComponentBestPractice>,
    #[serde(default)]
    pub accessibility: Vec<ComponentA11yItem>,
    #[serde(default)]
    pub props: Vec<ComponentPropSpec>,
}

static REGISTRY: OnceLock<Vec<ComponentSpec>> = OnceLock::new();
static NAME_INDEX: LazyLock<HashMap<String, usize>> = LazyLock::new(|| {
    all_component_specs()
        .iter()
        .enumerate()
        .map(|(idx, spec)| (spec.name.to_ascii_lowercase(), idx))
        .collect()
});
static SEARCH_INDEX: LazyLock<Vec<String>> = LazyLock::new(|| {
    all_component_specs()
        .iter()
        .map(|c| {
            format!(
                "{} {} {} {}",
                c.name.to_ascii_lowercase(),
                c.display_name.to_ascii_lowercase(),
                c.group.to_ascii_lowercase(),
                c.keywords
                    .iter()
                    .map(|k| k.to_ascii_lowercase())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })
        .collect()
});

/// Returns all 339 component specifications across all 58 categories.
pub fn all_component_specs() -> &'static [ComponentSpec] {
    REGISTRY
        .get_or_init(|| serde_json::from_str::<Vec<ComponentSpec>>(CATALOG_339_JSON).unwrap_or_default())
        .as_slice()
}

/// Looks up a component specification by exact or case-insensitive name in O(1) time.
pub fn find_component_spec(name: &str) -> Option<&'static ComponentSpec> {
    let key = name.trim().to_ascii_lowercase();
    let idx = *NAME_INDEX.get(&key)?;
    all_component_specs().get(idx)
}

/// Returns all distinct category group labels in catalog order.
pub fn all_component_groups() -> Vec<&'static str> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    for spec in all_component_specs() {
        if seen.insert(spec.group.as_str()) {
            ordered.push(spec.group.as_str());
        }
    }
    ordered
}

/// Returns all components belonging to a specific category group label.
pub fn components_in_group(group: &str) -> Vec<&'static ComponentSpec> {
    all_component_specs()
        .iter()
        .filter(|c| c.group.eq_ignore_ascii_case(group))
        .collect()
}

/// Searches components by name, display name, group, or keyword (literal, ReDoS-safe, pre-indexed).
pub fn search_components(query: &str) -> Vec<&'static ComponentSpec> {
    let q = query.trim().to_ascii_lowercase();
    let specs = all_component_specs();
    if q.is_empty() {
        return specs.iter().collect();
    }
    SEARCH_INDEX
        .iter()
        .enumerate()
        .filter(|(_, hay)| hay.contains(&q))
        .filter_map(|(idx, _)| specs.get(idx))
        .collect()
}
