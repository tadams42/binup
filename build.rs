use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/apps/mod.rs");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mod_path = Path::new(&manifest_dir).join("src/apps/mod.rs");
    let readme_path = Path::new(&manifest_dir).join("README.md");

    let mod_content = fs::read_to_string(&mod_path).expect("Failed to read src/apps/mod.rs");

    let mut entries: Vec<(String, String)> = mod_content
        .lines()
        .filter_map(parse_app_entry)
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let app_lines: Vec<String> = entries
        .iter()
        .map(|(id, url)| format!("- [{}]({})", id, url))
        .collect();

    let readme = fs::read_to_string(&readme_path).expect("Failed to read README.md");
    let lines: Vec<&str> = readme.lines().collect();

    let idx_first = lines
        .iter()
        .position(|l| *l == "Supported apps:")
        .expect("'Supported apps:' marker not found in README.md");
    let idx_last = lines
        .iter()
        .position(|l| *l == "## How to use it?")
        .expect("'## How to use it?' marker not found in README.md");

    let mut new_lines: Vec<String> =
        lines[..=idx_first].iter().map(|l| l.to_string()).collect();
    new_lines.push(String::new());
    new_lines.extend(app_lines);
    new_lines.push(String::new());
    new_lines.extend(lines[idx_last..].iter().map(|l| l.to_string()));

    let new_content = new_lines.join("\n") + "\n";
    fs::write(&readme_path, new_content).expect("Failed to write README.md");
}

fn parse_app_entry(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if !line.starts_with("AppEntry {") {
        return None;
    }
    let id = extract_quoted_field(line, "id:")?;
    let url = extract_quoted_field(line, "url:")?;
    Some((id, url))
}

fn extract_quoted_field(line: &str, field: &str) -> Option<String> {
    let start = line.find(field)? + field.len();
    let rest = line[start..].trim_start().strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
