use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsPropertyParameter, TsPropertyParameterFields};

impl FormatNode for TsPropertyParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPropertyParameterFields {
            modifiers,
            formal_parameter,
        } = self.as_fields();

        Ok(format_elements![
            modifiers.format(formatter)?,
            space_token(),
            formal_parameter.format(formatter)?
        ])
    }
}
