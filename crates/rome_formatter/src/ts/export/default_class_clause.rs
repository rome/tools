use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnyClass, JsExportDefaultClassClause};

impl ToFormatElement for JsExportDefaultClassClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let default_token = self.default_token().format(formatter)?;
        let class = JsAnyClass::from(self.clone()).to_format_element(formatter)?;

        Ok(format_elements![default_token, space_token(), class])
    }
}
