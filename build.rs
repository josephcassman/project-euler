use std::{env, fs};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

///
/// Automate the selection of modules in main.rs.
///
/// This build script finds all modules in src/problems
/// and then adds them to a build-artifact module which
/// is then used to map problem numbers provided on the
/// command line to the problem module containing
/// the code to run.
///

#[derive(Default)]
struct ModuleTree {
    files: Vec<(String, PathBuf)>,
    children: BTreeMap<String, ModuleTree>,
}

fn main () {
    let problems_dir = Path::new("src/problems");
    println!("cargo:rerun-if-changed={}", problems_dir.display());

    let mut tree = ModuleTree::default();
    let mut problems = Vec::new();
    collect_modules(problems_dir, problems_dir, &mut tree, &mut problems);
    problems.sort_by_key(|(number, _)| *number);

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out_dir.join("problem_modules.rs"), render_modules(&tree, 0)).unwrap();

    let mut registry = String::from("problems!(\n");
    for (number, path) in problems {
        registry.push_str(&format!(
            "    ({number}, project_euler::problems::{path}::run),\n"
        ));
    }
    registry.push_str(");\n");
    fs::write(out_dir.join("problem_registry.rs"), registry).unwrap();
}

fn collect_modules (
    root: &Path,
    directory: &Path,
    tree: &mut ModuleTree,
    problems: &mut Vec<(u32, String)>)
{
    let entries = fs::read_dir(directory).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap().to_owned();
            let child = tree.children.entry(name.clone()).or_default();
            collect_modules(root, &path, child, problems);
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else { continue; };
        let Some(number_text) = stem.strip_prefix('p') else { continue; };
        if number_text.len() != 4
            || !number_text
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            continue;
        }

        let number = number_text.parse::<u32>().unwrap();
        let module_path = path.strip_prefix(root).unwrap().with_extension("");
        let module_path = module_path
            .iter()
            .map(|part| part.to_str().unwrap())
            .collect::<Vec<_>>()
            .join("::");
        problems.push((number, module_path));
        tree.files.push((stem.to_owned(), path));
    }
}

fn render_modules (tree: &ModuleTree, depth: usize) -> String {
    let indent = "    ".repeat(depth);
    let mut output = String::new();

    for (name, path) in &tree.files {
        output.push_str(&format!(
            "{indent}#[path = {}]\npub mod {name};\n",
            rust_string(path),
        ));
    }

    for (name, child) in &tree.children {
        output.push_str(&format!("{indent}pub mod {name} {{\n"));
        output.push_str(&render_modules(child, depth + 1));
        output.push_str(&format!("{indent}}}\n"));
    }

    output
}

fn rust_string (path: &Path) -> String {
    let path = path
        .canonicalize()
        .unwrap()
        .to_string_lossy()
        .replace('\\', "\\\\");
    format!("\"{path}\"")
}
