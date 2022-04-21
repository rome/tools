use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsPredicateReturnType;
use rome_js_syntax::TsPredicateReturnTypeFields;

impl FormatNode for TsPredicateReturnType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPredicateReturnTypeFields {
            parameter_name,
            is_token,
            ty,
        } = self.as_fields();
        Ok(format_elements![
            parameter_name.format(formatter)?,
            space_token(),
            is_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?
        ])
    }
}
