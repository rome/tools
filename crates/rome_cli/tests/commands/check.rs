use pico_args::Arguments;
use std::env::temp_dir;
use std::ffi::OsString;
use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
use std::io::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink;
#[cfg(target_os = "windows")]
use std::os::windows::fs::{symlink_dir, symlink_file};
use std::path::{Path, PathBuf};

use crate::configs::{
    CONFIG_FILE_SIZE_LIMIT, CONFIG_IGNORE_SYMLINK, CONFIG_LINTER_AND_FILES_IGNORE,
    CONFIG_LINTER_DISABLED, CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC, CONFIG_LINTER_IGNORED_FILES,
    CONFIG_LINTER_SUPPRESSED_GROUP, CONFIG_LINTER_SUPPRESSED_RULE,
    CONFIG_LINTER_UPGRADE_DIAGNOSTIC, CONFIG_RECOMMENDED_GROUP,
};
use crate::snap_test::SnapshotPayload;
use crate::{assert_cli_snapshot, run_cli, FORMATTED, LINT_ERROR, PARSE_ERROR};
use rome_console::{BufferConsole, LogLevel, MarkupBuf};
use rome_fs::{ErrorEntry, FileSystemExt, MemoryFileSystem, OsFileSystem};
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
const NEW_SYMBOL: &str = "new Symbol(\"\");";

const FIX_BEFORE: &str = "
(1 >= -0)
";
const FIX_AFTER: &str = "
(1 >= 0)
";

const APPLY_SUGGESTED_BEFORE: &str = "let a = 4;
debugger;
console.log(a);
";

const APPLY_SUGGESTED_AFTER: &str = "const a = 4;\nconsole.log(a);\n";

const NO_DEBUGGER_BEFORE: &str = "debugger;";
const NO_DEBUGGER_AFTER: &str = "debugger;";

const UPGRADE_SEVERITY_CODE: &str = r#"if(!cond) { exprA(); } else { exprB() }"#;

const NURSERY_UNSTABLE: &str = r#"if(a = b) {}"#;

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");
}

#[test]
fn ok_read_only() {
    let mut fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
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
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    println!("{console:#?}");

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
                && content.contains("28")
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
        &mut console,
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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply"),
            file_path.as_os_str().into(),
        ]),
    );

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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply-unsafe"),
            OsString::from("--apply"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply-unsafe"),
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
fn apply_unsafe_with_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // last line doesn't have code fix
    let source = "let a = 4;
debugger;
console.log(a);
function f() { arguments; }
";

    let expected = "const a = 4;
console.log(a);
function f() { arguments; }
";

    let test1 = Path::new("test1.js");
    fs.insert(test1.into(), source.as_bytes());

    let test2 = Path::new("test2.js");
    fs.insert(test2.into(), source.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply-unsafe"),
            test1.as_os_str().into(),
            test2.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(test1)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, expected);
    drop(file);

    content.clear();

    let mut file = fs
        .open(test2)
        .expect("formatting target file was removed by the CLI");

    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "apply_unsafe_with_error",
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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
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
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let config_path = Path::new("rome.json");
    fs.insert(
        config_path.into(),
        CONFIG_LINTER_SUPPRESSED_GROUP.as_bytes(),
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
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
        &mut console,
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
                content.contains("suspicious/noDebugger")
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
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    let error_count = messages
        .iter()
        .filter(|m| m.level == LogLevel::Error)
        .filter(|m| {
            let content = format!("{:?}", m.content);
            content.contains("style/noNegationElse")
        })
        .count();

    assert_eq!(
        error_count, 1,
        "expected 1 error-level message in console buffer, found {error_count:?}:\n{:?}",
        console.out_buffer
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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply"),
            file_path_test1.as_os_str().into(),
            file_path_test2.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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

/// Creating a symbolic link will fail on Windows if the current process is
/// unprivileged. Since running tests as administrator is uncommon and
/// constraining, this error gets silently ignored if we're not running on CI
/// (the workflows are being being run with the correct permissions on CI)
#[cfg(target_os = "windows")]
macro_rules! check_windows_symlink {
    ($result:expr) => {
        match $result {
            Ok(res) => res,
            Err(err) if option_env!("CI") == Some("1") => panic!("failed to create symlink: {err}"),
            Err(_) => return,
        }
    };
}

#[test]
fn fs_error_dereferenced_symlink() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let root_path = temp_dir().join("rome_test_broken_symlink");
    let subdir_path = root_path.join("prefix");

    #[allow(unused_must_use)]
    {
        remove_dir_all(root_path.clone());
    }
    create_dir(root_path.clone()).unwrap();
    create_dir(subdir_path).unwrap();

    #[cfg(target_family = "unix")]
    {
        symlink(root_path.join("null"), root_path.join("broken_symlink")).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        check_windows_symlink!(symlink_file(
            root_path.join("null"),
            root_path.join("broken_symlink")
        ));
    }

    let result = run_cli(
        DynRef::Owned(Box::new(OsFileSystem)),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from(root_path.clone()),
        ]),
    );

    remove_dir_all(root_path).unwrap();

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_dereferenced_symlink",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_infinite_symlink_exapansion() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let root_path = temp_dir().join("rome_test_infinite_symlink_exapansion");
    let subdir1_path = root_path.join("prefix");
    let subdir2_path = root_path.join("foo").join("bar");

    #[allow(unused_must_use)]
    {
        remove_dir_all(root_path.clone());
    }
    create_dir(root_path.clone()).unwrap();
    create_dir(subdir1_path.clone()).unwrap();

    create_dir_all(subdir2_path.clone()).unwrap();

    #[cfg(target_family = "unix")]
    {
        symlink(subdir1_path.clone(), root_path.join("self_symlink1")).unwrap();
        symlink(subdir1_path, subdir2_path.join("self_symlink2")).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        check_windows_symlink!(symlink_dir(
            subdir1_path.clone(),
            root_path.join("self_symlink1")
        ));
        check_windows_symlink!(symlink_dir(
            subdir1_path,
            subdir2_path.join("self_symlink2")
        ));
    }

    let result = run_cli(
        DynRef::Owned(Box::new(OsFileSystem)),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from(root_path.clone()),
        ]),
    );

    remove_dir_all(root_path).unwrap();

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_infinite_symlink_expansion",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_read_only() {
    let mut fs = MemoryFileSystem::new_read_only();
    let mut console = BufferConsole::default();

    let file_path = Path::new("test.js");
    fs.insert(file_path.into(), *b"content");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    // Do not store the content of the file in the snapshot
    fs.remove(file_path);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_read_only",
        fs,
        console,
        result,
    ));
}

#[test]
fn fs_error_unknown() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert_error(PathBuf::from("prefix/ci.js"), ErrorEntry::UnknownFileType);

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), OsString::from("prefix")]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_error_unknown",
        fs,
        console,
        result,
    ));
}

// Symbolic link ignore pattern test
//
// Verifies, that ignore patterns to symbolic links are allowed.
//
// ├── rome.json
// ├── hidden_nested
// │   └── test
// │       └── symlink_testcase1_2 -> hidden_testcase1
// ├── hidden_testcase1
// │   └── test
// │       └── test.js // ok
// ├── hidden_testcase2
// │   ├── test1.ts // ignored
// │   ├── test2.ts // ignored
// │   └── test.js  // ok
// └── src
//     ├── symlink_testcase1_1 -> hidden_nested
//     └── symlink_testcase2 -> hidden_testcase2
#[test]
fn fs_files_ignore_symlink() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let root_path = temp_dir().join("rome_test_files_ignore_symlink");
    let src_path = root_path.join("src");

    let testcase1_path = root_path.join("hidden_testcase1");
    let testcase1_sub_path = testcase1_path.join("test");
    let testcase2_path = root_path.join("hidden_testcase2");

    let nested_path = root_path.join("hidden_nested");
    let nested_sub_path = nested_path.join("test");

    #[allow(unused_must_use)]
    {
        remove_dir_all(root_path.clone());
    }
    create_dir(root_path.clone()).unwrap();
    create_dir(src_path.clone()).unwrap();
    create_dir_all(testcase1_sub_path.clone()).unwrap();
    create_dir(testcase2_path.clone()).unwrap();
    create_dir_all(nested_sub_path.clone()).unwrap();

    // src/symlink_testcase1_1
    let symlink_testcase1_1_path = src_path.join("symlink_testcase1_1");
    // hidden_nested/test/symlink_testcase1_2
    let symlink_testcase1_2_path = nested_sub_path.join("symlink_testcase1_2");
    // src/symlink_testcase2
    let symlink_testcase2_path = src_path.join("symlink_testcase2");

    #[cfg(target_family = "unix")]
    {
        // src/test/symlink_testcase1_1 -> hidden_nested
        symlink(nested_path, symlink_testcase1_1_path).unwrap();
        // hidden_nested/test/symlink_testcase1_2 -> hidden_testcase1
        symlink(testcase1_path, symlink_testcase1_2_path).unwrap();
        // src/symlink_testcase2 -> hidden_testcase2
        symlink(testcase2_path.clone(), symlink_testcase2_path).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        check_windows_symlink!(symlink_dir(nested_path.clone(), symlink_testcase1_1_path));
        check_windows_symlink!(symlink_dir(
            testcase1_path.clone(),
            symlink_testcase1_2_path
        ));
        check_windows_symlink!(symlink_dir(testcase2_path.clone(), symlink_testcase2_path));
    }

    let config_path = root_path.join("rome.json");
    let mut config_file = File::create(config_path).unwrap();
    config_file
        .write_all(CONFIG_IGNORE_SYMLINK.as_bytes())
        .unwrap();

    let files: [PathBuf; 4] = [
        testcase1_sub_path.join("test.js"), // ok
        testcase2_path.join("test.js"),     // ok
        testcase2_path.join("test1.ts"),    // ignored
        testcase2_path.join("test2.ts"),    // ignored
    ];

    for file_path in files {
        let mut file = File::create(file_path).unwrap();
        file.write_all(APPLY_SUGGESTED_BEFORE.as_bytes()).unwrap();
    }

    let result = run_cli(
        DynRef::Owned(Box::new(OsFileSystem)),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--config-path"),
            OsString::from(root_path.clone()),
            OsString::from("--apply-unsafe"),
            OsString::from(src_path),
        ]),
    );

    remove_dir_all(root_path).unwrap();

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "fs_files_ignore_symlink",
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
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--files-max-size"),
            OsString::from("16"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

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
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--files-max-size"),
            OsString::from("-1"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "files_max_size_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_default() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    // Creates 40 diagnostics.
    for i in 0..40 {
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, LINT_ERROR.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), OsString::from("src")]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content.contains("useWhile") || node.content.contains("useBlockStatements")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics_default",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 20);
}

#[test]
fn max_diagnostics() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..20 {
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, LINT_ERROR.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--max-diagnostics"),
            OsString::from("10"),
            Path::new("src").as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content.contains("useWhile") || node.content.contains("useBlockStatements")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 10);
}

#[test]
fn no_supported_file_found() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![std::ffi::OsString::from("check"), ".".into()]),
    );

    eprintln!("{:?}", console.out_buffer);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_supported_file_found",
        fs,
        console,
        result,
    ));
}

#[test]
fn deprecated_suppression_comment() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(
        file_path.into(),
        *b"// rome-ignore lint(suspicious/noDoubleEquals): test
a == b;",
    );

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![
            std::ffi::OsString::from("check"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "deprecated_suppression_comment",
        fs,
        console,
        result,
    ));
}

#[test]
fn print_verbose() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--verbose"),
            file_path.as_os_str().into(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "print_verbose",
        fs,
        console,
        result,
    ));
}

#[test]
fn unsupported_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.txt");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "unsupported_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn suppression_syntax_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), *b"// rome-ignore(:\n");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppression_syntax_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn config_recommended_group() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_RECOMMENDED_GROUP.as_bytes());

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), NEW_SYMBOL.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "config_recommended_group",
        fs,
        console,
        result,
    ));
}

#[test]
fn nursery_unstable() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("check.js");
    fs.insert(file_path.into(), NURSERY_UNSTABLE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nursery_unstable",
        fs,
        console,
        result,
    ));
}

#[test]
fn organize_imports() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{ "organizeImports": { "enabled": true } }"#;

    let config_path = Path::new("rome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let file_path = Path::new("check.js");
    let content = r#"import { lorem, foom, bar } from "foo";
import * as something from "../something";
"#;
    let expected = r#"import * as something from "../something";
import { bar, foom, lorem } from "foo";
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("check"),
            OsString::from("--apply-unsafe"),
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

    assert_eq!(content, expected);

    drop(file);

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "organize_imports",
        fs,
        console,
        result,
    ));
}

#[test]
fn all_rules() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "linter": {
            "rules": { "all": true }
        }
    }"#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), FIX_BEFORE.as_bytes());

    let config_path = Path::new("rome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "all_rules",
        fs,
        console,
        result,
    ));
}

#[test]
fn top_level_all_down_level_not_all() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "linter": {
            "rules": {
                "all": true,
                "style": {
                    "all": false
                }
            }
        }
    }"#;

    // style/noArguments
    // style/noShoutyConstants
    // style/useSingleVarDeclarator
    let code = r#"
    function f() {arguments;}
    const FOO = "FOO";
    var x, y;
    "#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), code.as_bytes());

    let config_path = Path::new("rome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "top_level_all_down_level_not_all",
        fs,
        console,
        result,
    ));
}

#[test]
fn top_level_not_all_down_level_all() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "linter": {
            "rules": {
                "all": false,
                "style": {
                    "all": true
                }
            }
        }
    }"#;

    // style/noArguments
    // style/noShoutyConstants
    // style/useSingleVarDeclarator
    let code = r#"
    function f() {arguments;}
    const FOO = "FOO";
    var x, y;
    "#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), code.as_bytes());

    let config_path = Path::new("rome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "top_level_not_all_down_level_all",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_configured_globals() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "javascript": {
            "globals": ["foo", "bar"]
        }
    }"#;

    // style/useSingleVarDeclarator
    let code = r#"foo.call(); bar.call();"#;

    let file_path = Path::new("fix.js");
    fs.insert(file_path.into(), code.as_bytes());

    let config_path = Path::new("rome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![OsString::from("check"), file_path.as_os_str().into()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_configured_globals",
        fs,
        console,
        result,
    ));
}
