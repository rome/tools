use crate::prelude::*;
use crate::utils::format_template_chunk;

use rome_js_syntax::{TsTemplateChunkElement, TsTemplateChunkElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateChunkElement;

impl FormatNodeRule<TsTemplateChunkElement> for FormatTsTemplateChunkElement {
    fn fmt_fields(
        &self,
        node: &TsTemplateChunkElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsTemplateChunkElementFields {
            template_chunk_token,
        } = node.as_fields();

        let chunk = template_chunk_token?;
        format_template_chunk(chunk, formatter)
    }
}
