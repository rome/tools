use crate::prelude::*;
use crate::utils::JsObjectLike;
use rome_formatter::write;
use rome_js_syntax::TsObjectType;

#[derive(Debug, Clone, Default)]
pub struct FormatTsObjectType;

impl FormatNodeRule<TsObjectType> for FormatTsObjectType {
    fn fmt_fields(&self, node: &TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectLike::from(node.clone())])
    }
}
