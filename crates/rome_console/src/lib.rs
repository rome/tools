use std::io::Write;
use std::panic::RefUnwindSafe;

use fmt::Termcolor;
use termcolor::{ColorChoice, StandardStream};

pub mod codespan;
pub mod diff;
pub mod fmt;
mod markup;

pub use self::markup::{Markup, MarkupBuf, MarkupElement, MarkupNode};
pub use rome_markup::markup;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Log,
}

/// Generic abstraction over printing markup and diagnostics to an output,
/// which can be a terminal, a file, a memory buffer ...
pub trait Console: Sync + RefUnwindSafe {
    /// Prints a message (formatted using [markup]) to the console
    fn print(&mut self, level: LogLevel, args: Markup);
}

pub trait ConsoleExt: Console {
    fn error(&mut self, args: Markup);

    fn log(&mut self, args: Markup);
}

impl<T: Console + ?Sized> ConsoleExt for T {
    fn error(&mut self, args: Markup) {
        self.print(LogLevel::Error, args);
    }

    fn log(&mut self, args: Markup) {
        self.print(LogLevel::Log, args);
    }
}

/// Implementation of [Console] printing messages to the standard output
pub struct EnvConsole {
    out: StandardStream,
    err: StandardStream,
}

impl EnvConsole {
    /// Creates an instance of WriteConsole writing to the standard output
    pub fn from_env() -> Self {
        Self {
            out: StandardStream::stdout(ColorChoice::Always),
            err: StandardStream::stderr(ColorChoice::Always),
        }
    }
}

impl Console for EnvConsole {
    fn print(&mut self, level: LogLevel, args: Markup) {
        let mut out = match level {
            LogLevel::Error => self.err.lock(),
            LogLevel::Log => self.out.lock(),
        };

        fmt::Formatter::new(&mut Termcolor(&mut out))
            .write_markup(args)
            .unwrap();

        writeln!(out).unwrap();
    }
}

/// Implementation of [Console] storing all printed messages to a memory buffer
#[derive(Default, Debug)]
pub struct BufferConsole {
    pub buffer: Vec<Message>,
}

/// Individual message entry printed to a [BufferConsole]
#[derive(Debug)]
pub struct Message {
    pub level: LogLevel,
    pub content: MarkupBuf,
}

impl Console for BufferConsole {
    fn print(&mut self, level: LogLevel, args: Markup) {
        self.buffer.push(Message {
            level,
            content: args.to_owned(),
        });
    }
}
