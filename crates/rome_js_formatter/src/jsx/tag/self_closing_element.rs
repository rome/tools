use crate::prelude::*;

use crate::jsx::tag::opening_element::JsxAnyOpeningElement;

use rome_js_syntax::{
    JsxSelfClosingElement,
};


#[derive(Debug, Clone, Default)]
pub struct FormatJsxSelfClosingElement;

impl FormatNodeRule<JsxSelfClosingElement> for FormatJsxSelfClosingElement {
    fn fmt_fields(&self, node: &JsxSelfClosingElement, f: &mut JsFormatter) -> FormatResult<()> {
        JsxAnyOpeningElement::from(node.clone()).fmt(f)
    }
}
