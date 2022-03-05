use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_syntax::TsPredicateReturnType;
use rslint_syntax::TsPredicateReturnTypeFields;

impl ToFormatElement for TsPredicateReturnType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
