use crate::context::QuoteStyle;
use crate::JsFormatContext;
use rome_formatter::prelude::*;

/// Creates either a space using an expression child and a string literal,
/// or a regular space, depending on whether the group breaks or not.
///
/// ```jsx
///  <div> Winter Light </div>;
///
///  <div>
///    {" "}Winter Light
///    Through A Glass Darkly
///    The Silence
///    Seventh Seal
///    Wild Strawberries
///  </div>
/// ```
#[derive(Default)]
pub struct JsxSpace {}

impl Format for JsxSpace {
    type Context = JsFormatContext;

    fn format(&self, formatter: &Formatter<Self::Context>) -> FormatResult<FormatElement> {
        let jsx_space = match formatter.context().quote_style() {
            QuoteStyle::Double => "{\" \"}",
            QuoteStyle::Single => "{\' \'}",
        };

        Ok(format_elements![
            if_group_breaks(format_elements![token(jsx_space), soft_line_break()]),
            if_group_fits_on_single_line(space_token())
        ])
    }
}
