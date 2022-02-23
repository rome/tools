use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::TsConditionalType;
use rslint_parser::ast::TsConditionalTypeFields;

impl ToFormatElement for TsConditionalType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsConditionalTypeFields {
            check_type,
            extends_token,
            extends_type,
            question_mark_token,
            true_type,
            colon_token,
            false_type,
        } = self.as_fields();
        Ok(format_elements![
            check_type.format(formatter)?,
            space_token(),
            extends_token.format(formatter)?,
            space_token(),
            extends_type.format(formatter)?,
            space_token(),
            group_elements(format_elements![
                question_mark_token.format(formatter)?,
                space_token(),
                true_type.format(formatter)?,
            ]),
            space_token(),
            group_elements(format_elements![
                colon_token.format(formatter)?,
                space_token(),
                false_type.format(formatter)?
            ]),
        ])
    }
}
