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

const NO_DEBUGGER: &str = "debugger;";

const FIX_BEFORE: &str = "
if(a != -0) {}
";
const FIX_AFTER: &str = "
if(a != 0) {}
";

const APPLY_SUGGESTED_BEFORE: &str = "let a = 4;
debugger;
console.log(a);
";

const APPLY_SUGGESTED_AFTER: &str = "let a = 4;\nconsole.log(a);\n";

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
const NO_DEBUGGER_AFTER: &str = "debugger;";

const JS_ERRORS_BEFORE: &str = r#"try {
    !a && !b;
} catch (err) {
    err = 24;
}
"#;
const JS_ERRORS_AFTER: &str = "try {
    !a && !b;
} catch (err) {
    err = 24;
}
";

const NO_DEAD_CODE_ERROR: &str = r#"function f() {
    for (;;) {
        continue;
        break;
    }
}
"#;

mod check {
    use super::*;
    use crate::configs::{
        CONFIG_LINTER_DISABLED, CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC, CONFIG_LINTER_SUPPRESSED_GROUP,
        CONFIG_LINTER_SUPPRESSED_RULE, CONFIG_LINTER_UPGRADE_DIAGNOSTIC,
    };
    use rome_console::LogLevel;
    use rome_fs::FileSystemExt;

    #[test]
    fn ok() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), FORMATTED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");
    }

    #[test]
    fn parse_error() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), PARSE_ERROR.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        );

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }

        assert_cli_snapshot(module_path!(), "parse_error", fs, console);
    }

    #[test]
    fn lint_error() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), LINT_ERROR.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        );

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }

        assert_cli_snapshot(module_path!(), "lint_error", fs, console);
    }

    #[test]
    fn maximum_diagnostics() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Path::new("check.js");
        fs.insert(file_path.into(), ERRORS.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        );

        assert!(result.is_err());

        let messages = &console.out_buffer;

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

        assert_cli_snapshot(module_path!(), "maximum_diagnostics", fs, console);
    }

    #[test]
    fn apply_ok() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, FIX_AFTER);

        assert_cli_snapshot(module_path!(), "apply_ok", fs, console);
    }

    #[test]
    fn apply_noop() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_AFTER.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        );

        println!("{console:#?}");

        assert!(result.is_ok(), "run_cli returned {result:?}");

        assert_cli_snapshot(module_path!(), "apply_noop", fs, console);
    }

    #[test]
    fn apply_suggested_error() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), APPLY_SUGGESTED_BEFORE.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply-suggested"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");

        match result {
            Err(error) => {
                assert!(error
                    .to_string()
                    .contains("incompatible arguments '--apply' and '--apply-suggested"),)
            }
            _ => panic!("expected an error, but found none"),
        }

        assert_cli_snapshot(module_path!(), "apply_suggested_error", fs, console);
    }

    #[test]
    fn apply_suggested() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), APPLY_SUGGESTED_BEFORE.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply-suggested"),
                file_path.as_os_str().into(),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, APPLY_SUGGESTED_AFTER);

        assert_cli_snapshot(module_path!(), "apply_suggested", fs, console);
    }

    #[test]
    fn no_lint_if_linter_is_disabled_when_run_apply() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

        let config_path = Path::new("rome.json");
        fs.insert(config_path.into(), CONFIG_LINTER_DISABLED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, FIX_BEFORE);

        assert_cli_snapshot(
            module_path!(),
            "no_lint_if_linter_is_disabled_when_run_apply",
            fs,
            console,
        );
    }

    #[test]
    fn no_lint_if_linter_is_disabled() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

        let config_path = Path::new("rome.json");
        fs.insert(config_path.into(), CONFIG_LINTER_DISABLED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, FIX_BEFORE);

        assert_cli_snapshot(module_path!(), "no_lint_if_linter_is_disabled", fs, console);
    }

    #[test]
    fn should_disable_a_rule() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("fix.js");
        fs.insert(file_path.into(), NO_DEBUGGER_BEFORE.as_bytes());

        let config_path = Path::new("rome.json");
        fs.insert(config_path.into(), CONFIG_LINTER_SUPPRESSED_RULE.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, NO_DEBUGGER_AFTER);

        assert_cli_snapshot(module_path!(), "should_disable_a_rule", fs, console);
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

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("check"),
                OsString::from("--apply"),
                file_path.as_os_str().into(),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut buffer = String::new();
        fs.open(file_path)
            .unwrap()
            .read_to_string(&mut buffer)
            .unwrap();

        assert_eq!(buffer, JS_ERRORS_AFTER);

        assert_cli_snapshot(module_path!(), "should_disable_a_rule_group", fs, console);
    }

    #[test]
    fn downgrade_severity() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Path::new("rome.json");
        fs.insert(
            file_path.into(),
            CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC.as_bytes(),
        );

        let file_path = Path::new("file.js");
        fs.insert(file_path.into(), NO_DEBUGGER.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let messages = &console.out_buffer;

        assert_eq!(
            messages
                .iter()
                .filter(|m| m.level == LogLevel::Error)
                .filter(|m| {
                    let content = format!("{:#?}", m.content);
                    content.contains("js/noDebugger")
                })
                .count(),
            1
        );

        assert_cli_snapshot(module_path!(), "downgrade_severity", fs, console);
    }

    #[test]
    fn upgrade_severity() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Path::new("rome.json");
        fs.insert(
            file_path.into(),
            CONFIG_LINTER_UPGRADE_DIAGNOSTIC.as_bytes(),
        );

        let file_path = Path::new("file.js");
        fs.insert(file_path.into(), NO_DEAD_CODE_ERROR.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");

        let messages = &console.out_buffer;

        assert_eq!(
            messages
                .iter()
                .filter(|m| m.level == LogLevel::Error)
                .filter(|m| {
                    let content = format!("{:?}", m.content);
                    content.contains("js/noDeadCode")
                })
                .count(),
            1
        );

        assert_cli_snapshot(module_path!(), "upgrade_severity", fs, console);
    }
}

mod ci {
    use super::*;
    use rome_fs::FileSystemExt;

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

        assert_cli_snapshot(module_path!(), "ci_ok", fs, console);
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

        assert_cli_snapshot(module_path!(), "formatting_error", fs, console);
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

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }

        assert_cli_snapshot(module_path!(), "ci_parse_error", fs, console);
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

        match result {
            Err(Termination::CheckError) => {}
            _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
        }

        assert_cli_snapshot(module_path!(), "ci_lint_error", fs, console);
    }
}

mod format {
    use super::*;
    use crate::configs::{CONFIG_DISABLED_FORMATTER, CONFIG_FORMAT};
    use crate::snap_test::markup_to_string;
    use rome_console::markup;
    use rome_fs::FileSystemExt;

    #[test]
    fn print() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("format"), file_path.as_os_str().into()]),
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
        assert_cli_snapshot(module_path!(), "formatter_print", fs, console);
    }

    #[test]
    fn write() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), UNFORMATTED.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--write"),
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
        assert_cli_snapshot(module_path!(), "formatter_write", fs, console);
    }

    // Ensures lint warnings are not printed in format mode
    #[test]
    fn lint_warning() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("format.js");
        fs.insert(file_path.into(), LINT_ERROR.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![OsString::from("format"), file_path.as_os_str().into()]),
        );

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
        assert_eq!(
            console.out_buffer.len(),
            2,
            "console {:#?}",
            console.out_buffer
        );

        drop(file);
        assert_cli_snapshot(module_path!(), "formatter_lint_warning", fs, console);
    }

    #[test]
    fn indent_style_parse_errors() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--indent-style"),
                OsString::from("invalid"),
                OsString::from("file.js"),
            ]),
        );

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--indent-style"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }

        assert_cli_snapshot(module_path!(), "indent_style_parse_errors", fs, console);
    }

    #[test]
    fn indent_size_parse_errors_negative() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--indent-size"),
                OsString::from("-1"),
                OsString::from("file.js"),
            ]),
        );

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--indent-size"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }

        assert_cli_snapshot(
            module_path!(),
            "indent_size_parse_errors_negative",
            fs,
            console,
        );
    }

    #[test]
    fn indent_size_parse_errors_overflow() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--indent-size"),
                OsString::from("257"),
                OsString::from("file.js"),
            ]),
        );

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--indent-size"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }

        assert_cli_snapshot(
            module_path!(),
            "indent_size_parse_errors_overflow",
            fs,
            console,
        );
    }

    #[test]
    fn line_width_parse_errors_negative() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--line-width"),
                OsString::from("-1"),
                OsString::from("file.js"),
            ]),
        );

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--line-width"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }

        assert_cli_snapshot(
            module_path!(),
            "line_width_parse_errors_negative",
            fs,
            console,
        );
    }

    #[test]
    fn line_width_parse_errors_overflow() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--line-width"),
                OsString::from("321"),
                OsString::from("file.js"),
            ]),
        );

        match result {
            Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--line-width"),
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }

        assert_cli_snapshot(
            module_path!(),
            "line_width_parse_errors_overflow",
            fs,
            console,
        );
    }

    #[test]
    fn quote_properties_parse_errors_letter_case() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--quote-properties"),
                OsString::from("As-needed"),
                OsString::from("file.js"),
            ]),
        );

        match result {
            Err(Termination::ParseError { argument, .. }) => {
                assert_eq!(argument, "--quote-properties")
            }
            _ => panic!(
                "run_cli returned {result:?} for an invalid argument value, expected an error"
            ),
        }

        assert_cli_snapshot(
            module_path!(),
            "quote_properties_parse_errors_letter_case",
            fs,
            console,
        );
    }

    #[test]
    fn format_with_configuration() {
        let mut console = BufferConsole::default();
        let mut fs = MemoryFileSystem::default();
        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

        let file_path = Path::new("file.js");
        fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("file.js"),
                OsString::from("--write"),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, CUSTOM_FORMAT_AFTER);

        drop(file);
        assert_cli_snapshot(module_path!(), "format_with_configuration", fs, console);
    }

    #[test]
    fn format_is_disabled() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();
        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

        let file_path = Path::new("file.js");
        fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("file.js"),
                OsString::from("--write"),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let mut file = fs
            .open(file_path)
            .expect("formatting target file was removed by the CLI");

        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file from memory FS");

        assert_eq!(content, CUSTOM_FORMAT_BEFORE);

        drop(file);
        assert_cli_snapshot(module_path!(), "format_is_disabled", fs, console);
    }

    #[test]
    fn format_stdin_successfully() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        console
            .in_buffer
            .push("function f() {return{}}".to_string());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--stdin-file-path"),
                OsString::from("mock.js"),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let message = console
            .out_buffer
            .get(0)
            .expect("Console should have written a message");

        let content = markup_to_string(markup! {
            {message.content}
        });

        assert_eq!(content, "function f() {\n\treturn {};\n}\n");

        assert_cli_snapshot(module_path!(), "format_stdin_successfully", fs, console);
    }

    #[test]
    fn format_stdin_with_errors() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--stdin-file-path"),
                OsString::from("mock.js"),
            ]),
        );

        assert!(result.is_err(), "run_cli returned {result:?}");

        match result {
            Err(Termination::MissingArgument { argument }) => assert_eq!(argument, "stdin"),
            _ => {
                panic!("run_cli returned {result:?} for an unknown command help, expected an error")
            }
        }

        assert_cli_snapshot(module_path!(), "format_stdin_with_errors", fs, console);
    }

    #[test]
    fn does_not_format_if_disabled() {
        let mut fs = MemoryFileSystem::default();
        let mut console = BufferConsole::default();

        let file_path = Path::new("rome.json");
        fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

        console
            .in_buffer
            .push("function f() {return{}}".to_string());

        let result = run_cli(
            DynRef::Borrowed(&mut fs),
            DynRef::Borrowed(&mut console),
            Arguments::from_vec(vec![
                OsString::from("format"),
                OsString::from("--stdin-file-path"),
                OsString::from("mock.js"),
            ]),
        );

        assert!(result.is_ok(), "run_cli returned {result:?}");

        let message = console
            .out_buffer
            .get(0)
            .expect("Console should have written a message");

        let content = markup_to_string(markup! {
            {message.content}
        });

        assert_eq!(content, "function f() {return{}}".to_string());

        assert_cli_snapshot(module_path!(), "does_not_format_if_disabled", fs, console);
    }
}

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

        assert_cli_snapshot(module_path!(), "creates_config_file", fs, console);
    }
}

mod configuration {
    use super::*;
    use crate::configs::{
        CONFIG_ALL_FIELDS, CONFIG_BAD_LINE_WIDTH, CONFIG_INCORRECT_GLOBALS,
        CONFIG_INCORRECT_GLOBALS_V2, CONFIG_LINTER_WRONG_RULE,
    };
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

        assert_cli_snapshot(module_path!(), "correct_root", fs, console);
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

        assert_cli_snapshot(module_path!(), "line_width_error", fs, console);
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

        assert_cli_snapshot(module_path!(), "incorrect_rule_name", fs, console);
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

        assert_cli_snapshot(module_path!(), "incorrect_globals", fs, console);
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
        assert_cli_snapshot(module_path!(), "reports_formatter_check_mode", fs, console);
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

        assert_cli_snapshot(module_path!(), "reports_formatter_write", fs, console);
    }
}

/// Create an [App] instance using the provided [FileSystem] and [Console]
/// instance, and using an in-process "remote" instance of the workspace
fn run_cli<'app>(
    fs: DynRef<'app, dyn FileSystem>,
    console: DynRef<'app, dyn Console>,
    args: Arguments,
) -> Result<(), Termination> {
    use rome_bin::SocketTransport;
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

    let transport = SocketTransport::open(runtime, client);

    let workspace = workspace::client(transport).unwrap();
    let app = App::new(fs, console, WorkspaceRef::Owned(workspace));

    let session = CliSession { app, args };
    session.run()
}
