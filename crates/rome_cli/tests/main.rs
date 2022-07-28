mod configs;

use std::{ffi::OsString, path::Path};

use pico_args::Arguments;
use rome_cli::{run_cli, CliSession, Termination};
use rome_console::BufferConsole;
use rome_fs::MemoryFileSystem;
use rome_service::{App, DynRef};

const UNFORMATTED: &str = "  statement(  )  ";
const FORMATTED: &str = "statement();\n";

const PARSE_ERROR: &str = "if\n";
const LINT_ERROR: &str = "for(;true;);\n";

const ERRORS: &str = r#"
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);for(;true;);
"#;

const FIX_BEFORE: &str = "
if(a != -0) {}
";
const FIX_AFTER: &str = "if(a != 0) {}
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

const NO_DEBUGGER_BEFORE: &str = "debugger;";
const NO_DEBUGGER_AFTER: &str = "debugger;\n";

const JS_ERRORS_BEFORE: &str = r#"try {
    !a && !b;
} catch (err) {
    err = 24;
}
"#;
const JS_ERRORS_AFTER: &str = "try {\
    \n\t!a && !b;
} catch (err) {\
    \n\terr = 24;
}
";

mod check {
    use super::*;
    use crate::configs::{
        CONFIG_LINTER_DISABLED, CONFIG_LINTER_SUPPRESSED_GROUP, CONFIG_LINTER_SUPPRESSED_RULE,
    };
    use rome_console::LogLevel;
    use rome_fs::FileSystemExt;

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
    fn maximum_diagnostics() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), ERRORS.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Borrowed(&mut console),
            ),
            args: Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        });

        eprintln!("{:?}", console.buffer);

        // TODO lint errors are disabled until the linter is stable
        assert!(result.is_ok());

        let messages = console.buffer;

        assert_eq!(
            messages
                .iter()
                .filter(|m| m.level == LogLevel::Error)
                .count(),
            20_usize
        );

        assert!(messages
            .iter()
            .filter(|m| m.level == LogLevel::Log)
            .any(|m| {
                let content = format!("{:?}", m.content);
                content.contains("The number of diagnostics exceeds the number allowed by Rome")
                    && content.contains("Diagnostics not shown")
                    && content.contains("76")
            }));
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

    #[test]
    fn no_lint_if_linter_is_disabled_when_run_apply() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

        let config_path = Path::new("rome.json");
        fs.insert(config_path.into(), CONFIG_LINTER_DISABLED.as_bytes());

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

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, FIX_BEFORE);
    }

    #[test]
    fn no_lint_if_linter_is_disabled() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

        let config_path = Path::new("rome.json");
        fs.insert(config_path.into(), CONFIG_LINTER_DISABLED.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Borrowed(&mut console),
            ),
            args: Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        });

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, FIX_BEFORE);
    }

    #[test]
    fn should_disable_a_rule() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), NO_DEBUGGER_BEFORE.as_bytes());

        let config_path = Path::new("rome.json");
        fs.insert(config_path.into(), CONFIG_LINTER_SUPPRESSED_RULE.as_bytes());

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

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, NO_DEBUGGER_AFTER);
    }

    #[test]
    fn should_disable_a_rule_group() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), JS_ERRORS_BEFORE.as_bytes());

        let config_path = Path::new("rome.json");
        fs.insert(
            config_path.into(),
            CONFIG_LINTER_SUPPRESSED_GROUP.as_bytes(),
        );

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

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, JS_ERRORS_AFTER);
    }
}

mod ci {
    use super::*;
    use rome_fs::FileSystemExt;

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

        let mut console = BufferConsole::default();
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(fs)),
                DynRef::Borrowed(&mut console),
            ),
            args: Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
        });

        eprintln!("{:?}", console.buffer);

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }
    }
}

mod format {
    use super::*;
    use crate::configs::{CONFIG_DISABLED_FORMATTER, CONFIG_FORMAT};
    use rome_fs::FileSystemExt;

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

        eprintln!("{:?}", console.buffer);

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
        let file_path = Path::new("rome.json");
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
        let file_path = Path::new("rome.json");
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
    use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;

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

    #[test]
    fn incorrect_value() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--max-diagnostics=foo"),
            ]),
        });

        match result {
            Err(Termination::ParseError { argument, .. }) => {
                assert_eq!(argument, "--max-diagnostics");
            }
            _ => panic!("run_cli returned {result:?} for a malformed, expected an error"),
        }
    }

    #[test]
    fn overflow_value() {
        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Owned(Box::new(MemoryFileSystem::default())),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--max-diagnostics=500"),
            ]),
        });

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
    use crate::configs::CONFIG_INIT_DEFAULT;
    use pico_args::Arguments;
    use rome_cli::{run_cli, CliSession};
    use rome_console::BufferConsole;
    use rome_fs::{FileSystemExt, MemoryFileSystem};
    use rome_service::{App, DynRef};
    use std::ffi::OsString;
    use std::path::Path;

    #[test]
    fn creates_config_file() {
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("init")]),
        });
        assert!(result.is_ok(), "run_cli returned {result:?}");

        let file_path = Path::new("rome.json");

        let mut file = fs
            .open(file_path)
            .expect("configuration file was not written on disk");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");
        assert_eq!(content, CONFIG_INIT_DEFAULT);
    }
}

mod configuration {
    use crate::configs::{
        CONFIG_ALL_FIELDS, CONFIG_BAD_LINE_WIDTH, CONFIG_INCORRECT_GLOBALS,
        CONFIG_LINTER_WRONG_RULE,
    };
    use pico_args::Arguments;
    use rome_cli::{run_cli, CliSession};
    use rome_console::BufferConsole;
    use rome_fs::MemoryFileSystem;
    use rome_service::{App, DynRef};
    use std::ffi::OsString;
    use std::path::Path;

    #[test]
    fn correct_root() {
        let mut fs = MemoryFileSystem::default();
        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_ALL_FIELDS.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("format"), OsString::from("file.js")]),
        });

        assert!(result.is_ok());
    }

    #[test]
    fn line_width_error() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_BAD_LINE_WIDTH.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("format"), OsString::from("file.js")]),
        });
        assert!(result.is_err());

        match result {
            Err(error) => {
                assert!(error
                    .to_string()
                    .contains("The line width exceeds the maximum value (320)"),)
            }
            _ => panic!("expected an error, but found none"),
        }
    }

    #[test]
    fn incorrect_rule_name() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_LINTER_WRONG_RULE.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("check"), OsString::from("file.js")]),
        });

        match result {
            Err(error) => {
                assert!(error.to_string().contains("Invalid rule name `foo_rule`"),)
            }
            _ => panic!("expected an error, but found none"),
        }
    }

    #[test]
    fn incorrect_globals() {
        let mut fs = MemoryFileSystem::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_INCORRECT_GLOBALS.as_bytes());

        let result = run_cli(CliSession {
            app: App::with_filesystem_and_console(
                DynRef::Borrowed(&mut fs),
                DynRef::Owned(Box::new(BufferConsole::default())),
            ),
            args: Arguments::from_vec(vec![OsString::from("check"), OsString::from("file.js")]),
        });

        assert!(result.is_err());

        match result {
            Err(error) => {
                assert!(error
                    .to_string()
                    .contains("invalid type: boolean `false`, expected a string"),)
            }
            _ => panic!("expected an error, but found none"),
        }
    }
}
