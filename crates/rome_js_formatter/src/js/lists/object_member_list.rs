use crate::formatter::TrailingSeparator;
use crate::{
    join_elements_soft_line, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::{AstNode, AstSeparatedList, JsObjectMemberList};

impl ToFormatElement for JsObjectMemberList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let members =
            formatter.format_separated(self, || token(","), TrailingSeparator::default())?;

        Ok(join_elements_soft_line(
            self.elements()
                // This unwrap is guarded by the call to format_separated above
                .map(|node| node.node().unwrap().syntax().clone())
                .zip(members),
        ))
    }
}
