use crate::prelude::*;
use crate::utils::format_initializer_clause;
use crate::FormatNodeFields;
use rome_js_syntax::{TsEnumMember, TsEnumMemberFields};

impl FormatNodeFields<TsEnumMember> for FormatNodeRule<TsEnumMember> {
    fn format_fields(node: &TsEnumMember, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsEnumMemberFields { name, initializer } = node.as_fields();

        let name = name.format();
        let initializer = format_initializer_clause(formatter, initializer)?;

        formatted![formatter, [name, initializer]]
    }
}
