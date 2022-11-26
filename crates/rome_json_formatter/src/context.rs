use crate::JsonCommentStyle;
use rome_formatter::prelude::*;
use rome_formatter::{
    CstFormatContext, FormatContext, FormatOptions, IndentStyle, LineWidth, TransformSourceMap,
};

use rome_formatter::comments::{Comments, FormatPlainComment};
use rome_json_syntax::JsonLanguage;
use std::fmt;

#[derive(Debug)]
pub struct JsonFormatContext {
    options: JsonFormatOptions,
    comments: Comments<JsonLanguage>,
}

impl JsonFormatContext {
    pub fn new(options: JsonFormatOptions) -> Self {
        Self {
            options,
            comments: Comments::default(),
        }
    }
}

impl FormatContext for JsonFormatContext {
    type Options = JsonFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

impl CstFormatContext for JsonFormatContext {
    type Language = JsonLanguage;
    type Style = JsonCommentStyle;
    type CommentRule = FormatPlainComment<Self>;

    fn comments(&self) -> &Comments<Self::Language> {
        &self.comments
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsonFormatOptions {
    indent_style: IndentStyle,
    line_width: LineWidth,
}

impl JsonFormatOptions {
    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }
}

impl FormatOptions for JsonFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for JsonFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Line width: {}", self.line_width.value())
    }
}
