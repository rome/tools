use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeofType, TsTypeofTypeFields};

impl FormatNodeFields<TsTypeofType> for FormatNodeRule<TsTypeofType> {
    fn format_fields(node: &TsTypeofType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsTypeofTypeFields {
            typeof_token,
            expression_name,
        } = node.as_fields();

        let r#typeof = typeof_token.format();
        let expression_name = expression_name.format();
        formatted![formatter, [r#typeof, space_token(), expression_name]]
    }
}
