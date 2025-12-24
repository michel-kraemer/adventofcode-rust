use std::{
    fs::{self, File, FileTimes},
    path::Path,
};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, bin_name = "cargo")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    BenchAoc {
        /// Paths of the projects to benchmark
        #[arg(required = true)]
        paths: Vec<String>,
    },
}

/// Copy a project from one directory to another. Skip the `target` directory,
/// `Cargo.toml`, and `main.rs`. Also skip `Cargo.lock` unless it does not exist
/// in the destination directory yet.
fn copy_project(src: &Path, dest: &Path) -> Result<()> {
    let target_path = src.join("target");
    for entry in WalkDir::new(src).into_iter().filter_entry(|e| {
        !(
            // skip `target` directory
            (e.file_type().is_dir() && e.path() == target_path) ||
            // skip Cargo.toml
            e.file_name() == "Cargo.toml" ||
            // skip main.rs
            e.file_name() == "main.rs"
        )
    }) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(src)?;
        let dest = dest.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(dest)?;
        } else if entry.file_name() != "Cargo.lock" || !fs::exists(&dest)? {
            let metadata = entry.metadata()?;
            fs::copy(entry.path(), &dest)?;
            let f = File::open(dest)?;
            let times = FileTimes::new()
                .set_accessed(metadata.accessed()?)
                .set_modified(metadata.modified()?);
            f.set_times(times)?;
        }
    }

    Ok(())
}

/// Writes a slice as the entire contents of a file unless the file already
/// exists and the contents are equal.
fn write_file_if_necessary(file_path: &Path, contents: &str) -> Result<()> {
    if fs::exists(file_path)? {
        let old_contents = fs::read_to_string(file_path)?;
        if old_contents != contents {
            fs::write(file_path, contents)?;
        }
    } else {
        fs::write(file_path, contents)?;
    }
    Ok(())
}

/// Replace the `main` function in a `main.rs` contents string with
/// `__bench_aoc_main`.
fn replace_main_fn(main_rs: &str) -> Result<String> {
    let Some(main_fn_pos) = main_rs.find("fn main() {") else {
        bail!("Unable to find main function in main.rs");
    };
    let mut result = String::new();
    result.push_str(&main_rs[..main_fn_pos]);
    result.push_str("fn __bench_aoc_main() {");
    result.push_str(&main_rs[main_fn_pos + 11..]);
    Ok(result)
}

/// Replace all `fs::read_to_string` calls in a `main.rs` contents string with
/// `__aoc_bench_read_to_string`.
fn replace_read_input(main_rs: &str) -> String {
    main_rs.replace("fs::read_to_string", "__aoc_bench_read_to_string")
}

/// Replace all `println!` calls in a `main.rs` contents string with `format!`
/// calls. Wrap them in [std::hint::black_box] so they won't be optimized away.
fn replace_printlns(main_rs: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    for (start, part) in main_rs.match_indices("println!(") {
        result.push_str(&main_rs[last_end..start]);
        let mut depth = 1;
        let mut skipped = 0;
        for c in main_rs[start + part.len()..].chars() {
            skipped += 1;
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
            }
            if depth == 0 {
                break;
            }
        }
        last_end = start + part.len() + skipped;
        result.push_str("let _ = std::hint::black_box(format!(");
        result.push_str(&main_rs[start + part.len()..last_end]);
        result.push(')');
    }
    result.push_str(&main_rs[last_end..]);
    result
}

/// Add boilerplate code for Criterion to a `main.rs` contents string
fn add_criterion_boilerplate(main_rs: &mut String, benchmark_name: String) {
    main_rs.push_str(&format!(
        r#"
fn __aoc_bench_criterion_benchmark(c: &mut criterion::Criterion) {{
    c.bench_function("{benchmark_name}", |b| b.iter(__bench_aoc_main));
}}

criterion::criterion_group!(__aoc_bench, __aoc_bench_criterion_benchmark);
criterion::criterion_main!(__aoc_bench);
"#
    ));
}

/// Add boilerplate code that reads input files
fn add_read_input_boilerplate(main_rs: &mut String, input_files: Vec<String>) {
    main_rs.push_str(
        r#"
#[inline(always)]
fn __aoc_bench_read_to_string(path: &str) -> std::io::Result<String> {
"#,
    );

    // inline known input files
    for name in input_files {
        main_rs.push_str(&format!(
            r#"
        if path == "{name}" {{
            Ok(include_str!("../{name}").to_string())
        }} else
    "#,
        ));
    }

    // fallback if an input file could not be inlined
    main_rs.push_str(
        r#"
    {
        eprintln!("Warning: Reading input from {path}");
        fs::read_to_string(path)
    }
}
"#,
    );
}

/// Find all possible input files in the project directory at `path` and return
/// their names. Do not decent into subdirectories. All files with the extension
/// `.txt` are considered input files.
fn find_input_files(path: &Path) -> Result<Vec<String>> {
    let mut result = Vec::new();
    for entry in WalkDir::new(path).max_depth(1) {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "txt") {
            result.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Ok(result)
}

/// Read the `src/main.rs` file from the project directory at `path`, patch it,
/// and write the results to the copied project directory at `bench_aoc_path`.
fn patch_main_rs(path: &Path, bench_aoc_path: &Path, benchmark_name: String) -> Result<()> {
    let orig_main_rs_path = path.join("src").join("main.rs");
    let dest_main_rs_path = bench_aoc_path.join("src").join("__bench_aoc_main.rs");
    let mut main_rs = fs::read_to_string(&orig_main_rs_path)?;

    main_rs = replace_main_fn(&main_rs)?;
    main_rs = replace_read_input(&main_rs);
    main_rs = replace_printlns(&main_rs);
    add_criterion_boilerplate(&mut main_rs, benchmark_name);

    let input_files = find_input_files(path)?;
    add_read_input_boilerplate(&mut main_rs, input_files);

    write_file_if_necessary(&dest_main_rs_path, &main_rs)
}

/// Read the `Cargo.toml` file from the project directory at `path`, patch it,
/// and write the results to the copied project directory at `bench_aoc_path`.
/// Adds `Criterion` to the dependencies and adds the configuration for `cargo
/// bench`. Also converts all relative dependency paths to absolute ones.
fn patch_cargo_toml(path: &Path, bench_aoc_path: &Path) -> Result<String> {
    let orig_cargo_toml_path = path.join("Cargo.toml");
    let dest_cargo_toml_path = bench_aoc_path.join("Cargo.toml");
    let cargo_toml = fs::read_to_string(&orig_cargo_toml_path)?;
    let mut document = cargo_toml.parse::<DocumentMut>()?;

    let benchmark_name = document["package"]["name"]
        .as_str()
        .context("Project does not have a name")?
        .to_string();

    let dependencies_table = document
        .entry("dependencies")
        .or_insert(Table::default().into())
        .as_table_mut()
        .unwrap();
    dependencies_table["criterion"] = value("0");

    for (_, v) in dependencies_table.iter_mut() {
        if let Some(t) = v.as_table_like_mut()
            && let Some(path_value) = t.get_mut("path")
            && let Some(dependency_path) = path_value.as_str()
        {
            let relative_path = Path::new(dependency_path);
            if relative_path.is_relative() {
                let absolute_path = path.join(relative_path).canonicalize()?;
                *path_value = value(absolute_path.to_string_lossy().as_ref());
            }
        }
    }

    let mut bench_table = Table::default();
    bench_table["name"] = value(&benchmark_name);
    bench_table["harness"] = value(false);
    bench_table["path"] = value("src/__bench_aoc_main.rs");

    let mut bench_array = ArrayOfTables::default();
    bench_array.push(bench_table);
    document["bench"] = Item::ArrayOfTables(bench_array);

    write_file_if_necessary(&dest_cargo_toml_path, &document.to_string())?;

    Ok(benchmark_name)
}

/// Benchmark a project at the given `path`
fn bench(path: &str) -> Result<()> {
    let path = std::path::PathBuf::from(path);

    // look for Cargo.toml
    if !fs::exists(path.join("Cargo.toml"))? {
        bail!("Unable to find Cargo.toml in path {path:?}");
    }

    // look for src/main.rs
    if !fs::exists(path.join("src").join("main.rs"))? {
        bail!("Unable to find src/main.rs in path {path:?}");
    }

    // create temporary project directory
    let target_path = path.join("target");
    let bench_aoc_path = target_path.join("bench-aoc");
    fs::create_dir_all(&bench_aoc_path)?;

    copy_project(&path, &bench_aoc_path)?;

    let benchmark_name = patch_cargo_toml(&path, &bench_aoc_path)?;
    patch_main_rs(&path, &bench_aoc_path, benchmark_name)?;

    let mut process = std::process::Command::new("cargo")
        .args(["bench", "--target-dir", ".."])
        .current_dir(bench_aoc_path)
        .spawn()?;
    process.wait()?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let Command::BenchAoc { paths } = args.command;

    for path in paths {
        bench(&path)?;
    }

    Ok(())
}
