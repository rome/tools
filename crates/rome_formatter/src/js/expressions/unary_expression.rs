use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsUnaryExpression;

use rslint_parser::{token_set, T};

impl ToFormatElement for JsUnaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let operator = self.operator()?;
        let space_or_empty =
            if token_set![T![delete], T![void], T![typeof]].contains(operator.kind()) {
                space_token()
            } else {
                empty_element()
            };
        Ok(format_elements![
            self.operator().format(formatter)?,
            space_or_empty,
            self.argument().format(formatter)?,
        ])
    }
}
