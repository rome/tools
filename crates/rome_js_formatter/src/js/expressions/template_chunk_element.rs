use crate::prelude::*;
use crate::utils::format_template_chunk;

use crate::FormatNodeFields;
use rome_js_syntax::{JsTemplateChunkElement, JsTemplateChunkElementFields};

impl FormatNodeFields<JsTemplateChunkElement> for FormatNodeRule<JsTemplateChunkElement> {
    fn fmt_fields(node: &JsTemplateChunkElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        let JsTemplateChunkElementFields {
            template_chunk_token,
        } = node.as_fields();

        let chunk = template_chunk_token?;
        format_template_chunk(chunk, formatter)
    }
}
