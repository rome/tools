use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsImplementsClause;
use rslint_parser::ast::TsImplementsClauseFields;

impl ToFormatElement for TsImplementsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsImplementsClauseFields {
            implements_token,
            types,
        } = self.as_fields();

        Ok(format_elements![
            implements_token.format(formatter)?,
            space_token(),
            types.format(formatter)?,
        ])
    }
}
