use std::io::Write;
use std::panic::RefUnwindSafe;

use termcolor::{ColorChoice, StandardStream};
use write::Termcolor;

pub mod codespan;
pub mod diff;
pub mod fmt;
mod markup;
mod write;

pub use self::markup::{Markup, MarkupBuf, MarkupElement, MarkupNode};
pub use rome_markup::markup;

/// Determines the "ouput stream" a message should get printed to
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    /// Print the message to the `Error` stream of the console, for instance
    /// "stderr" for the [EnvConsole]
    Error,
    /// Print the message to the `Log` stream of the console, for instance
    /// "stdout" for the [EnvConsole]
    Log,
}

/// Generic abstraction over printing markup and diagnostics to an output,
/// which can be a terminal, a file, a memory buffer ...
pub trait Console: Send + Sync + RefUnwindSafe {
    /// Prints a message (formatted using [markup]) to the console
    fn print(&mut self, level: LogLevel, args: Markup);
}

/// Extension trait for [Console] providing convenience printing methods
pub trait ConsoleExt: Console {
    /// Prints a piece of markup with level [LogLevel::Error]
    fn error(&mut self, args: Markup);

    /// Prints a piece of markup with level [LogLevel::Log]
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

/// Implementation of [Console] printing messages to the standard output and standard error
pub struct EnvConsole {
    out: StandardStream,
    err: StandardStream,
}

impl EnvConsole {
    pub fn new(no_colors: bool) -> Self {
        let out_mode = if no_colors || !atty::is(atty::Stream::Stdout) {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        };
        let err_mode = if no_colors || !atty::is(atty::Stream::Stderr) {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        };

        Self {
            out: StandardStream::stdout(out_mode),
            err: StandardStream::stderr(err_mode),
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
