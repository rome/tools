use crate::prelude::*;

use crate::parentheses::{
    is_in_many_type_union_or_intersection_list, operator_type_or_higher_needs_parens,
    NeedsParentheses,
};
use crate::utils::FormatTypeMemberSeparator;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsSyntaxKind, JsSyntaxNode, TsIntersectionTypeElementList, TsIntersectionTypeFields,
    TsUnionTypeVariantList,
};
use rome_js_syntax::{JsSyntaxToken, TsIntersectionType};

#[derive(Debug, Clone, Default)]
pub struct FormatTsIntersectionType;

impl FormatNodeRule<TsIntersectionType> for FormatTsIntersectionType {
    fn fmt_fields(&self, node: &TsIntersectionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIntersectionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();
        write!(
            f,
            [group(&format_args!(
                FormatTypeMemberSeparator::new(leading_separator_token.as_ref()),
                types.format()
            ))]
        )
    }

    fn needs_parentheses(&self, item: &TsIntersectionType) -> bool {
        item.needs_parentheses()
    }
}

pub struct FormatTypeSetLeadingSeparator<'a> {
    pub(crate) separator: JsSyntaxKind,
    pub(crate) leading_separator: Option<&'a JsSyntaxToken>,
}

impl Format<JsFormatContext> for FormatTypeSetLeadingSeparator<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match &self.leading_separator {
            Some(token) => {
                format_only_if_breaks(token, &format_args!(token.format(), space())).fmt(f)
            }
            None => write!(
                f,
                [if_group_breaks(&format_args![
                    format_inserted(self.separator),
                    space()
                ])]
            ),
        }
    }
}

impl NeedsParentheses for TsIntersectionType {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        union_or_intersection_type_needs_parentheses(
            self.syntax(),
            parent,
            &TsIntersectionOrUnionTypeList::TsIntersectionTypeElementList(self.types()),
        )
    }
}

pub(super) fn union_or_intersection_type_needs_parentheses(
    node: &JsSyntaxNode,
    parent: &JsSyntaxNode,
    types: &TsIntersectionOrUnionTypeList,
) -> bool {
    debug_assert!(matches!(
        node.kind(),
        JsSyntaxKind::TS_INTERSECTION_TYPE | JsSyntaxKind::TS_UNION_TYPE
    ));

    if is_in_many_type_union_or_intersection_list(node, parent) {
        types.len() > 1
    } else {
        operator_type_or_higher_needs_parens(node, parent)
    }
}

pub(super) enum TsIntersectionOrUnionTypeList {
    TsIntersectionTypeElementList(TsIntersectionTypeElementList),
    TsUnionTypeVariantList(TsUnionTypeVariantList),
}

impl TsIntersectionOrUnionTypeList {
    fn len(&self) -> usize {
        match self {
            TsIntersectionOrUnionTypeList::TsIntersectionTypeElementList(list) => list.len(),
            TsIntersectionOrUnionTypeList::TsUnionTypeVariantList(list) => list.len(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::TsIntersectionType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("let s: (string & number)[] = symbol();", TsIntersectionType);

        assert_needs_parentheses!("let s: unique (string & number);", TsIntersectionType);

        assert_needs_parentheses!("let s: [number, ...(string & number)]", TsIntersectionType);
        assert_needs_parentheses!("let s: [(string & number)?]", TsIntersectionType);

        assert_needs_parentheses!("let s: (string & number)[a]", TsIntersectionType);
        assert_not_needs_parentheses!("let s: a[(string & number)]", TsIntersectionType);

        assert_not_needs_parentheses!("let s: (&a) & (&b)", TsIntersectionType[1]);
        assert_not_needs_parentheses!("let s: (&a) & (&b)", TsIntersectionType[2]);

        assert_needs_parentheses!("let s: (a & b) & (&c)", TsIntersectionType[1]);
        assert_not_needs_parentheses!("let s: (a & b) & (&c)", TsIntersectionType[2]);
    }
}
