use crate::context::QuoteStyle;
use crate::{JsFormatContext, JsFormatter};
use rome_formatter::if_group_fits_on_line;
use rome_formatter::prelude::*;
use rome_formatter::{format_args, write};

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

impl Format<JsFormatContext> for JsxSpace {
    fn fmt(&self, formatter: &mut JsFormatter) -> FormatResult<()> {
        let jsx_space = match formatter.context().quote_style() {
            QuoteStyle::Double => "{\" \"}",
            QuoteStyle::Single => "{\' \'}",
        };

        write![
            formatter,
            [
                if_group_breaks(&format_args![text(jsx_space), hard_line_break()]),
                if_group_fits_on_line(&space())
            ]
        ]
    }
}
