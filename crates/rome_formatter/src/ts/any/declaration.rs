//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyDeclaration;
impl ToFormatElement for TsAnyDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsEnumStatement(node) => node.to_format_element(formatter),
            Self::TsTypeAliasStatement(node) => node.to_format_element(formatter),
            Self::TsDeclareFunctionStatement(node) => node.to_format_element(formatter),
        }
    }
}
