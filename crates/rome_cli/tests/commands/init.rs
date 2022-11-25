use crate::configs::{CONFIG_INIT_DEFAULT, CONFIG_INIT_DEFAULT_WHEN_INSTALLED};
use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use pico_args::Arguments;
use rome_console::BufferConsole;
use rome_fs::{FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;
use std::ffi::OsString;
use std::path::Path;

#[test]
fn creates_config_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("init")]),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    let file_path = Path::new("rome.json");

    let mut file = fs
        .open(file_path)
        .expect("configuration file was not written on disk");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");
    assert_eq!(content, CONFIG_INIT_DEFAULT);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "creates_config_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn creates_config_file_when_rome_installed_via_package_manager() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("./node_modules/rome/configuration_schema.json");
    fs.insert(file_path.into(), *b"{}");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("init")]),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    let file_path = Path::new("rome.json");

    let mut file = fs
        .open(file_path)
        .expect("configuration file was not written on disk");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");
    assert_eq!(content, CONFIG_INIT_DEFAULT_WHEN_INSTALLED);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "creates_config_file_when_rome_installed_via_package_manager",
        fs,
        console,
        result,
    ));
}
