use crate::prelude::*;
use crate::utils::format_template_chunk;

use rome_js_syntax::{JsTemplateChunkElement, JsTemplateChunkElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsTemplateChunkElement;

impl FormatNodeRule<JsTemplateChunkElement> for FormatJsTemplateChunkElement {
    fn fmt_fields(
        &self,
        node: &JsTemplateChunkElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsTemplateChunkElementFields {
            template_chunk_token,
        } = node.as_fields();

        let chunk = template_chunk_token?;
        format_template_chunk(chunk, formatter)
    }
}
