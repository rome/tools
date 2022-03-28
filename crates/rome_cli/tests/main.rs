use std::{ffi::OsString, path::Path};

use pico_args::Arguments;
use rome_cli::{run_cli, CliSession, Termination};
use rome_console::BufferConsole;
use rome_core::{App, DynRef};
use rome_fs::{FileSystem, MemoryFileSystem};

#[test]
fn test_format_cli() {
    let mut fs = MemoryFileSystem::default();

    let file_path = Path::new("format.js");
    fs.insert(file_path.into(), b"statement()".as_slice());

    let mut console = BufferConsole::default();
    let app =
        App::with_filesystem_and_console(DynRef::Borrowed(&mut fs), DynRef::Borrowed(&mut console));

    let result = run_cli(CliSession {
        app,
        args: Arguments::from_vec(vec![OsString::from("format"), file_path.as_os_str().into()]),
    });

    assert!(result.is_ok(), "run_cli returned {result:?}");

    let mut file = fs
        .open(file_path)
        .expect("formatting target file was removed by the CLI");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("failed to read file from memory FS");

    assert_eq!(content, "statement();\n");

    assert_eq!(console.buffer.len(), 1);
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
fn test_indent_size_parse_errors() {
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
        args: Arguments::from_vec(vec![OsString::from("format")]),
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
    fs.insert(
        file_path.into(),
        b"  unformatted_statement(  )  ".as_slice(),
    );

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
