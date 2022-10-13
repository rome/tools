mod commands;
mod configs;
#[cfg(test)]
mod snap_test;

#[cfg(test)]
use snap_test::assert_cli_snapshot;

use std::{ffi::OsString, path::Path};

use pico_args::Arguments;
use rome_cli::{CliSession, Termination};
use rome_console::{BufferConsole, Console};
use rome_fs::{FileSystem, MemoryFileSystem};
use rome_service::{App, DynRef};

const UNFORMATTED: &str = "  statement(  )  ";
const FORMATTED: &str = "statement();\n";

const PARSE_ERROR: &str = "if\n";
const LINT_ERROR: &str = "for(;true;);\n";

const CUSTOM_FORMAT_BEFORE: &str = r#"
function f() {
return { something }
}
"#;

mod help {
    use super::*;

    #[test]
    fn unknown_command() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("unknown"), OsString::from("--help")]),
        );

        match result {
            Err(Termination::UnknownCommandHelp { command }) => assert_eq!(command, "unknown"),
            _ => {
                panic!("run_cli returned {result:?} for an unknown command help, expected an error")
            }
        }
    }
}

mod main {
    use super::*;
    use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;

    #[test]
    fn unknown_command() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("unknown")]),
        );

        match result {
            Err(Termination::UnknownCommand { command }) => assert_eq!(command, "unknown"),
            _ => panic!("run_cli returned {result:?} for an unknown command, expected an error"),
        }
    }

    #[test]
    fn unexpected_argument() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--unknown"),
                OsString::from("file.js"),
            ]),
        );

        match result {
            Err(Termination::UnexpectedArgument { argument, .. }) => {
                assert_eq!(argument, OsString::from("--unknown"))
            }
            _ => panic!("run_cli returned {result:?} for an unknown argument, expected an error"),
        }
    }

    #[test]
    fn empty_arguments() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("format")]),
        );

        match result {
            Err(Termination::EmptyArguments) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }

    #[test]
    fn missing_argument() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("format"), OsString::from("--write")]),
        );

        match result {
            Err(Termination::MissingArgument { argument }) => assert_eq!(argument, "<INPUT>"),
            _ => panic!("run_cli returned {result:?} for a missing argument, expected an error"),
        }
    }

    #[test]
    fn incorrect_value() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--max-diagnostics=foo"),
            ]),
        );

        match result {
            Err(Termination::ParseError { argument, .. }) => {
                assert_eq!(argument, "--max-diagnostics");
            }
            _ => panic!("run_cli returned {result:?} for a malformed, expected an error"),
        }
    }

    #[test]
    fn overflow_value() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--max-diagnostics=500"),
            ]),
        );

        match result {
            Err(Termination::OverflowNumberArgument(argument, limit)) => {
                assert_eq!(argument, "--max-diagnostics");
                assert_eq!(limit, MAXIMUM_DISPLAYABLE_DIAGNOSTICS);
            }
            _ => panic!("run_cli returned {result:?} for a malformed, expected an error"),
        }
    }
}

mod init {
    use super::*;
    use crate::configs::CONFIG_INIT_DEFAULT;
    use crate::snap_test::SnapshotPayload;
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
}

mod configuration {
    use super::*;
    use crate::configs::{
        CONFIG_ALL_FIELDS, CONFIG_BAD_LINE_WIDTH, CONFIG_INCORRECT_GLOBALS,
        CONFIG_INCORRECT_GLOBALS_V2, CONFIG_LINTER_WRONG_RULE,
    };
    use crate::snap_test::SnapshotPayload;
    use pico_args::Arguments;
    use rome_console::BufferConsole;
    use rome_fs::MemoryFileSystem;
    use rome_service::DynRef;
    use std::ffi::OsString;
    use std::path::Path;

    #[test]
    fn correct_root() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_ALL_FIELDS.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("format"), OsString::from("file.js")]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "correct_root",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn line_width_error() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_BAD_LINE_WIDTH.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("format"), OsString::from("file.js")]),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "line_width_error",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn incorrect_rule_name() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_LINTER_WRONG_RULE.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), OsString::from("file.js")]),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "incorrect_rule_name",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn incorrect_globals() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_INCORRECT_GLOBALS.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), OsString::from("file.js")]),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "incorrect_globals",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn ignore_globals() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_INCORRECT_GLOBALS_V2.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), OsString::from("file.js")]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");
    }
}

mod reporter_json {
    use super::*;
    use crate::snap_test::SnapshotPayload;
    use crate::UNFORMATTED;
    use pico_args::Arguments;
    use rome_fs::FileSystemExt;

    #[test]
    fn reports_formatter_check_mode() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                std::ffi::OsString::from("format"),
                std::ffi::OsString::from("--json"),
                file_path.as_os_str().into(),
            ]),
        );

        eprintln!("{:?}", console.out_buffer);

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, UNFORMATTED);

        assert_eq!(console.out_buffer.len(), 1);

        drop(file);
        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "reports_formatter_check_mode",
            fs,
            console,
            result,
        ));
    }

    #[test]
    fn reports_formatter_write() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                std::ffi::OsString::from("format"),
                std::ffi::OsString::from("--write"),
                std::ffi::OsString::from("--json"),
                file_path.as_os_str().into(),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, FORMATTED);

        assert_eq!(console.out_buffer.len(), 1);

        drop(file);

        assert_cli_snapshot(SnapshotPayload::new(
            module_path!(),
            "reports_formatter_write",
            fs,
            console,
            result,
        ));
    }
}

/// Create an [App] instance using the provided [FileSystem] and [Console]
/// instance, and using an in-process "remote" instance of the workspace
pub(crate) fn run_cli<'app>(
    fs: DynRef<'app, dyn FileSystem>,
    console: DynRef<'app, dyn Console>,
    args: Arguments,
) -> Result<(), Termination> {
    use rome_cli::SocketTransport;
    use rome_lsp::ServerFactory;
    use rome_service::{workspace, WorkspaceRef};
    use tokio::{
        io::{duplex, split},
        runtime::Runtime,
    };

    let factory = ServerFactory::default();
    let connection = factory.create();

    let runtime = Runtime::new().expect("failed to create runtime");

    let (client, server) = duplex(4096);
    let (stdin, stdout) = split(server);
    runtime.spawn(connection.accept(stdin, stdout));

    let (client_read, client_write) = split(client);
    let transport = SocketTransport::open(runtime, client_read, client_write);

    let workspace = workspace::client(transport).unwrap();
    let app = App::new(fs, console, WorkspaceRef::Owned(workspace));

    let session = CliSession { app, args };
    session.run()
}
