use crate::{IndentStyle, LineWidth};

/// Options that affect how the [crate::Printer] prints the format tokens
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrinterOptions {
    /// Width of a single tab character (does it equal 2, 4, ... spaces?)
    pub tab_width: u8,

    /// What's the max width of a line. Defaults to 80
    pub print_width: LineWidth,

    /// The type of line ending to apply to the printed input
    pub line_ending: LineEnding,

    /// The never ending question whatever to use spaces or tabs, and if spaces, how many spaces
    /// to indent code.
    ///
    /// * Tab: Value is '\t'
    /// * Spaces: String containing the number of spaces per indention level, e.g. "  " for using two spaces
    pub indent_string: String,
}

impl PrinterOptions {
    pub fn with_print_width(mut self, width: LineWidth) -> Self {
        self.print_width = width;
        self
    }

    pub fn with_indent(mut self, style: IndentStyle) -> Self {
        match style {
            IndentStyle::Tab => {
                self.indent_string = String::from("\t");
                self.tab_width = 2;
            }
            IndentStyle::Space(quantity) => {
                self.indent_string = " ".repeat(quantity as usize);
                self.tab_width = quantity;
            }
        }

        self
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineEnding {
    ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
    LineFeed,

    /// Carriage Return + Line Feed characters (\r\n), common on Windows
    CarriageReturnLineFeed,

    /// Carriage Return character only (\r), used very rarely
    CarriageReturn,
}

impl LineEnding {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            LineEnding::LineFeed => "\n",
            LineEnding::CarriageReturnLineFeed => "\r\n",
            LineEnding::CarriageReturn => "\r",
        }
    }
}

impl Default for PrinterOptions {
    fn default() -> Self {
        PrinterOptions {
            tab_width: 2,
            print_width: LineWidth::default(),
            indent_string: String::from("\t"),
            line_ending: LineEnding::LineFeed,
        }
    }
}
