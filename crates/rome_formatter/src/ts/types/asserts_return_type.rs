use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsAssertsReturnType;
use rome_js_syntax::TsAssertsReturnTypeFields;

impl ToFormatElement for TsAssertsReturnType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAssertsReturnTypeFields {
            parameter_name,
            asserts_token,
            predicate,
        } = self.as_fields();
        Ok(format_elements![
            asserts_token.format(formatter)?,
            space_token(),
            parameter_name.format(formatter)?,
            space_token(),
            predicate.format_or_empty(formatter)?
        ])
    }
}
