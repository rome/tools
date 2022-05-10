use crate::format_traits::FormatOptional;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsNamedTupleTypeElement, TsNamedTupleTypeElementFields};

impl FormatNode for TsNamedTupleTypeElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
        formatted![
            formatter,
            dotdotdot,
            name,
            question_mark,
            colon,
            space_token(),
            ty,
        ]
    }
}
