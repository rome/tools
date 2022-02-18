use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsNamedTupleTypeElement;

impl ToFormatElement for TsNamedTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let dotdotdot = self.dotdotdot_token().format_or_empty(formatter)?;
        let name = self.name().format(formatter)?;
        let question_mark = self.question_mark_token().format_or_empty(formatter)?;
        let colon = self.colon_token().format(formatter)?;
        let ty = self.ty().format(formatter)?;
        Ok(format_elements![
            dotdotdot,
            name,
            question_mark,
            colon,
            space_token(),
            ty,
        ])
    }
}
