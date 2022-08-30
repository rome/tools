use crate::prelude::*;

use crate::utils::{with_assignment_layout, AssignmentLikeLayout};
use rome_formatter::{write, FormatRuleWithOptions};
use rome_js_syntax::JsInitializerClause;
use rome_js_syntax::JsInitializerClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsInitializerClause {
    assignment_layout: Option<AssignmentLikeLayout>,
}

#[derive(Default, Debug)]
pub struct FormatJsInitializerClauseOptions {
    pub(crate) assignment_layout: Option<AssignmentLikeLayout>,
}

impl FormatRuleWithOptions<JsInitializerClause> for FormatJsInitializerClause {
    type Options = FormatJsInitializerClauseOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.assignment_layout = options.assignment_layout;
        self
    }
}

impl FormatNodeRule<JsInitializerClause> for FormatJsInitializerClause {
    fn fmt_fields(&self, node: &JsInitializerClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsInitializerClauseFields {
            eq_token,
            expression,
        } = node.as_fields();

        write![
            f,
            [
                eq_token.format(),
                space(),
                with_assignment_layout(&expression?, self.assignment_layout)
            ]
        ]
    }
}
