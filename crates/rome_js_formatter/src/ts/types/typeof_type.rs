use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsTypeofType, TsTypeofTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeofType;

impl FormatNodeRule<TsTypeofType> for FormatTsTypeofType {
    fn fmt_fields(node: &TsTypeofType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeofTypeFields {
            typeof_token,
            expression_name,
        } = node.as_fields();

        write![
            f,
            [
                typeof_token.format(),
                space_token(),
                expression_name.format()
            ]
        ]
    }
}
