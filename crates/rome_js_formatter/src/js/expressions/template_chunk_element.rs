use crate::prelude::*;
use crate::utils::format_template_chunk;

use crate::FormatNodeFields;
use rome_js_syntax::{JsTemplateChunkElement, JsTemplateChunkElementFields};

impl FormatNodeFields<JsTemplateChunkElement> for FormatNodeRule<JsTemplateChunkElement> {
    fn format_fields(
        node: &JsTemplateChunkElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsTemplateChunkElementFields {
            template_chunk_token,
        } = node.as_fields();

        let chunk = template_chunk_token?;
        format_template_chunk(chunk, formatter)
    }
}
