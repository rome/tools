use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsAssertsCondition;
use rome_js_syntax::TsAssertsConditionFields;

impl FormatNodeFields<TsAssertsCondition> for FormatNodeRule<TsAssertsCondition> {
    fn format_fields(
        node: &TsAssertsCondition,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsAssertsConditionFields { is_token, ty } = node.as_fields();
        formatted![formatter, [is_token.format(), space_token(), ty.format()]]
    }
}
