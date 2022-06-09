use crate::generated::FormatJsxAttributeList;
use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsxAttributeList;

impl FormatRule<JsxAttributeList> for FormatJsxAttributeList {
    type Context = JsFormatContext;

    fn fmt(node: &JsxAttributeList, f: &mut JsFormatter) -> FormatResult<()> {
        let attributes = format_with(|f| {
            f.join_with(&soft_line_break_or_space())
                .entries(node.iter().formatted())
                .finish()
        });

        write!(f, [group_elements(&soft_block_indent(&attributes))])
    }
}
