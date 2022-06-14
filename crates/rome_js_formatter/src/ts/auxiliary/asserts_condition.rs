use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsAssertsCondition;
use rome_js_syntax::TsAssertsConditionFields;

impl FormatNodeFields<TsAssertsCondition> for FormatNodeRule<TsAssertsCondition> {
    fn fmt_fields(node: &TsAssertsCondition, f: &mut JsFormatter) -> FormatResult<()> {
        let TsAssertsConditionFields { is_token, ty } = node.as_fields();
        write![f, [is_token.format(), space_token(), ty.format()]]
    }
}
