use crate::formatter::TrailingSeparator;
use crate::{
    format_element::join_elements_soft_line, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::{ast::JsObjectMemberList, AstSeparatedList};

impl ToFormatElement for JsObjectMemberList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let members =
            formatter.format_separated(self.clone(), || token(","), TrailingSeparator::Allowed)?;

        Ok(join_elements_soft_line(
            self.elements()
                // This unwrap is guarded by the call to format_separated above
                .map(|node| node.node().unwrap())
                .zip(members),
        ))
    }
}
