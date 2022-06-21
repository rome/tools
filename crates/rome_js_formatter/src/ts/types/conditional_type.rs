use crate::prelude::*;
use crate::utils::{format_conditional, Conditional};

use rome_js_syntax::TsConditionalType;

#[derive(Debug, Clone, Default)]
pub struct FormatTsConditionalType;

impl FormatNodeRule<TsConditionalType> for FormatTsConditionalType {
    fn fmt_fields(
        &self,
        node: &TsConditionalType,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_conditional(&Conditional::Type(node.clone()), formatter, false)
    }
}
