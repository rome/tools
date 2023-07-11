mod cases;
mod commands;
mod configs;
#[cfg(test)]
mod snap_test;

#[cfg(test)]
use snap_test::assert_cli_snapshot;

use bpaf::ParseFailure;
use std::path::Path;

use rome_cli::{rome_command, CliDiagnostic, CliSession};
use rome_console::{markup, BufferConsole, Console, ConsoleExt};
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
    use bpaf::Args;

    #[test]
    fn unknown_command() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("unknown"), ("--help")].as_slice()),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");
    }
}

mod main {
    use super::*;
    use bpaf::Args;

    #[test]
    fn unknown_command() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("unknown")].as_slice()),
        );
        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn unexpected_argument() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("format"), ("--unknown"), ("file.js")].as_slice()),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn empty_arguments() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("format")].as_slice()),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn missing_argument() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("format"), ("--write")].as_slice()),
        );
        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn incorrect_value() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("check"), ("--max-diagnostics=foo")].as_slice()),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");
    }

    #[test]
    fn overflow_value() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("check"), ("--max-diagnostics=500")].as_slice()),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");
    }
    //
    // #[test]
    // fn no_colors() {
    //     let mut args = Args::from([("--colors=off")]);
    //     let result = color_from_arguments(&mut args);
    //
    //     assert!(result.is_ok(), "run_cli returned {result:?}");
    // }
    //
    // #[test]
    // fn force_colors() {
    //     let mut args = Args::from([("--colors=force")]);
    //     let result = color_from_arguments(&mut args);
    //
    //     assert!(result.is_ok(), "run_cli returned {result:?}");
    // }
    //
    // #[test]
    // fn invalid_colors() {
    //     let mut args = Args::from([("--colors=other")]);
    //     let result = color_from_arguments(&mut args);
    //     assert!(result.is_err(), "run_cli returned {result:?}");
    // }
}

mod configuration {
    use super::*;
    use crate::configs::{
        CONFIG_ALL_FIELDS, CONFIG_BAD_LINE_WIDTH, CONFIG_INCORRECT_GLOBALS,
        CONFIG_INCORRECT_GLOBALS_V2, CONFIG_LINTER_WRONG_RULE,
    };
    use crate::snap_test::SnapshotPayload;
    use bpaf::Args;
    use rome_console::BufferConsole;
    use rome_fs::MemoryFileSystem;
    use rome_service::DynRef;
    use std::path::Path;

    #[test]
    fn correct_root() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_ALL_FIELDS.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("format"), ("file.js")].as_slice()),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");

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
            &mut console,
            Args::from([("format"), ("file.js")].as_slice()),
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
            &mut console,
            Args::from([("check"), ("file.js")].as_slice()),
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
            &mut console,
            Args::from([("check"), ("file.js")].as_slice()),
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

        fs.insert(
            Path::new("rome.json").into(),
            CONFIG_INCORRECT_GLOBALS_V2.as_bytes(),
        );
        fs.insert(Path::new("file.js").into(), UNFORMATTED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from([("check"), ("file.js")].as_slice()),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");
    }
}

mod reporter_json {
    use super::*;
    use crate::snap_test::SnapshotPayload;
    use crate::UNFORMATTED;
    use bpaf::Args;
    use rome_fs::FileSystemExt;

    #[test]
    fn reports_formatter_check_mode() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            &mut console,
            Args::from(
                [
                    ("format"),
                    ("--json"),
                    file_path.as_os_str().to_str().unwrap(),
                ]
                .as_slice(),
            ),
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
            &mut console,
            Args::from(
                [
                    "format",
                    "--write",
                    "--json",
                    file_path.as_os_str().to_str().unwrap(),
                ]
                .as_slice(),
            ),
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
    console: &'app mut dyn Console,
    args: bpaf::Args,
) -> Result<(), CliDiagnostic> {
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
    let app = App::new(console, WorkspaceRef::Owned(workspace));

    let mut session = CliSession { app };
    let command = rome_command().run_inner(args);
    match command {
        Ok(command) => session.run(command),
        Err(failure) => {
            if let ParseFailure::Stdout(help, _) = &failure {
                let console = &mut session.app.console;
                console.log(markup! {{help.to_string()}});
                Ok(())
            } else {
                Err(CliDiagnostic::parse_error_bpaf(failure))
            }
        }
    }
}
