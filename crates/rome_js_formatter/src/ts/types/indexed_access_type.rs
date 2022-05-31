use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsIndexedAccessType;
use rome_js_syntax::TsIndexedAccessTypeFields;

impl FormatNodeFields<TsIndexedAccessType> for FormatNodeRule<TsIndexedAccessType> {
    fn format_fields(
        node: &TsIndexedAccessType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsIndexedAccessTypeFields {
            object_type,
            l_brack_token,
            index_type,
            r_brack_token,
        } = node.as_fields();
        formatted![
            formatter,
            [
                object_type.format(),
                l_brack_token.format(),
                index_type.format(),
                r_brack_token.format()
            ]
        ]
    }
}
