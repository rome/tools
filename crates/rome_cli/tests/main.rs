use std::{ffi::OsString, path::Path};

use pico_args::Arguments;
use rome_cli::{run_cli, CliSession, Termination};
use rome_console::BufferConsole;
use rome_fs::{FileSystem, MemoryFileSystem};
use rome_service::{App, DynRef};

const UNFORMATTED: &str = "  statement(  )  ";
const FORMATTED: &str = "statement();\n";

const PARSE_ERROR: &str = "if\n";
const LINT_ERROR: &str = "for(;true;);\n";

const FIX_BEFORE: &str = "
var a, b, c;
var d, e, f;
var g, h, i;
";
const FIX_AFTER: &str = "
var a;
var b;
var c;
var d;
var e;
var f;
var g;
var h;
var i;
";

const CUSTOM_FORMAT_BEFORE: &str = r#"
function f() {
return { something }
}
"#;

// six spaces
const CUSTOM_FORMAT_AFTER: &str = r#"function f() {
      return { something };
}
"#;

const CONFIG_FORMAT: &str = r#"{
  "root": true,
  "formatter": {
    "enabled": true,
    "lineWidth": 160,
    "indentStyle": "space",
    "indentSize": 6
  }
}
"#;

const CONFIG_DISABLED_FORMATTER: &str = r#"{
  "root": true,
  "formatter": {
    "enabled": false
  }
}
"#;

mod check {
    use super::*;

    #[test]
    fn ok() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), FORMATTED.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(fs)),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");
    }

    #[test]
    fn parse_error() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), PARSE_ERROR.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(fs)),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        });

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }

    #[test]
    #[ignore = "lint errors are disabled until the linter is stable"]
    fn lint_error() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), LINT_ERROR.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(fs)),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        });

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }

    #[test]
    fn apply_ok() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Borrowed(&mut console),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        });

        println!("{console:#?}");

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, FIX_AFTER);
    }

    #[test]
    fn apply_noop() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_AFTER.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Borrowed(&mut console),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        });

        println!("{console:#?}");

        assert!(result.is_ok(), "run_cli returned {result:?}");
    }
}

mod ci {
    use super::*;

    #[test]
    fn ok() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("ci.js");
        fs.insert(file_path.into(), FORMATTED.as_bytes());

        let mut console = BufferConsole::default();
        let app = App::with_filesystem_and_console(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
        );

        let result = run_cli(CliSession {
            app,
            args: Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, FORMATTED);

        if console.buffer.len() != 1 {
            panic!("unexpected console content: {:#?}", console.buffer);
        }
    }

    #[test]
    fn formatting_error() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("ci.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(fs)),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
        });

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }

    #[test]
    fn parse_error() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("ci.js");
        fs.insert(file_path.into(), PARSE_ERROR.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(fs)),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
        });

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }

    #[test]
    fn lint_error() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("ci.js");
        fs.insert(file_path.into(), LINT_ERROR.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(fs)),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
        });

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }
}

mod format {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn print() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("format"), file_path.as_os_str().into()]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, UNFORMATTED);
    }

    #[test]
    fn write() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let mut console = BufferConsole::default();
        let app = App::with_filesystem_and_console(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
        );

        let result = run_cli(CliSession {
            app,
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--write"),
                file_path.as_os_str().into(),
            ]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, FORMATTED);

        assert_eq!(console.buffer.len(), 1);
    }

    // Ensures lint warnings are not printed in format mode
    #[test]
    fn lint_warning() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), LINT_ERROR.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Borrowed(&mut console),
            ),
            args: Arguments::from_vec(vec![OsString::from("format"), file_path.as_os_str().into()]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, LINT_ERROR);

        // The console buffer is expected to contain the following message:
        // 0: "Formatter would have printed the following content"
        // 1: "Compared 1 files"
        assert_eq!(console.buffer.len(), 2, "console {:#?}", console.buffer);
    }

    #[test]
    fn indent_style_parse_errors() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--indent-style"),
                OsString::from("invalid"),
                OsString::from("file.js"),
            ]),
        });

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--indent-style"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }
    }

    #[test]
    fn indent_size_parse_errors_negative() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--indent-size"),
                OsString::from("-1"),
                OsString::from("file.js"),
            ]),
        });

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--indent-size"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }
    }

    #[test]
    fn indent_size_parse_errors_overflow() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--indent-size"),
                OsString::from("257"),
                OsString::from("file.js"),
            ]),
        });

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--indent-size"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }
    }

    #[test]
    fn line_width_parse_errors_negative() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--line-width"),
                OsString::from("-1"),
                OsString::from("file.js"),
            ]),
        });

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--line-width"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }
    }

    #[test]
    fn line_width_parse_errors_overflow() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--line-width"),
                OsString::from("321"),
                OsString::from("file.js"),
            ]),
        });

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--line-width"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }
    }

    #[test]
    fn format_with_configuration() {
        let mut fs = MemoryFileSystem::default();
        let config_path = current_dir().unwrap().join("rome.json");
        let file_path = Path::new(config_path.as_os_str());
        fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

        let file_path = Path::new("file.js");
        fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("file.js"),
                OsString::from("--write"),
            ]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, CUSTOM_FORMAT_AFTER);
    }

    #[test]
    fn format_is_disabled() {
        let mut fs = MemoryFileSystem::default();
        let config_path = current_dir().unwrap().join("rome.json");
        let file_path = Path::new(config_path.as_os_str());
        fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

        let file_path = Path::new("file.js");
        fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("file.js"),
                OsString::from("--write"),
            ]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, CUSTOM_FORMAT_BEFORE);
    }
}

mod help {
    use super::*;

    #[test]
    fn unknown_command() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("unknown"), OsString::from("--help")]),
        });

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

    #[test]
    fn unknown_command() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("unknown")]),
        });

        match result {
            Err(Termination::UnknownCommand { command }) => assert_eq!(command, "unknown"),
            _ => panic!("run_cli returned {result:?} for an unknown command, expected an error"),
        }
    }

    #[test]
    fn unexpected_argument() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--unknown"),
                OsString::from("file.js"),
            ]),
        });

        match result {
            Err(Termination::UnexpectedArgument { argument, .. }) => {
                assert_eq!(argument, OsString::from("--unknown"))
            }
            _ => panic!("run_cli returned {result:?} for an unknown argument, expected an error"),
        }
    }

    #[test]
    fn empty_arguments() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("format")]),
        });

        match result {
            Err(Termination::EmptyArguments) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }

    #[test]
    fn missing_argument() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("format"), OsString::from("--write")]),
        });

        match result {
            Err(Termination::MissingArgument { argument }) => assert_eq!(argument, "<INPUT>"),
            _ => panic!("run_cli returned {result:?} for a missing argument, expected an error"),
        }
    }
}
