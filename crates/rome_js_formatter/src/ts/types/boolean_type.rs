use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsBooleanType, TsBooleanTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsBooleanType;

impl FormatNodeRule<TsBooleanType> for FormatTsBooleanType {
    fn fmt_fields(&self, node: &TsBooleanType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBooleanTypeFields { boolean_token } = node.as_fields();

        write![f, [boolean_token.format()]]
    }
}
