use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DetectRule {
    pub file: String,
    #[serde(default)]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct VarRule {
    pub file: String,
    pub key: String,
    #[serde(default)]
    pub map: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum LayerCmd {
    Literal(String),
    Templated { cmd: String, var: VarRule },
}

#[derive(Deserialize)]
struct RawStep {
    #[serde(default)]
    detect: Vec<DetectRule>,
    #[serde(default)]
    layer: Vec<LayerCmd>,
    #[serde(default)]
    cmd: Option<String>,
    #[serde(flatten)]
    sub: HashMap<String, RawStep>,
}

#[derive(Deserialize)]
struct RawBoard {
    name: String,
    #[serde(flatten)]
    toolchains: HashMap<String, RawStep>,
}

impl RawStep {
    fn into_toolchain(self, name: String) -> Toolchain {
        Toolchain {
            name,
            detect: self.detect,
            layer: self.layer,
            default_cmd: self.cmd,
            sub: self
                .sub
                .into_iter()
                .map(|(k, v)| v.into_toolchain(k))
                .collect(),
        }
    }
}

/// A node in a board's toolchain tree.
#[derive(Debug, Clone)]
pub struct Toolchain {
    pub name: String,
    pub(crate) detect: Vec<DetectRule>,
    pub(crate) layer: Vec<LayerCmd>,
    pub(crate) default_cmd: Option<String>,
    pub(crate) sub: Vec<Toolchain>,
}

/// recipe which holds OCI
#[derive(Debug, Clone)]
pub struct Recipe {
    /// path label, e.g. `"rust → cargo → rustc → thumbv6mnonenabi"`.
    pub label: String,
    /// Full Dockerfile content — the accumulated layer lines verbatim.
    pub dockerfile: String,
    /// Default command to exec inside the container (typically the build command); user can override.
    pub default_cmd: String,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub name: String,
    pub toolchains: Vec<Toolchain>,
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let raw = RawBoard::deserialize(d)?;
        Ok(Board {
            name: raw.name,
            toolchains: raw
                .toolchains
                .into_iter()
                .map(|(k, v)| v.into_toolchain(k))
                .collect(),
        })
    }
}

impl Board {
    pub fn resolve_recipes(&self, project_path: &Path) -> Vec<Recipe> {
        collect_leaves(&self.toolchains, &[], vec![], project_path, true)
    }

    pub fn all_recipes(&self, project_path: &Path) -> Vec<Recipe> {
        collect_leaves(&self.toolchains, &[], vec![], project_path, false)
    }

    pub fn get_recipe_by_path(&self, name: &str, project_path: &Path) -> Result<Recipe> {
        if let Some(r) = self
            .resolve_recipes(project_path)
            .into_iter()
            .find(|r| r.label == name || r.label.split(" → ").last() == Some(name))
        {
            return Ok(r);
        }

        let all = self.all_recipes(project_path);
        if let Some(r) = all
            .iter()
            .find(|r| r.label == name || r.label.split(" → ").last() == Some(name))
            .cloned()
        {
            return Ok(r);
        }

        let available = all
            .iter()
            .map(|r| r.label.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        bail!(
            "Recipe '{}' not found for '{}'. Available: {}",
            name,
            self.name,
            available
        )
    }
}

fn matches_detect(rules: &[DetectRule], project_path: &Path) -> bool {
    rules.iter().all(|rule| {
        let pattern = project_path.join(&rule.file);
        let paths: Vec<_> = glob::glob(&pattern.to_string_lossy())
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .collect();

        if paths.is_empty() {
            return false;
        }

        match &rule.key {
            None => true,
            Some(key) => paths.iter().any(|p| {
                std::fs::read_to_string(p)
                    .map(|content| file_contains_key(&content, key))
                    .unwrap_or(false)
            }),
        }
    })
}

/// Search `key` in parsed TOML/JSON (key names and string values),
/// falling back to a raw substring search for non-structured files.
fn file_contains_key(content: &str, key: &str) -> bool {
    if let Ok(v) = content.parse::<toml::Value>() {
        return toml_contains(v, key);
    }
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(content) {
        return json_contains(v, key);
    }
    content.contains(key)
}

fn toml_contains(value: toml::Value, target: &str) -> bool {
    match value {
        toml::Value::Table(map) => {
            map.contains_key(target) || map.values().any(|v| toml_contains(v.clone(), target))
        }
        toml::Value::Array(arr) => arr.into_iter().any(|v| toml_contains(v, target)),
        toml::Value::String(s) => s == target || s.contains(target),
        _ => false,
    }
}

fn json_contains(value: serde_json::Value, target: &str) -> bool {
    match value {
        serde_json::Value::Object(map) => {
            map.contains_key(target) || map.into_values().any(|v| json_contains(v, target))
        }
        serde_json::Value::Array(arr) => arr.into_iter().any(|v| json_contains(v, target)),
        serde_json::Value::String(s) => s == target || s.contains(target),
        _ => false,
    }
}

fn resolve_var(rule: &VarRule, project_path: &Path) -> Option<String> {
    let content = std::fs::read_to_string(project_path.join(&rule.file)).ok()?;

    let raw = if let Ok(v) = content.parse::<toml::Value>() {
        find_toml_value(&v, &rule.key)
    } else if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
        find_json_value(&v, &rule.key)
    } else {
        None
    }?;

    Some(if rule.map.is_empty() {
        raw
    } else {
        rule.map.get(&raw).cloned().unwrap_or(raw)
    })
}

fn find_toml_value(value: &toml::Value, key: &str) -> Option<String> {
    match value {
        toml::Value::Table(map) => {
            if let Some(v) = map.get(key) {
                return v.as_str().map(str::to_owned);
            }
            map.values().find_map(|v| find_toml_value(v, key))
        }
        _ => None,
    }
}

fn find_json_value(value: &serde_json::Value, key: &str) -> Option<String> {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(v) = map.get(key) {
                return v.as_str().map(str::to_owned);
            }
            map.values().find_map(|v| find_json_value(v, key))
        }
        _ => None,
    }
}

fn resolve_layer(cmd: &LayerCmd, project_path: &Path) -> String {
    match cmd {
        LayerCmd::Literal(s) => s.clone(),
        LayerCmd::Templated { cmd, var } => {
            let value = resolve_var(var, project_path).unwrap_or_else(|| "latest".to_owned());
            cmd.replace("${var}", &value)
        }
    }
}

fn collect_leaves(
    toolchains: &[Toolchain],
    path: &[String],
    lines: Vec<String>,
    project_path: &Path,
    run_detect: bool,
) -> Vec<Recipe> {
    let mut results = Vec::new();

    for tc in toolchains {
        if run_detect && !tc.detect.is_empty() && !matches_detect(&tc.detect, project_path) {
            continue;
        }

        let mut cur_path = path.to_vec();
        cur_path.push(tc.name.clone());

        let mut cur_lines = lines.clone();
        for layer_cmd in &tc.layer {
            cur_lines.push(resolve_layer(layer_cmd, project_path));
        }

        if let Some(default_cmd) = &tc.default_cmd {
            results.push(Recipe {
                label: cur_path.join(" → "),
                dockerfile: cur_lines.join("\n"),
                default_cmd: default_cmd.clone(),
            });
        } else if !tc.sub.is_empty() {
            results.extend(collect_leaves(
                &tc.sub,
                &cur_path,
                cur_lines,
                project_path,
                run_detect,
            ));
        }
    }

    results
}

/// Compute an image tag for a recipe.
/// `registry` — remote registry prefix (e.g. `"ghcr.io/myorg"`);
///              `None` falls back to `"fork-local"` for local-only builds.
/// Version is extracted from the `FROM image:version` line in the dockerfile.
/// Format: `{registry}/{board}/{label_as_dot_path}:{version}`
pub fn image_tag(registry: Option<&str>, board: &str, recipe: &Recipe) -> String {
    let ns = registry.unwrap_or("fork-local");
    let version = recipe
        .dockerfile
        .lines()
        .find_map(|line| line.strip_prefix("FROM "))
        .and_then(|img| img.split(':').nth(1))
        .unwrap_or("latest");
    let path_tag = recipe.label.replace(" → ", ".");
    format!("{}/{}/{}:{}", ns, board, path_tag, version)
}
