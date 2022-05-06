use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsAssertsCondition;
use rome_js_syntax::TsAssertsConditionFields;

impl FormatNode for TsAssertsCondition {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAssertsConditionFields { is_token, ty } = self.as_fields();
        formatted![
            formatter,
            is_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?
        ]
    }
}
