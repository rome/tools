use crate::run_cli;
use crate::snap_test::{CliSnapshot, SnapshotPayload};
use pico_args::Arguments;
use rome_console::BufferConsole;
use rome_fs::MemoryFileSystem;
use rome_service::DynRef;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("rage")]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "rage_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_configuration() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Path::new("rome.json").to_path_buf(),
        r#"{
  "formatter": {
    "enabled": false
  }
}"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("rage")]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn with_malformed_configuration() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        Path::new("rome.json").to_path_buf(),
        r#"{
  "formatter": {
    "enabled":
  }
}"#,
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("rage")]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_rage_snapshot(SnapshotPayload::new(
        module_path!(),
        "with_malformed_configuration",
        fs,
        console,
        result,
    ));
}

fn assert_rage_snapshot(payload: SnapshotPayload<'_>) {
    let test_name = payload.test_name;
    let module_path = payload.module_path;

    let mut snapshot = CliSnapshot::from(payload);

    // Replace any platform specific content that may yield unstable results.
    for message in snapshot.messages.iter_mut() {
        *message = message
            .lines()
            .map(|line| match line.trim_start().split_once(':') {
                Some((
                    "CPU Architecture" | "OS" | "NO_COLOR" | "TERM" | "Color support",
                    value,
                )) => line.replace(value.trim_start(), "**PLACEHOLDER**"),
                _ => line.to_string(),
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    let content = snapshot.emit_content_snapshot();

    let module_path = module_path.replace("::", "_");
    let snapshot_path = PathBuf::from("../snapshots").join(module_path);

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => snapshot_path
    }, {
        insta::assert_snapshot!(test_name, content);

    });
}
