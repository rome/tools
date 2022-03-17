use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{TsDecoratedClassDeclaration, TsDecoratedClassDeclarationFields};

impl ToFormatElement for TsDecoratedClassDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDecoratedClassDeclarationFields {
            decorators,
            declaration,
        } = self.as_fields();

        Ok(format_elements![
            decorators.format(formatter)?,
            hard_line_break(),
            declaration.format(formatter)?
        ])
    }
}
