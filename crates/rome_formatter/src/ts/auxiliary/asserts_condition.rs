use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsAssertsCondition;
use rslint_parser::ast::TsAssertsConditionFields;

impl ToFormatElement for TsAssertsCondition {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAssertsConditionFields { is_token, ty } = self.as_fields();
        Ok(format_elements![
            is_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?
        ])
    }
}
