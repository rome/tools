use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsMappedTypeOptionalModifierClause;
use rome_js_syntax::TsMappedTypeOptionalModifierClauseFields;

impl FormatNode for TsMappedTypeOptionalModifierClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMappedTypeOptionalModifierClauseFields {
            operator_token,
            question_mark_token,
        } = self.as_fields();

        formatted![
            formatter,
            operator_token.format(formatter)?,
            question_mark_token.format(formatter)?
        ]
    }
}
