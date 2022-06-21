use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsxAttributeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxAttributeList;

impl FormatNodeRule<JsxAttributeList> for FormatJsxAttributeList {
    fn fmt_fields(&self, node: &JsxAttributeList, f: &mut JsFormatter) -> FormatResult<()> {
        let attributes = format_with(|f| {
            f.join_with(&soft_line_break_or_space())
                .entries(node.iter().formatted())
                .finish()
        });

        write!(f, [group_elements(&soft_block_indent(&attributes))])
    }
}
