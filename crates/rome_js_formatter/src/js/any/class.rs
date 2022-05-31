use crate::generated::FormatJsAnyClass;
use crate::prelude::*;
use rome_js_syntax::JsAnyClass;
use rome_rowan::AstNode;

impl FormatRule<JsAnyClass> for FormatJsAnyClass {
    type Context = JsFormatContext;

    fn format(node: &JsAnyClass, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let abstract_token = node.abstract_token();
        let id = node.id();
        let extends = node.extends_clause();
        let implements_clause = node.implements_clause();

        let format = implements_clause.format();

        let implements_clause = format.with_or_empty(|implements_clause| {
            formatted![formatter, [space_token(), implements_clause]]
        });

        formatted![
            formatter,
            [
                abstract_token
                    .format()
                    .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                node.class_token().format(),
                id.format()
                    .with_or_empty(|id| formatted![formatter, [space_token(), id]]),
                node.type_parameters().format(),
                extends.format().with_or_empty(|extends_clause| formatted![
                    formatter,
                    [space_token(), extends_clause]
                ]),
                implements_clause,
                space_token(),
                formatter
                    .delimited(
                        &node.l_curly_token()?,
                        join_elements_hard_line(
                            node.members()
                                .into_iter()
                                .map(|node| node.syntax().clone())
                                .zip(formatter.format_all(node.members().iter().formatted())?)
                        ),
                        &node.r_curly_token()?
                    )
                    .block_indent()
                    .finish()
            ]
        ]
    }
}
