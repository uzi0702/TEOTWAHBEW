//! Integration and system tests for the `teot` binary.
//!
//! Each test creates an isolated temporary directory, runs the binary against it,
//! and asserts on the stdout / exit status.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> Self {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let path = std::env::temp_dir().join(format!("teot_{label}_{pid}_{n}"));
        fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn create_file(&self, name: &str) {
        fs::write(self.0.join(name), b"").unwrap();
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn teot() -> Command {
    Command::new(env!("CARGO_BIN_EXE_teot"))
}

// --- M テスト（結合テスト） ---

#[test]
fn basic_output_contains_filename() {
    let dir = TempDir::new("basic");
    dir.create_file("hello.txt");

    let out = teot().current_dir(dir.path()).output().unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("hello.txt"));
}

#[test]
fn output_line_contains_date_and_size() {
    let dir = TempDir::new("format");
    dir.create_file("data.txt");

    let out = teot().current_dir(dir.path()).output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout.lines().find(|l| l.contains("data.txt")).unwrap();

    // Date pattern: four consecutive digits followed by '-'  (YYYY-)
    let chars: Vec<char> = line.chars().collect();
    let has_year = chars.windows(5).any(|w| {
        w[..4].iter().all(|c| c.is_ascii_digit()) && w[4] == '-'
    });
    assert!(has_year, "output line should contain a date: {line}");

    // Size unit
    assert!(
        line.contains(" B") || line.contains("KB") || line.contains("MB") || line.contains("GB"),
        "output line should contain a human-readable size: {line}"
    );
}

// --- L テスト（システムテスト） ---

#[test]
fn source_only_flag_filters_non_source_files() {
    let dir = TempDir::new("source_only");
    dir.create_file("main.rs");
    dir.create_file("README.md");
    dir.create_file("notes.txt");

    let out = teot().arg("-c").current_dir(dir.path()).output().unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("main.rs"), "main.rs should appear with -c");
    assert!(!stdout.contains("README.md"), "README.md should not appear with -c");
    assert!(!stdout.contains("notes.txt"), "notes.txt should not appear with -c");
}

#[test]
fn sort_by_date_runs_without_error() {
    let dir = TempDir::new("sort_date");
    dir.create_file("a.txt");
    dir.create_file("b.txt");

    let out = teot().arg("-sd").current_dir(dir.path()).output().unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("a.txt"));
    assert!(stdout.contains("b.txt"));
}

#[test]
fn color_flag_adds_ansi_escape_to_all_files() {
    let dir = TempDir::new("color_all");
    dir.create_file("test.txt");

    let out = teot()
        .arg("--color=31")
        .current_dir(dir.path())
        .output()
        .unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\x1b[31m"),
        "output should contain ANSI color code for 31"
    );
}

#[test]
fn color_without_equal_fails_with_error_message() {
    let dir = TempDir::new("color_no_equal");
    dir.create_file("a.txt");

    let out = teot()
        .args(["--color", "31"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    assert!(!out.status.success(), "--color without '=' should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--color=<colorcode>"),
        "stderr should explain the correct form: {stderr}"
    );
}

#[test]
fn color_name_fails_with_error_message() {
    let dir = TempDir::new("color_name");
    dir.create_file("a.txt");

    let out = teot()
        .arg("--color=red")
        .current_dir(dir.path())
        .output()
        .unwrap();

    assert!(!out.status.success(), "--color=red should fail");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("invalid color code"),
        "stderr should report the invalid code: {stderr}"
    );
}

#[test]
fn color_with_target_applies_only_to_named_file() {
    let dir = TempDir::new("color_target");
    dir.create_file("target.rs");
    dir.create_file("other.txt");

    let out = teot()
        .args(["--color=31", "target.rs"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);

    let target_line = stdout.lines().find(|l| l.contains("target.rs")).unwrap();
    assert!(
        target_line.contains("\x1b[31m"),
        "target.rs line should have color: {target_line}"
    );

    let other_line = stdout.lines().find(|l| l.contains("other.txt")).unwrap();
    assert!(
        !other_line.contains("\x1b[31m"),
        "other.txt line should not have color 31: {other_line}"
    );
}
