use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{TsNamedTupleTypeElement, TsNamedTupleTypeElementFields};

impl ToFormatElement for TsNamedTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNamedTupleTypeElementFields {
            ty,
            question_mark_token,
            colon_token,
            name,
            dotdotdot_token,
        } = self.as_fields();
        let dotdotdot = dotdotdot_token.format_or_empty(formatter)?;
        let name = name.format(formatter)?;
        let question_mark = question_mark_token.format_or_empty(formatter)?;
        let colon = colon_token.format(formatter)?;
        let ty = ty.format(formatter)?;
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
