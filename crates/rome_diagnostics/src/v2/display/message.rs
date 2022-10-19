use rome_console::fmt::{Formatter, Termcolor};
use rome_console::{markup, MarkupBuf};
use termcolor::NoColor;

/// Convenient type that can be used when message and descriptions match, and they need to be
/// displayed using different formatters
///
/// ## Examples
///
/// ```
/// use rome_diagnostics::v2::{Diagnostic, MessageAndDescription};
///
/// #[derive(Debug, Diagnostic)]
/// struct TestDiagnostic {
///     #[message]
///     #[description]
///     message: MessageAndDescription
/// }
/// ```
#[derive(Debug)]
pub struct MessageAndDescription {
    /// Shown when medium supports custom markup
    message: MarkupBuf,
    /// Shown when the medium doesn't support markup
    description: String,
}

impl MessageAndDescription {
    /// It sets a custom message. It updates only the message.
    pub fn set_message(&mut self, new_message: MarkupBuf) {
        self.message = new_message;
    }

    /// It sets a custom description. It updates only the description
    pub fn set_description(&mut self, new_description: String) {
        self.description = new_description;
    }
}

impl From<String> for MessageAndDescription {
    fn from(description: String) -> Self {
        Self {
            message: markup! { {description} }.to_owned(),
            description,
        }
    }
}

impl From<MarkupBuf> for MessageAndDescription {
    fn from(message: MarkupBuf) -> Self {
        let description = markup_to_string(&message);
        Self {
            message,
            description,
        }
    }
}

impl std::fmt::Display for MessageAndDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description)
    }
}

impl rome_console::fmt::Display for MessageAndDescription {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup! {{self.message}})
    }
}

/// Utility function to transform a [MarkupBuf] into a [String]
fn markup_to_string(markup: &MarkupBuf) -> String {
    let mut buffer = Vec::new();
    let mut write = Termcolor(NoColor::new(&mut buffer));
    let mut fmt = Formatter::new(&mut write);
    fmt.write_markup(markup! { {markup} })
        .expect("to have written in the buffer");

    String::from_utf8(buffer).expect("to have convert a buffer into a String")
}
