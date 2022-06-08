use crate::prelude::*;
use crate::utils::{format_conditional, Conditional};
use crate::FormatNodeFields;
use rome_js_syntax::TsConditionalType;

impl FormatNodeFields<TsConditionalType> for FormatNodeRule<TsConditionalType> {
    fn fmt_fields(node: &TsConditionalType, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_conditional(&Conditional::Type(node.clone()), formatter, false)
    }
}
