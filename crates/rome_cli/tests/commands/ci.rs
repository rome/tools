use crate::configs::{CONFIG_DISABLED_FORMATTER, CONFIG_LINTER_DISABLED};
use crate::snap_test::SnapshotPayload;
use crate::{
    assert_cli_snapshot, run_cli, CUSTOM_FORMAT_BEFORE, FORMATTED, LINT_ERROR, PARSE_ERROR,
    UNFORMATTED,
};
use pico_args::Arguments;
use rome_cli::Termination;
use rome_console::BufferConsole;
use rome_fs::{FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;
use std::ffi::OsString;
use std::path::Path;

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, FORMATTED);

    if console.out_buffer.len() != 1 {
        panic!("unexpected console content: {:#?}", console.out_buffer);
    }

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn formatting_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
    );

    match result {
        Err(Termination::CheckError) => {}
        _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatting_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_parse_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), PARSE_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
    );

    match &result {
        Err(Termination::CheckError) => {}
        _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_lint_error() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let mut console = BufferConsole::default();
    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
    );

    match &result {
        Err(Termination::CheckError) => {}
        _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_lint_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_formatter() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_linter() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_LINTER_DISABLED.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, CUSTOM_FORMAT_BEFORE);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    // Do not store the content of the file in the snapshot
    fs.remove(file_path);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large",
        fs,
        console,
        result,
    ));
}
