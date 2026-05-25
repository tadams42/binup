use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/apps/");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);

    update_readme(manifest_dir);
}

fn category_display_name(category: &str) -> &str {
    match category {
        "containers" => "Containers",
        "data" => "Data",
        "dev_envs" => "Dev envs",
        "dev_tools" => "Dev tools",
        "files" => "Files",
        "git" => "Git",
        "http" => "HTTP",
        "logs" => "Logs",
        "other" => "Other",
        "shell" => "Shell",
        other => other,
    }
}

fn update_readme(manifest_dir: &Path) {
    let mod_path = manifest_dir.join("src/apps/mod.rs");
    let readme_path = manifest_dir.join("README.md");

    let mod_content = fs::read_to_string(&mod_path).expect("Failed to read src/apps/mod.rs");

    let mut entries: Vec<(String, String, String, String)> =
        parse_app_entries(manifest_dir, &mod_content);

    entries.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));

    let mut app_lines: Vec<String> = Vec::new();
    let mut current_category: Option<&str> = None;
    for (id, url, cat, description) in &entries {
        let cat_str = cat.as_str();
        if current_category != Some(cat_str) {
            if current_category.is_some() {
                app_lines.push(String::new());
            }
            app_lines.push(format!("### {}", category_display_name(cat_str)));
            app_lines.push(String::new());
            current_category = Some(cat_str);
        }
        app_lines.push(format!("- [{}]({}) — {}", id, url, description));
    }

    let readme = fs::read_to_string(&readme_path).expect("Failed to read README.md");
    let lines: Vec<&str> = readme.lines().collect();

    let idx_first = lines
        .iter()
        .position(|l| *l == "## Supported apps")
        .expect("'Supported apps' marker not found in README.md");
    let idx_last = lines
        .iter()
        .position(|l| l.starts_with("[^1]: "))
        .expect("'[^1]:' marker not found in README.md");

    let mut new_lines: Vec<String> = lines[..=idx_first].iter().map(|l| l.to_string()).collect();
    new_lines.push(String::new());
    new_lines.extend(app_lines);
    new_lines.push(String::new());
    new_lines.extend(lines[idx_last..].iter().map(|l| l.to_string()));

    let new_content = new_lines.join("\n") + "\n";
    fs::write(&readme_path, new_content).expect("Failed to write README.md");
}

fn parse_app_entries(manifest_dir: &Path, content: &str) -> Vec<(String, String, String, String)> {
    let mut entries = Vec::new();
    let mut pending: Option<(Option<String>, Option<String>, Option<String>, Option<String>)> =
        None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("AppEntry {") {
            let id =
                extract_path_field(line, "id:").and_then(|p| resolve_const(manifest_dir, &p, "ID"));
            let url = extract_path_field(line, "url:")
                .and_then(|p| resolve_const(manifest_dir, &p, "URL"));
            let cat = extract_quoted_field(line, "category:");
            let desc = extract_path_field(line, "description:")
                .and_then(|p| resolve_const(manifest_dir, &p, "DESCRIPTION"));
            if let (Some(id), Some(url), Some(cat), Some(desc)) =
                (id.clone(), url.clone(), cat.clone(), desc.clone())
            {
                entries.push((id, url, cat, desc));
            } else {
                pending = Some((id, url, cat, desc));
            }
        } else if let Some((ref mut id, ref mut url, ref mut cat, ref mut desc)) = pending {
            if id.is_none() {
                *id = extract_path_field(line, "id:")
                    .and_then(|p| resolve_const(manifest_dir, &p, "ID"));
            }
            if url.is_none() {
                *url = extract_path_field(line, "url:")
                    .and_then(|p| resolve_const(manifest_dir, &p, "URL"));
            }
            if cat.is_none() {
                *cat = extract_quoted_field(line, "category:");
            }
            if desc.is_none() {
                *desc = extract_path_field(line, "description:")
                    .and_then(|p| resolve_const(manifest_dir, &p, "DESCRIPTION"));
            }
            if id.is_some() && url.is_some() && cat.is_some() && desc.is_some() {
                entries.push((
                    id.take().unwrap(),
                    url.take().unwrap(),
                    cat.take().unwrap(),
                    desc.take().unwrap(),
                ));
                pending = None;
            }
        }
    }
    entries
}

fn resolve_const(manifest_dir: &Path, path_expr: &str, const_name: &str) -> Option<String> {
    let suffix = format!("::{const_name}");
    let without_const = path_expr.trim_end_matches(suffix.as_str());
    let parts: Vec<&str> = without_const.split("::").collect();

    // Full module path style: "files::ripgrep::Ripgrep::URL"
    if parts.len() >= 2 {
        let module_parts = &parts[..parts.len() - 1];
        let file_path = manifest_dir
            .join("src/apps")
            .join(module_parts.join("/"))
            .with_extension("rs");
        if let Some(val) = fs::read_to_string(&file_path)
            .ok()
            .and_then(|c| read_const_from_content(&c, const_name))
        {
            return Some(val);
        }
    }

    // Short name style: "Ripgrep::URL" — scan all app source files
    let struct_name = parts.last()?;
    find_const_in_apps(manifest_dir, struct_name, const_name)
}

fn read_const_from_content(content: &str, const_name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let search = format!("pub const {const_name}:");
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with(&search) {
            if let Some(val) = extract_quoted_field(trimmed, "=") {
                return Some(val);
            }
            if let Some(next) = lines.get(i + 1) {
                let next = next.trim().strip_prefix('"')?;
                return Some(next[..next.find('"')?].to_string());
            }
        }
    }
    None
}

fn find_const_in_apps(manifest_dir: &Path, struct_name: &str, const_name: &str) -> Option<String> {
    walk_rs_files_for_const(&manifest_dir.join("src/apps"), struct_name, const_name)
}

fn walk_rs_files_for_const(dir: &Path, struct_name: &str, const_name: &str) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            if let Some(val) = walk_rs_files_for_const(&path, struct_name, const_name) {
                return Some(val);
            }
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Some(val) = try_const_from_file(&path, struct_name, const_name) {
                return Some(val);
            }
        }
    }
    None
}

fn try_const_from_file(file_path: &Path, struct_name: &str, const_name: &str) -> Option<String> {
    let content = fs::read_to_string(file_path).ok()?;
    if !content.contains(&format!("pub struct {struct_name}")) {
        return None;
    }
    read_const_from_content(&content, const_name)
}

fn extract_path_field(line: &str, field: &str) -> Option<String> {
    let start = line.find(field)? + field.len();
    let rest = line[start..].trim_start();
    if rest.starts_with('"') {
        return None; // string literal, not a path expression
    }
    let end = rest
        .find(',')
        .unwrap_or_else(|| rest.find('}').unwrap_or(rest.len()));
    let expr = rest[..end].trim().to_string();
    if expr.is_empty() { None } else { Some(expr) }
}

fn extract_quoted_field(line: &str, field: &str) -> Option<String> {
    let start = line.find(field)? + field.len();
    let rest = line[start..].trim_start().strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
