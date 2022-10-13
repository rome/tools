use crate::configs::{
    CONFIG_DISABLED_FORMATTER, CONFIG_FORMAT, CONFIG_FORMATTER_IGNORED_FILES, CONFIG_ISSUE_3175_1,
    CONFIG_ISSUE_3175_2,
};
use crate::snap_test::{markup_to_string, SnapshotPayload};
use crate::{
    assert_cli_snapshot, run_cli, CUSTOM_FORMAT_BEFORE, FORMATTED, LINT_ERROR, UNFORMATTED,
};
use pico_args::Arguments;
use rome_cli::Termination;
use rome_console::{markup, BufferConsole};
use rome_fs::{FileSystemExt, MemoryFileSystem};
use rome_service::DynRef;
use std::ffi::OsString;
use std::path::Path;

// six spaces
const CUSTOM_FORMAT_AFTER: &str = r#"function f() {
      return { something };
}
"#;

const APPLY_QUOTE_STYLE_BEFORE: &str = r#"
let a = "something";
let b = {
    "hey": "hello"
};"#;

const APPLY_QUOTE_STYLE_AFTER: &str = "let a = 'something';
let b = {\n\t'hey': 'hello',\n};\n";

const CUSTOM_CONFIGURATION_BEFORE: &str = r#"function f() {
  return { a, b }
}"#;

const CUSTOM_CONFIGURATION_AFTER: &str = "function f() {
        return {
                a,
                b,
        };
}
";

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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_print",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_write",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "formatter_lint_warning",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--line-width"),
            OsString::from("10"),
            OsString::from("--indent-style"),
            OsString::from("space"),
            OsString::from("--indent-size"),
            OsString::from("8"),
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

    assert_eq!(content, CUSTOM_CONFIGURATION_AFTER);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration_over_config_file() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_FORMAT.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), CUSTOM_CONFIGURATION_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--line-width"),
            OsString::from("10"),
            OsString::from("--indent-style"),
            OsString::from("space"),
            OsString::from("--indent-size"),
            OsString::from("8"),
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

    assert_eq!(content, CUSTOM_CONFIGURATION_AFTER);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration_over_config_file",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration_over_config_file_issue_3175_v1() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_ISSUE_3175_1.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), "import React from 'react';\n".as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--quote-style"),
            OsString::from("single"),
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

    assert_eq!(content, "import React from 'react';\n");

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration_over_config_file_issue_3175_v1",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_configuration_over_config_file_issue_3175_v2() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let source = r#"function f() {
  return 'hey';
}
"#;

    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_ISSUE_3175_2.as_bytes());

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), source.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--indent-style"),
            OsString::from("space"),
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

    assert_eq!(content, source);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_configuration_over_config_file_issue_3175_v2",
        fs,
        console,
        result,
    ));
}

#[test]
fn applies_custom_quote_style() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("file.js");
    fs.insert(file_path.into(), APPLY_QUOTE_STYLE_BEFORE.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--quote-style"),
            OsString::from("single"),
            OsString::from("--quote-properties"),
            OsString::from("preserve"),
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

    assert_eq!(content, APPLY_QUOTE_STYLE_AFTER);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "applies_custom_quote_style",
        fs,
        console,
        result,
    ));
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_style_parse_errors",
        fs,
        console,
        result,
    ));
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_size_parse_errors_negative",
        fs,
        console,
        result,
    ));
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "indent_size_parse_errors_overflow",
        fs,
        console,
        result,
    ));
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

    match &result {
        Err(Termination::ParseError { argument, .. }) => assert_eq!(*argument, "--line-width"),
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "line_width_parse_errors_negative",
        fs,
        console,
        result,
    ));
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

    match &result {
        Err(Termination::ParseError { argument, .. }) => assert_eq!(*argument, "--line-width"),
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "line_width_parse_errors_overflow",
        fs,
        console,
        result,
    ));
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

    match &result {
        Err(Termination::ParseError { argument, .. }) => {
            assert_eq!(*argument, "--quote-properties")
        }
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "quote_properties_parse_errors_letter_case",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_with_configuration",
        fs,
        console,
        result,
    ));
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
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_is_disabled",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_successfully",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "format_stdin_with_errors",
        fs,
        console,
        result,
    ));
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

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_format_if_disabled",
        fs,
        console,
        result,
    ));
}

#[test]
fn does_not_format_ignored_files() {
    let mut console = BufferConsole::default();
    let mut fs = MemoryFileSystem::default();
    let file_path = Path::new("rome.json");
    fs.insert(file_path.into(), CONFIG_FORMATTER_IGNORED_FILES.as_bytes());

    let file_path = Path::new("test.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("test.js"),
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

    assert_eq!(content, UNFORMATTED);

    drop(file);
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "does_not_format_ignored_files",
        fs,
        console,
        result,
    ));
}

#[test]
fn file_too_large() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), "statement();\n".repeat(80660).as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        DynRef::Borrowed(&mut console),
        Arguments::from_vec(vec![
            OsString::from("format"),
            file_path.as_os_str().into(),
            OsString::from("--write"),
        ]),
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
