//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyInProperty;
impl ToFormatElement for JsAnyInProperty {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsPrivateName(node) => node.format(formatter),
            Self::JsAnyExpression(node) => node.format(formatter),
        }
    }
}
