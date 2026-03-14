use std::io::{self, Read};
use std::path::Path;
use toml::Value;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "supports" {
        std::process::exit(0);
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read stdin");

    let mut json: serde_json::Value =
        serde_json::from_str(&input).expect("failed to parse mdbook JSON");

    let root = json[0]["root"]
        .as_str()
        .expect("missing root in context")
        .to_owned();

    let recipes_dir = Path::new(&root).join("../../recipes");
    let content = generate_recipes_md(&recipes_dir);

    replace_chapter(&mut json[1]["sections"], "recipes.md", &content);

    print!("{}", json[1]);
}

fn replace_chapter(sections: &mut serde_json::Value, target_path: &str, content: &str) {
    let Some(arr) = sections.as_array_mut() else {
        return;
    };
    for item in arr.iter_mut() {
        if let Some(ch) = item.get_mut("Chapter") {
            let path_matches = ch["path"]
                .as_str()
                .map(|p| p.replace('\\', "/") == target_path)
                .unwrap_or(false);
            if path_matches {
                ch["content"] = serde_json::Value::String(content.to_owned());
                return;
            }
            if let Some(sub) = ch.get_mut("sub_items") {
                replace_chapter(sub, target_path, content);
            }
        }
    }
}

fn generate_recipes_md(recipes_dir: &Path) -> String {
    let mut entries: Vec<_> = std::fs::read_dir(recipes_dir)
        .expect("cannot read recipes/ directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("toml"))
        .collect();
    entries.sort();

    let mut out = String::from("# Supported recipes\n\n");
    out.push_str("Each board ships with definitions for the toolchains listed below. Fork auto-selects the right one based on your workspace files.\n");

    for path in &entries {
        let src = std::fs::read_to_string(path).expect("failed to read board TOML");
        let value: Value = src.parse().expect("invalid TOML");
        let table = value.as_table().expect("board TOML must be a table");

        let name = table
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let recipes = collect_recipes(table, vec![], vec![]);

        out.push_str(&format!("\n## {}\n\n", board_title(name)));
        out.push_str("| Recipe | Detected by |\n");
        out.push_str("|--------|-------------|\n");

        for (label, detect) in &recipes {
            let detect_str = if detect.is_empty() {
                "—".to_owned()
            } else {
                detect.join(", ")
            };
            out.push_str(&format!("| `{}` | {} |\n", label, detect_str));
        }
    }

    out
}

fn collect_recipes(
    table: &toml::map::Map<String, Value>,
    path: Vec<String>,
    parent_detect: Vec<String>,
) -> Vec<(String, Vec<String>)> {
    let local_detect = extract_detect(table);
    let mut combined = parent_detect;
    combined.extend(local_detect);

    if table.contains_key("cmd") {
        let label = if path.is_empty() {
            "default".to_owned()
        } else {
            path.join(" → ")
        };
        return vec![(label, combined)];
    }

    let mut results = Vec::new();
    for (key, value) in table {
        if matches!(key.as_str(), "detect" | "layer" | "cmd" | "name") {
            continue;
        }
        if let Value::Table(sub) = value {
            let mut sub_path = path.clone();
            sub_path.push(key.clone());
            results.extend(collect_recipes(sub, sub_path, combined.clone()));
        }
    }
    results
}

fn extract_detect(table: &toml::map::Map<String, Value>) -> Vec<String> {
    let Some(Value::Array(rules)) = table.get("detect") else {
        return vec![];
    };
    rules
        .iter()
        .filter_map(|rule| {
            let t = rule.as_table()?;
            let file = t.get("file")?.as_str()?;
            Some(match t.get("key").and_then(|v| v.as_str()) {
                Some(key) => format!("`{}` contains `{}`", file, key),
                None => format!("`{}` exists", file),
            })
        })
        .collect()
}

fn board_title(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_uppercase().to_string() + &name[1..])
        .unwrap_or_default()
}
