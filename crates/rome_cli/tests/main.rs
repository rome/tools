use std::{ffi::OsString, path::Path};

use pico_args::Arguments;
use rome_cli::{run_cli, CliSession, Termination};
use rome_console::BufferConsole;
use rome_core::{App, DynRef};
use rome_fs::{FileSystem, MemoryFileSystem};

const UNFORMATTED: &str = "  statement(  )  ";
const FORMATTED: &str = "statement();\n";

#[test]
fn test_format_print() {
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
fn test_format_write() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let mut console = BufferConsole::default();
    let app =
        App::with_filesystem_and_console(DynRef::Borrowed(&mut fs), DynRef::Borrowed(&mut console));

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

#[test]
fn test_format_ci() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), FORMATTED.as_bytes());

    let mut console = BufferConsole::default();
    let app =
        App::with_filesystem_and_console(DynRef::Borrowed(&mut fs), DynRef::Borrowed(&mut console));

    let result = run_cli(CliSession {
        app,
        args: Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--ci"),
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

    if console.buffer.len() != 1 {
        panic!("unexpected console content: {:#?}", console.buffer);
    }
}

#[test]
fn test_unknown_command() {
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
fn test_unknown_command_help() {
    let result = run_cli(CliSession {
        app: App::with_filesystem_and_console(
            DynRef::Owned(Box::new(MemoryFileSystem::default())),
            DynRef::Owned(Box::new(BufferConsole::default())),
        ),
        args: Arguments::from_vec(vec![OsString::from("unknown"), OsString::from("--help")]),
    });

    match result {
        Err(Termination::UnknownCommandHelp { command }) => assert_eq!(command, "unknown"),
        _ => panic!("run_cli returned {result:?} for an unknown command help, expected an error"),
    }
}

#[test]
fn test_indent_style_parse_errors() {
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }
}

#[test]
fn test_indent_size_parse_errors_negative() {
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }
}

#[test]
fn test_indent_size_parse_errors_overflow() {
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }
}

#[test]
fn test_line_width_parse_errors_negative() {
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }
}

#[test]
fn test_line_width_parse_errors_overflow() {
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
        _ => panic!("run_cli returned {result:?} for an invalid argument value, expected an error"),
    }
}

#[test]
fn test_unexpected_argument() {
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
fn test_missing_argument() {
    let result = run_cli(CliSession {
        app: App::with_filesystem_and_console(
            DynRef::Owned(Box::new(MemoryFileSystem::default())),
            DynRef::Owned(Box::new(BufferConsole::default())),
        ),
        args: Arguments::from_vec(vec![OsString::from("format"), OsString::from("--ci")]),
    });

    match result {
        Err(Termination::MissingArgument { argument }) => assert_eq!(argument, "<INPUT>"),
        _ => panic!("run_cli returned {result:?} for a missing argument, expected an error"),
    }
}

#[test]
fn test_formatting_error() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), UNFORMATTED.as_bytes());

    let result = run_cli(CliSession {
        app: App::with_filesystem_and_console(
            DynRef::Owned(Box::new(fs)),
            DynRef::Owned(Box::new(BufferConsole::default())),
        ),
        args: Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--ci"),
            file_path.as_os_str().into(),
        ]),
    });

    match result {
        Err(Termination::FormattingError) => {}
        _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
    }
}

#[test]
fn test_empty_arguments() {
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
fn test_incompatible_arguments() {
    let result = run_cli(CliSession {
        app: App::with_filesystem_and_console(
            DynRef::Owned(Box::new(MemoryFileSystem::default())),
            DynRef::Owned(Box::new(BufferConsole::default())),
        ),
        args: Arguments::from_vec(vec![
            OsString::from("format"),
            OsString::from("--write"),
            OsString::from("--ci"),
            OsString::from("format.js"),
        ]),
    });

    match result {
        Err(Termination::IncompatibleArguments("--write", "--ci")) => {}
        _ => panic!("run_cli returned {result:?} for a failed CI check, expected an error"),
    }
}
