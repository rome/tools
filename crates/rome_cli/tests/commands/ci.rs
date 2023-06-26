use crate::configs::{
    CONFIG_DISABLED_FORMATTER, CONFIG_FILE_SIZE_LIMIT, CONFIG_LINTER_DISABLED,
    CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC,
};
use crate::snap_test::SnapshotPayload;
use crate::{
    assert_cli_snapshot, run_cli, CUSTOM_FORMAT_BEFORE, FORMATTED, LINT_ERROR, PARSE_ERROR,
    UNFORMATTED,
};
use bpaf::Args;
use rome_console::{BufferConsole, LogLevel, MarkupBuf};
use rome_fs::{FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;
use std::path::{Path, PathBuf};

const INCORRECT_CODE: &str = "let a = !b || !c";

const UNFORMATTED_AND_INCORRECT: &str = "statement(    ) ; let a = !b || !c;";

const CI_CONFIGURATION: &str = r#"
{
    "formatter": {
        "enabled": true
    },
    "linter": {
        "enabled": true,
        "rules": {
            "recommended": true
        }
    }
}
"#;

#[test]
fn ci_help() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), "--help"]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_help",
        fs,
        console,
        result,
    ));
}

#[test]
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_ok",
        fs,
        console,
        result,
    ));
}

#[test]
fn formatting_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatting_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_parse_error() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), PARSE_ERROR.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_parse_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_lint_error() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let mut console = BufferConsole::default();
    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_lint_error",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_formatter() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        PathBuf::from("rome.json"),
        CONFIG_DISABLED_FORMATTER.as_bytes(),
    );

    let input_file = Path::new("file.js");

    fs.insert(input_file.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), input_file.as_os_str().to_str().unwrap()]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(input_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_formatter",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_formatter_via_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let input_file = Path::new("file.js");
    fs.insert(input_file.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--formatter-enabled=false"),
            input_file.as_os_str().to_str().unwrap(),
        ]),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(input_file)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_formatter_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_linter() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    fs.insert(
        PathBuf::from("rome.json"),
        CONFIG_LINTER_DISABLED.as_bytes(),
    );

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_FORMAT_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, CUSTOM_FORMAT_BEFORE);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_linter",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_run_linter_via_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), UNFORMATTED_AND_INCORRECT.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--linter-enabled=false"),
            file_path.as_os_str().to_str().unwrap(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED_AND_INCORRECT);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_run_linter_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_does_not_organize_imports_via_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");

    let content = r#"import { lorem, foom, bar } from "foo";
import * as something from "../something";
"#;
    fs.insert(file_path.into(), content.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--organize-imports-enabled=false"),
            file_path.as_os_str().to_str().unwrap(),
        ]),
    );

    // assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut received = String::new();
    file.read_to_string(&mut received)
        .expect("failed to read file from memory FS");

    assert_eq!(received, content);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_does_not_organize_imports_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_errors_for_all_disabled_checks() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CI_CONFIGURATION.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), UNFORMATTED_AND_INCORRECT.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--linter-enabled=false"),
            ("--formatter-enabled=false"),
            ("--organize-imports-enabled=false"),
            file_path.as_os_str().to_str().unwrap(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, UNFORMATTED_AND_INCORRECT);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_errors_for_all_disabled_checks",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
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

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
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

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--files-max-size=16"),
            file_path.as_os_str().to_str().unwrap(),
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

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--files-max-size=-1"),
            file_path.as_os_str().to_str().unwrap(),
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
fn ci_runs_linter_not_formatter_issue_3495() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_DISABLED_FORMATTER.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), INCORRECT_CODE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("ci target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_runs_linter_not_formatter_issue_3495",
        fs,
        console,
        result,
    ));
}

#[test]
fn max_diagnostics_default() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..60 {
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), ("src")]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content
                .contains("File content differs from formatting output")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    for i in 0..60 {
        let file_path = format!("src/file_{i}.js");
        fs.remove(Path::new(&file_path));
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "max_diagnostics_default",
        fs,
        console,
        result,
    ));

    assert_eq!(diagnostic_count, 50);
}

#[test]
fn max_diagnostics() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    for i in 0..60 {
        let file_path = PathBuf::from(format!("src/file_{i}.js"));
        fs.insert(file_path, UNFORMATTED.as_bytes());
    }

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), ("--max-diagnostics"), ("10"), ("src")]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut diagnostic_count = 0;
    let mut filtered_messages = Vec::new();

    for msg in console.out_buffer {
        let MarkupBuf(nodes) = &msg.content;
        let is_diagnostic = nodes.iter().any(|node| {
            node.content
                .contains("File content differs from formatting output")
        });

        if is_diagnostic {
            diagnostic_count += 1;
        } else {
            filtered_messages.push(msg);
        }
    }

    console.out_buffer = filtered_messages;

    for i in 0..60 {
        let file_path = format!("src/file_{i}.js");
        fs.remove(Path::new(&file_path));
    }

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
fn print_verbose() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), LINT_ERROR.as_bytes());

    let mut console = BufferConsole::default();
    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--verbose"),
            file_path.as_os_str().to_str().unwrap(),
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
fn suppress_warnings() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_path = Path::new("rome.json");
    fs.insert(
        rome_path.into(),
        CONFIG_LINTER_DOWNGRADE_DIAGNOSTIC.as_bytes(),
    );

    let file_path = Path::new("file.ts");

    const DEBUG_AND_ANY: &str = "debugger; const a: any = 1;";

    fs.insert(file_path.into(), DEBUG_AND_ANY.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--suppress-warnings"),
            file_path.as_os_str().to_str().unwrap(),
        ]),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    let messages = &console.out_buffer;

    assert_eq!(
        messages
            .iter()
            .filter(|m| m.level == LogLevel::Error)
            .filter(|m| {
                let content = format!("{:#?}", m.content);
                content.contains("suspicious/noExplicitAny")
            })
            .count(),
        1
    );

    assert_eq!(
        messages
            .iter()
            .filter(|m| {
                let content = format!("{:#?}", m.content);
                content.contains("suspicious/noDebugger")
            })
            .count(),
        0
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "suppress_warnings",
        fs,
        console,
        result,
    ));
}

#[test]
fn ci_formatter_linter_organize_imports() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
    "linter": {
        "enabled": true,
        "rules": {
            "recommended": true
        }
    },
    "organizeImports": {
        "enabled": true
    }
}"#;

    let input = r#"
import { B, C } from "b.js"
import A from "a.js"


something( )
    "#;

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), rome_json.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), input.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[("ci"), file_path.as_os_str().to_str().unwrap()]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("ci target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ci_formatter_linter_organize_imports",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_ignored_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let rome_json = r#"{
        "vcs": {
            "enabled": true,
            "clientKind": "git",
            "useIgnoreFile": true
        }
    }"#;

    let git_ignore = r#"
file2.js
"#;

    let code2 = r#"foo.call(); bar.call();"#;
    let code1 = r#"array.map(sentence => sentence.split(' ')).flat();"#;

    // ignored files
    let file_path1 = Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // configuration
    let config_path = Path::new("rome.json");
    fs.insert(config_path.into(), rome_json.as_bytes());

    // git folder
    let git_folder = Path::new(".git");
    fs.insert(git_folder.into(), "".as_bytes());

    // git ignore file
    let ignore_file = Path::new(".gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            file_path1.as_os_str().to_str().unwrap(),
            file_path2.as_os_str().to_str().unwrap(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_ignored_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignore_vcs_ignored_file_via_cli() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let git_ignore = r#"
file2.js
"#;

    let code2 = r#"foo.call();


	bar.call();"#;
    let code1 = r#"array.map(sentence => sentence.split(' ')).flat();"#;

    // ignored files
    let file_path1 = Path::new("file1.js");
    fs.insert(file_path1.into(), code1.as_bytes());
    let file_path2 = Path::new("file2.js");
    fs.insert(file_path2.into(), code2.as_bytes());

    // git folder
    let git_folder = Path::new("./.git");
    fs.insert(git_folder.into(), "".as_bytes());

    // git ignore file
    let ignore_file = Path::new("./.gitignore");
    fs.insert(ignore_file.into(), git_ignore.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            ("--vcs-enabled=true"),
            ("--vcs-client-kind=git"),
            ("--vcs-use-ignore-file=true"),
            ("--vcs-root=."),
            file_path1.as_os_str().to_str().unwrap(),
            file_path2.as_os_str().to_str().unwrap(),
        ]),
    );

    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignore_vcs_ignored_file_via_cli",
        fs,
        console,
        result,
    ));
}

#[test]
fn ignores_unknown_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path1 = Path::new("test.txt");
    fs.insert(file_path1.into(), *b"content");

    let file_path2 = Path::new("test.js");
    fs.insert(file_path2.into(), *b"console.log('bar');\n");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from(&[
            ("ci"),
            file_path1.as_os_str().to_str().unwrap(),
            file_path2.as_os_str().to_str().unwrap(),
            "--files-ignore-unknown=true",
        ]),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "ignores_unknown_file",
        fs,
        console,
        result,
    ));
}
