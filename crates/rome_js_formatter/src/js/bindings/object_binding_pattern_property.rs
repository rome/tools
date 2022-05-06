use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};

use rome_js_syntax::JsObjectBindingPatternProperty;
use rome_js_syntax::JsObjectBindingPatternPropertyFields;

impl FormatNode for JsObjectBindingPatternProperty {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectBindingPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = self.as_fields();

        let init_node = init
            .format_with_or_empty(formatter, |node| formatted![formatter, space_token(), node])?;
        formatted![
            formatter,
            member.format(formatter)?,
            colon_token.format(formatter)?,
            space_token(),
            pattern.format(formatter)?,
            init_node,
        ]
    }
}
