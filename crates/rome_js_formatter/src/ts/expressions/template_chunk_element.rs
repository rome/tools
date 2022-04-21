use crate::utils::format_template_chunk;
use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::{TsTemplateChunkElement, TsTemplateChunkElementFields};

impl FormatNode for TsTemplateChunkElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTemplateChunkElementFields {
            template_chunk_token,
        } = self.as_fields();

        let chunk = template_chunk_token?;
        format_template_chunk(chunk, formatter)
    }
}
