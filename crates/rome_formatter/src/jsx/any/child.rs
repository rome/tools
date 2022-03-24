//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxAnyChild;
impl ToFormatElement for JsxAnyChild {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxElement(node) => node.format(formatter),
            Self::JsxSelfClosingElement(node) => node.format(formatter),
            Self::JsxText(node) => node.format(formatter),
            Self::JsxExpressionChild(node) => node.format(formatter),
            Self::JsxSpreadChild(node) => node.format(formatter),
        }
    }
}
