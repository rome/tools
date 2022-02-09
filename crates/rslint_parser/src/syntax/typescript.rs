//! TypeScript specific functions.

mod statement;
mod ts_parse_error;
mod types;

use crate::syntax::expr::{parse_identifier, parse_unary_expr, ExpressionContext};
use crate::syntax::js_parse_error::{expected_expression, expected_identifier, expected_ts_type};

use crate::syntax::util::{expect_contextual_keyword, is_at_contextual_keyword};
use crate::{JsSyntaxKind::*, *};
use rome_rowan::SyntaxKind;

pub(crate) use self::statement::*;
pub(crate) use self::types::*;

fn parse_ts_identifier_binding(p: &mut Parser) -> ParsedSyntax {
    parse_identifier(p, TS_IDENTIFIER_BINDING).map(|mut ident| {
        if ident.kind().is_unknown() {
            return ident;
        }

        let name = p.source(ident.range(p));

        if is_reserved_type_name(name) {
            let error = p
                .err_builder(&format!("Type alias cannot be {}", name))
                .primary(ident.range(p), "");
            p.error(error);
            ident.change_to_unknown(p);
        }

        ident
    })
}

// test ts ts_type_assertion_expression
// let x = <const>"hello";
// let y = <string> x;
pub(crate) fn parse_ts_type_assertion_expression(
    p: &mut Parser,
    context: ExpressionContext,
) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    p.expect(T![>]);
    parse_unary_expr(p, context).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, TS_TYPE_ASSERTION_EXPRESSION))
}
pub(crate) fn parse_ts_implements_clause(p: &mut Parser) -> ParsedSyntax {
    if !is_at_contextual_keyword(p, "implements") {
        return Absent;
    }

    // test_err class_implements
    // class B implements C {}

    let m = p.start();
    expect_contextual_keyword(p, "implements", T![implements]);
    parse_ts_type_list(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, TS_IMPLEMENTS_CLAUSE))
}

fn parse_ts_type_list(p: &mut Parser) -> ParsedSyntax {
    parse_ts_name_with_type_arguments(p).map(|element| {
        let list = element.precede(p);

        while p.eat(T![,]) {
            parse_ts_name_with_type_arguments(p).or_add_diagnostic(p, expected_identifier);
        }

        list.complete(p, TS_TYPE_LIST)
    })
}

fn parse_ts_name_with_type_arguments(p: &mut Parser) -> ParsedSyntax {
    parse_ts_name(p).map(|name| {
        let m = name.precede(p);
        parse_ts_type_arguments(p).ok();
        m.complete(p, TS_NAME_WITH_TYPE_ARGUMENTS)
    })
}

pub(crate) fn try_parse(
    p: &mut Parser,
    func: impl FnOnce(&mut Parser) -> ParsedSyntax,
) -> ParsedSyntax {
    let checkpoint = p.checkpoint();

    let res = if p.state.no_recovery {
        func(p)
    } else {
        let last_no_recovery = std::mem::replace(&mut p.state.no_recovery, true);
        let res = func(p);
        p.state.no_recovery = last_no_recovery;
        res
    };

    if res.is_absent() {
        p.rewind(checkpoint);
    }
    res
}
