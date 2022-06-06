use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsIndexedAccessType;
use rome_js_syntax::TsIndexedAccessTypeFields;

impl FormatNodeFields<TsIndexedAccessType> for FormatNodeRule<TsIndexedAccessType> {
    fn fmt_fields(node: &TsIndexedAccessType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexedAccessTypeFields {
            object_type,
            l_brack_token,
            index_type,
            r_brack_token,
        } = node.as_fields();
        write![
            f,
            [
                object_type.format(),
                l_brack_token.format(),
                index_type.format(),
                r_brack_token.format()
            ]
        ]
    }
}
