use pico_args::Arguments;
use rome_cli::Termination;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::configs::{
    CONFIG_FILE_SIZE_LIMIT, CONFIG_LINTER_AND_FILES_IGNORE, CONFIG_LINTER_DISABLED,
    CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC, CONFIG_LINTER_IGNORED_FILES,
    CONFIG_LINTER_SUPPRESSED_GROUP, CONFIG_LINTER_SUPPRESSED_RULE,
    CONFIG_LINTER_UPGRADE_DIAGNOSTIC,
};
use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli, FORMATTED, LINT_ERROR, PARSE_ERROR};
use rome_console::{BufferConsole, LogLevel};
use rome_fs::{ErrorEntry, FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;

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

const UPGRADE_SEVERITY_CODE: &str = r#"class A extends B {
    constructor() {}
}"#;

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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "parse_error",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "lint_error",
        fs,
        console,
        result,
    ));
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

    assert!(result.is_err(), "run_cli returned {result:?}");

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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "maximum_diagnostics",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_ok",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_noop",
        fs,
        console,
        result,
    ));
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

    match &result {
        Err(error) => {
            assert!(error
                .to_string()
                .contains("incompatible arguments '--apply' and '--apply-suggested"),)
        }
        _ => panic!("expected an error, but found none"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_suggested_error",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_suggested",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_if_linter_is_disabled_when_run_apply",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_if_linter_is_disabled",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_disable_a_rule",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_disable_a_rule_group",
        fs,
        console,
        result,
    ));
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

    println!("{console:?}");

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    assert_eq!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Error)
            .filter(|m| {
                let content = format!("{:#?}", m.content);
                content.contains("correctness/noDebugger")
            })
            .count(),
        1
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "downgrade_severity",
        fs,
        console,
        result,
    ));
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
    fs.insert(file_path.into(), UPGRADE_SEVERITY_CODE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    dbg!(&result);
    assert_eq!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Error)
            .filter(|m| {
                let content = format!("{:?}", m.content);
                content.contains("nursery/noInvalidConstructorSuper")
            })
            .count(),
        1
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "upgrade_severity",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_lint_when_file_is_ignored() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_LINTER_IGNORED_FILES.as_bytes());

    let file_path = Path::new("test.js");
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

    assert_eq!(buffer, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_when_file_is_ignored",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_lint_if_files_are_listed_in_ignore_option() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_LINTER_AND_FILES_IGNORE.as_bytes());

    let file_path_test1 = Path::new("test1.js");
    fs.insert(file_path_test1.into(), FIX_BEFORE.as_bytes());

    let file_path_test2 = Path::new("test2.js");
    fs.insert(file_path_test2.into(), FIX_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply"),
            file_path_test1.as_os_str().into(),
            file_path_test2.as_os_str().into(),
        ]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut buffer = String::new();
    fs.open(file_path_test1)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    let mut buffer = String::new();
    fs.open(file_path_test2)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    assert_eq!(buffer, FIX_BEFORE);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_lint_if_files_are_listed_in_ignore_option",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_symlink() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert_error(PathBuf::from("prefix/ci.js"), ErrorEntry::SymbolicLink);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("check"), OsString::from("prefix")]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_symlink",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_unknown() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert_error(PathBuf::from("prefix/ci.js"), ErrorEntry::Unknown);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("check"), OsString::from("prefix")]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_unknown",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
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

#[test]
fn file_too_large_config_limit() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(PathBuf::from("rome.json"), CONFIG_FILE_SIZE_LIMIT);

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_config_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large_cli_limit() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--files-max-size"),
            OsString::from("16"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_too_large_cli_limit",
        fs,
        console,
        result,
    ));
}

#[test]
fn files_max_size_parse_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--files-max-size"),
            OsString::from("-1"),
            file_path.as_os_str().into(),
        ]),
    );

    match result {
        Err(Termination::ParseError { argument, .. }) => assert_eq!(argument, "--files-max-size"),
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "files_max_size_parse_error",
        fs,
        console,
        result,
    ));
}
