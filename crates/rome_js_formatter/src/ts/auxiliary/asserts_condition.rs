use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsAssertsCondition;
use rome_js_syntax::TsAssertsConditionFields;

impl FormatNode for TsAssertsCondition {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsAssertsConditionFields { is_token, ty } = self.as_fields();
        Ok(format_elements![
            is_token.format(formatter)?,
            space_token(),
            ty.format(formatter)?
        ])
    }
}
