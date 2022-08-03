use crate::prelude::*;
use crate::utils::JsAnyConditional;

use rome_js_syntax::TsConditionalType;

#[derive(Debug, Clone, Default)]
pub struct FormatTsConditionalType;

impl FormatNodeRule<TsConditionalType> for FormatTsConditionalType {
    fn fmt_fields(
        &self,
        node: &TsConditionalType,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        JsAnyConditional::from(node.clone()).fmt(formatter)
    }
}
