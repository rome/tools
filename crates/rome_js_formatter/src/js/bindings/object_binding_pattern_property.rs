use crate::format_traits::FormatOptional;
use crate::utils::{format_property_name, PropertyNameCheckMode};
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
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

        let init_node =
            init.format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;
        Ok(format_elements![
            format_property_name(member?, formatter, PropertyNameCheckMode::Alphanumeric)?,
            colon_token.format(formatter)?,
            space_token(),
            pattern.format(formatter)?,
            init_node,
        ])
    }
}
