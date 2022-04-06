use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{TsPropertyParameter, TsPropertyParameterFields};

impl ToFormatElement for TsPropertyParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
