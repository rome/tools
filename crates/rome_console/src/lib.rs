use std::panic::RefUnwindSafe;

use rome_diagnostics::{file::Files, Diagnostic, Emitter};
use termcolor::{ColorChoice, NoColor, StandardStream, WriteColor};

mod markup;

pub use self::markup::{Markup, MarkupElement, MarkupNode};
pub use rome_markup::markup;

/// Generic abstraction over printing markup and diagnostics to an output,
/// which can be a terminal, a file, a memory buffer ...
pub trait Console: Sync + RefUnwindSafe {
    /// Prints a message (formatted using [markup]) to the console
    fn message(&mut self, args: Markup);

    /// Prints a diagnostic to the console using the provided file map to
    /// display source code
    fn diagnostic(&mut self, fs: &dyn Files, diag: &Diagnostic);
}

/// Implementation of [Console] printing messages to a writable stream
pub struct WriteConsole<O, E> {
    out: O,
    err: E,
}

impl<O, E> Console for WriteConsole<O, E>
where
    O: WriteColor + Sync + RefUnwindSafe,
    E: WriteColor + Sync + RefUnwindSafe,
{
    fn message(&mut self, args: Markup) {
        args.print(&mut self.out).unwrap();
        writeln!(self.out).unwrap();
    }

    fn diagnostic(&mut self, fs: &dyn Files, diag: &Diagnostic) {
        Emitter::new(fs)
            .emit_with_writer(diag, &mut self.err)
            .unwrap();
    }
}

/// Type alias of [WriteConsole] printing to the standard output
pub type EnvConsole = WriteConsole<StandardStream, StandardStream>;

impl EnvConsole {
    /// Creates an instance of WriteConsole writing to the standard output
    pub fn from_env() -> Self {
        Self {
            out: StandardStream::stdout(ColorChoice::Always),
            err: StandardStream::stderr(ColorChoice::Always),
        }
    }
}

/// Implementation of [Console] storing all printed messages to a memory buffer
#[derive(Default, Debug)]
pub struct BufferConsole {
    pub buffer: Vec<Message>,
}

/// Individual message entry printed to a [BufferConsole]
#[derive(Debug)]
pub enum Message {
    Message(String),
    Diagnostic(Diagnostic),
}

impl Console for BufferConsole {
    fn message(&mut self, args: Markup) {
        let mut message = Vec::new();

        {
            let mut writer = NoColor::new(&mut message);
            args.print(&mut writer).unwrap();
        }

        let message = String::from_utf8(message).unwrap();
        self.buffer.push(Message::Message(message));
    }

    fn diagnostic(&mut self, _: &dyn Files, diag: &Diagnostic) {
        self.buffer.push(Message::Diagnostic(diag.clone()));
    }
}
