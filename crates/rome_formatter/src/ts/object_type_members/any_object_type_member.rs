use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyObjectTypeMember;

impl ToFormatElement for TsAnyObjectTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            TsAnyObjectTypeMember::TsCallSignatureObjectTypeMember(_) => todo!(),
            TsAnyObjectTypeMember::TsConstructSignatureObjectTypeMember(_) => todo!(),
            TsAnyObjectTypeMember::TsGetterSignatureObjectTypeMember(_) => todo!(),
            TsAnyObjectTypeMember::TsIndexSignatureObjectTypeMember(_) => todo!(),
            TsAnyObjectTypeMember::TsMethodSignatureObjectTypeMember(_) => todo!(),
            TsAnyObjectTypeMember::TsPropertySignatureObjectTypeMember(node) => {
                node.to_format_element(formatter)
            }
            TsAnyObjectTypeMember::TsSetterSignatureObjectTypeMember(_) => todo!(),
        }
    }
}
