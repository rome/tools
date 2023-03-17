use crate::configs::{CONFIG_DISABLED_FORMATTER, CONFIG_FILE_SIZE_LIMIT, CONFIG_LINTER_DISABLED};
use crate::snap_test::SnapshotPayload;
use crate::{
    assert_cli_snapshot, run_cli, CUSTOM_FORMAT_BEFORE, FORMATTED, LINT_ERROR, PARSE_ERROR,
    UNFORMATTED,
};
use pico_args::Arguments;
use rome_console::{BufferConsole, MarkupBuf};
use rome_fs::{FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;
use std::ffi::OsString;
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
fn ok() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
        Arguments::from_vec(vec![OsString::from("ci"), input_file.as_os_str().into()]),
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
        Arguments::from_vec(vec![
            OsString::from("ci"),
            OsString::from("--formatter-enabled=false"),
            input_file.as_os_str().into(),
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
        Arguments::from_vec(vec![
            OsString::from("ci"),
            OsString::from("--linter-enabled=false"),
            file_path.as_os_str().into(),
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
        Arguments::from_vec(vec![
            OsString::from("ci"),
            OsString::from("--organize-imports-enabled=false"),
            file_path.as_os_str().into(),
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
        Arguments::from_vec(vec![
            OsString::from("ci"),
            OsString::from("--linter-enabled=false"),
            OsString::from("--formatter-enabled=false"),
            OsString::from("--organize-imports-enabled=false"),
            file_path.as_os_str().into(),
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
        Arguments::from_vec(vec![
            OsString::from("ci"),
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

    let file_path = Path::new("ci.js");
    fs.insert(file_path.into(), "statement1();\nstatement2();");

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Arguments::from_vec(vec![
            OsString::from("ci"),
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
        Arguments::from_vec(vec![OsString::from("ci"), OsString::from("src")]),
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
        Arguments::from_vec(vec![
            OsString::from("ci"),
            OsString::from("--max-diagnostics"),
            OsString::from("10"),
            OsString::from("src"),
        ]),
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
        Arguments::from_vec(vec![
            OsString::from("ci"),
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
        Arguments::from_vec(vec![OsString::from("ci"), file_path.as_os_str().into()]),
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
