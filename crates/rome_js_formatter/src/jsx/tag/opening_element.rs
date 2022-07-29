use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsxOpeningElement, JsxOpeningElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxOpeningElement;

impl FormatNodeRule<JsxOpeningElement> for FormatJsxOpeningElement {
    fn fmt_fields(&self, node: &JsxOpeningElement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxOpeningElementFields {
            l_angle_token,
            name,
            type_arguments,
            attributes,
            r_angle_token,
        } = node.as_fields();
        write!(
            f,
            [
                l_angle_token.format(),
                name.format(),
                type_arguments.format(),
                line_suffix_boundary(),
            ]
        )?;

        if !attributes.is_empty() {
            write!(f, [space(), attributes.format(), line_suffix_boundary()])?;
        }

        write!(f, [r_angle_token.format()])
    }
}
