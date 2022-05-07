use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{formatted, space_token, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsObjectBindingPatternShorthandProperty;
use rome_js_syntax::JsObjectBindingPatternShorthandPropertyFields;

impl FormatNode for JsObjectBindingPatternShorthandProperty {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternShorthandPropertyFields { identifier, init } = self.as_fields();

        let init_node = init.with_or_empty(|node| formatted![formatter, space_token(), node]);
        formatted![formatter, identifier.format(formatter)?, init_node]
    }
}
