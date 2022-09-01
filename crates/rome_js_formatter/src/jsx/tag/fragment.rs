use crate::prelude::*;

use crate::jsx::tag::element::JsxAnyTagWithChildren;
use rome_formatter::write;
use rome_js_syntax::JsxFragment;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxFragment;

impl FormatNodeRule<JsxFragment> for FormatJsxFragment {
    fn fmt_fields(&self, node: &JsxFragment, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsxAnyTagWithChildren::from(node.clone())])
    }
}
