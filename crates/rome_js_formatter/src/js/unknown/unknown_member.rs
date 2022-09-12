

use crate::FormatUnknownNodeRule;
use rome_js_syntax::JsUnknownMember;


#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownMember;

impl FormatUnknownNodeRule<JsUnknownMember> for FormatJsUnknownMember {}
