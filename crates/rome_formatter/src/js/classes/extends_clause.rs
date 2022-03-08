use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsExtendsClause;
use rome_js_syntax::JsExtendsClauseFields;

impl ToFormatElement for JsExtendsClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = self.as_fields();

        Ok(format_elements![
            extends_token.format(formatter)?,
            space_token(),
            super_class.format(formatter)?,
            type_arguments.format_or_empty(formatter)?,
        ])
    }
}
