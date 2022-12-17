use crate::prelude::*;

use crate::parentheses::{operator_type_or_higher_needs_parens, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TsInferType, TsInferTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsInferType;

impl FormatNodeRule<TsInferType> for FormatTsInferType {
    fn fmt_fields(&self, node: &TsInferType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInferTypeFields {
            infer_token,
            name,
            constraint,
        } = node.as_fields();

        if constraint.is_none() {
            write![f, [infer_token.format(), space(), name.format()]]
        } else {
            write![
                f,
                [
                    infer_token.format(),
                    space(),
                    name.format(),
                    space(),
                    constraint.format()
                ]
            ]
        }
    }

    fn needs_parentheses(&self, item: &TsInferType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsInferType {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if parent.kind() == JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT {
            false
        } else {
            operator_type_or_higher_needs_parens(self.syntax(), parent)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::TsInferType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("let s: (infer string)[] = symbol();", TsInferType);

        assert_needs_parentheses!("let s: unique (infer string);", TsInferType);

        assert_not_needs_parentheses!("let s: [number, ...infer string]", TsInferType);
        assert_needs_parentheses!("let s: [(infer string)?]", TsInferType);

        assert_needs_parentheses!("let s: (infer string)[a]", TsInferType);
        assert_not_needs_parentheses!("let s: a[(infer string)]", TsInferType);
    }
}
