use crate::prelude::*;
use crate::utils::format_template_chunk;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTemplateChunkElement, TsTemplateChunkElementFields};

impl FormatNodeFields<TsTemplateChunkElement> for FormatNodeRule<TsTemplateChunkElement> {
    fn format_fields(
        node: &TsTemplateChunkElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTemplateChunkElementFields {
            template_chunk_token,
        } = node.as_fields();

        let chunk = template_chunk_token?;
        format_template_chunk(chunk, formatter)
    }
}
