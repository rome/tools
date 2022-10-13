use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli};
use pico_args::Arguments;
use rome_console::BufferConsole;
use rome_fs::MemoryFileSystem;
use rome_service::DynRef;
use std::ffi::OsString;

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("--version")]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "version_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn full() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("version")]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "version_full",
        fs,
        console,
        result,
    ));
}
