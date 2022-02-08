use rslint_lexer::T;

use crate::syntax::function::LineBreak;
use crate::syntax::stmt::StatementContext;
use crate::syntax::util::is_at_contextual_keyword;
use crate::ParsedSyntax::{Absent, Present};
use crate::{ParsedSyntax, Parser};

pub(crate) fn is_at_ts_abstract_class_statement(
    p: &Parser,
    should_check_line_break: LineBreak,
) -> bool {
    let tokens = is_at_contextual_keyword(p, "abstract") && p.nth_at(1, T![class]);
    if should_check_line_break == LineBreak::DoCheck {
        tokens && !p.has_linebreak_before_n(1)
    } else {
        tokens
    }
}

// test ts typescript_abstract_classes
// abstract class A {}
pub(crate) fn parse_ts_abstract_class_statement(
    p: &mut Parser,
    _ctx: StatementContext,
) -> ParsedSyntax {
    if !is_at_ts_abstract_class_statement(p, LineBreak::DoCheck) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(rslint_lexer::JsSyntaxKind::ABSTRACT_KW);

    let mut class =
        crate::syntax::class::parse_class(p, m, crate::syntax::class::ClassKind::Declaration);

    // test_err abstract_class_in_js
    // abstract class A {}
    class.err_if_not_ts(
        p,
        "`abstract classes` can only be declared in TypeScript files",
    );
    Present(class)
}
