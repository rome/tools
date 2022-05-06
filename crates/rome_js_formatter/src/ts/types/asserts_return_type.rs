use crate::format_traits::FormatOptional;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsAssertsReturnType;
use rome_js_syntax::TsAssertsReturnTypeFields;

impl FormatNode for TsAssertsReturnType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAssertsReturnTypeFields {
            parameter_name,
            asserts_token,
            predicate,
        } = self.as_fields();
        formatted![
            formatter,
            asserts_token.format(formatter)?,
            space_token(),
            parameter_name.format(formatter)?,
            space_token(),
            predicate.format_or_empty(formatter)?
        ]
    }
}
