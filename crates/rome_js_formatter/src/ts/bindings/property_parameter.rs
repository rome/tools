use crate::prelude::*;
use rome_js_syntax::{TsPropertyParameter, TsPropertyParameterFields};

impl FormatNode for TsPropertyParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPropertyParameterFields {
            modifiers,
            formal_parameter,
        } = self.as_fields();

        formatted![
            formatter,
            modifiers.format(formatter)?,
            space_token(),
            formal_parameter.format(formatter)?
        ]
    }
}
