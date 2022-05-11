use crate::formatter::TrailingSeparator;
use crate::prelude::*;
use rome_js_syntax::JsObjectMemberList;
use rome_rowan::{AstNode, AstSeparatedList};

impl Format for JsObjectMemberList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
