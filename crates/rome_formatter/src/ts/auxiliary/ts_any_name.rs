use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyName;

impl ToFormatElement for TsAnyName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            TsAnyName::JsReferenceIdentifier(node) => node.to_format_element(formatter),
            TsAnyName::TsQualifiedName(_) => todo!(),
        }
    }
}
