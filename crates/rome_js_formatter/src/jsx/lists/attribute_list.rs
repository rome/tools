use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsxAttributeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxAttributeList;

impl FormatRule<JsxAttributeList> for FormatJsxAttributeList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsxAttributeList, f: &mut JsFormatter) -> FormatResult<()> {
        let attributes = format_with(|f| {
            f.join_with(&soft_line_break_or_space())
                .entries(node.iter().formatted())
                .finish()
        });

        write!(f, [group(&soft_block_indent(&attributes))])
    }
}
