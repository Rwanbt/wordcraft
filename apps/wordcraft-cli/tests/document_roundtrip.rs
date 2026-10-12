//! End-to-end CLI contracts: the actual compiled binary creates, edits, reads and
//! converts synthetic documents. No installed GUI, network or personal files needed.
#![cfg(not(target_arch = "wasm32"))]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("wordcraft-cli-e2e-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir(&path).expect("unique test directory");
        Self(path)
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_wordcraft-cli")).args(args).output().expect("spawn actual wordcraft-cli")
}

fn success(output: &Output) {
    assert!(
        output.status.success(),
        "status: {:?}, stdout: {}, stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("synthetic directory path is UTF-8")
}

#[test]
fn real_cli_creates_reopens_edits_and_exports_a_document() {
    let d = Scratch::new();
    let original = d.path("first.docx");
    let changed = d.path("second.docx");
    let export = d.path("export.txt");
    const MARKER: &str = "Synthetic CLI document sentinel 4217";
    const EDIT: &str = " appended edit";

    success(&cli(&[
        "run",
        "--cmd",
        r#"text.insert={"text":"Synthetic CLI document sentinel 4217"}"#,
        "--save",
        path_str(&original),
    ]));
    let bytes = std::fs::read(&original).expect("DOCX saved");
    assert!(bytes.starts_with(b"PK"), "a DOCX should be a ZIP archive");
    assert!(bytes.len() > 100, "not an empty document");

    let reopened = cli(&["text", path_str(&original)]);
    success(&reopened);
    assert!(String::from_utf8_lossy(&reopened.stdout).contains(MARKER));

    success(&cli(&[
        "run",
        "--file",
        path_str(&original),
        "--cmd",
        "caret.docEnd",
        "--cmd",
        r#"text.insert={"text":" appended edit"}"#,
        "--save",
        path_str(&changed),
    ]));
    let changed_text = cli(&["text", path_str(&changed)]);
    success(&changed_text);
    let changed_string = String::from_utf8_lossy(&changed_text.stdout);
    assert!(changed_string.contains(MARKER), "reopened content was lost: {changed_string}");
    assert!(changed_string.contains(EDIT), "new text was lost: {changed_string}");

    success(&cli(&["convert", path_str(&changed), path_str(&export)]));
    let exported = std::fs::read_to_string(&export).expect("TXT output");
    assert!(exported.contains(MARKER));
    assert!(exported.contains(EDIT));

    let still_original = cli(&["text", path_str(&original)]);
    success(&still_original);
    assert!(String::from_utf8_lossy(&still_original.stdout).contains(MARKER));
    assert!(!String::from_utf8_lossy(&still_original.stdout).contains(EDIT));

    let info = cli(&["info", path_str(&changed)]);
    success(&info);
    let report: serde_json::Value = serde_json::from_slice(&info.stdout).expect("valid JSON info");
    assert!(report.is_object(), "info must be machine readable");
}

#[test]
fn failed_conversion_leaves_existing_destination_byte_identical() {
    let d = Scratch::new();
    let missing = d.path("not-there.docx");
    let destination = d.path("existing.txt");
    let sentinel = b"Keep exact destination bytes and this trailing newline.\n";
    std::fs::write(&destination, sentinel).expect("create existing output");
    let result = cli(&["convert", path_str(&missing), path_str(&destination)]);
    assert!(!result.status.success(), "missing source unexpectedly succeeded");
    assert_eq!(std::fs::read(&destination).expect("preserved destination"), sentinel);
    assert!(!missing.exists());
    assert!(!String::from_utf8_lossy(&result.stderr).contains("panicked"));
}
