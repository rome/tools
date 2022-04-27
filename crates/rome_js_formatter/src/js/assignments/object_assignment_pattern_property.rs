use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};

use crate::utils::{format_property_name, PropertyNameCheckMode};
use rome_js_syntax::JsObjectAssignmentPatternProperty;
use rome_js_syntax::JsObjectAssignmentPatternPropertyFields;

impl FormatNode for JsObjectAssignmentPatternProperty {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternPropertyFields {
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
