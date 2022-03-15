use std::{ffi::OsString, path::Path, sync::Arc};

use pico_args::Arguments;
use rome_cli::{run_cli, CliSession};
use rome_core::App;
use rome_fs::{FileSystem, MemoryFileSystem};

#[test]
fn test_format_cli() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), b"statement()".as_slice());

    let fs = Arc::new(fs);
    let result = run_cli(CliSession {
        app: App::with_filesystem(fs.clone()),
        args: Arguments::from_vec(vec![OsString::from("format"), file_path.as_os_str().into()]),
    });

    assert_eq!(result, Ok(()));

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, "statement();\n");
}
