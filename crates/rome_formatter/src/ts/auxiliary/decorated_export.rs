use crate::{
    format_elements, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use crate::formatter_traits::FormatTokenAndNode;

use rome_js_syntax::{TsDecoratedExport, TsDecoratedExportFields};
impl ToFormatElement for TsDecoratedExport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDecoratedExportFields { decorators, export } = self.as_fields();

        Ok(format_elements![
            decorators.format(formatter)?,
            hard_line_break(),
            export.format(formatter)?
        ])
    }
}
