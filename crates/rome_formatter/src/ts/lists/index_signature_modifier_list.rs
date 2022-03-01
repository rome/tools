use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsIndexSignatureModifierList;
use rslint_parser::AstNodeList;

impl ToFormatElement for TsIndexSignatureModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(self.iter())?,
        ))
    }
}
