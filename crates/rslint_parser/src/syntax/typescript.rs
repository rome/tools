//! TypeScript specific functions.

mod types;
mod enums;

use super::expr::{parse_assignment_expression_or_higher, parse_lhs_expr, parse_name};
use crate::parser::ParserProgress;
#[allow(deprecated)]
use crate::parser::SingleTokenParseRecovery;
use crate::syntax::expr::{parse_identifier, parse_unary_expr, ExpressionContext};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{expected_expression, expected_identifier, expected_ts_type};

use crate::{JsSyntaxKind::*, *};
use rome_rowan::SyntaxKind;

pub(crate) use self::types::*;
pub(crate) use self::enums::*;

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

// test ts_type_assertion_expression
// // TYPESCRIPT
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

// FIXME: ts allows trailing commas but this doesnt, we need to figure out a way
// to peek at the next token and see if its the end of the heritage clause
pub(crate) fn ts_heritage_clause(p: &mut Parser, exprs: bool) -> Vec<CompletedMarker> {
    let mut elems = Vec::with_capacity(1);
    let m = p.start();
    if exprs {
        parse_lhs_expr(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression);
    } else {
        parse_ts_name(p).or_add_diagnostic(p, expected_identifier);
    }

    parse_ts_type_arguments(p).ok();

    // it doesnt matter if we complete as ts_expr_with_type_args even if its an lhs expr
    // because exprs: true will only be used with `class extends foo, bar`, in which case
    // the first expr will be "unwrapped" to go to the class' node and the rest are errors
    elems.push(m.complete(p, TS_EXPR_WITH_TYPE_ARGS));

    let mut progress = ParserProgress::default();
    while p.eat(T![,]) {
        progress.assert_progressing(p);
        let m = p.start();
        if exprs {
            parse_lhs_expr(p, ExpressionContext::default())
                .or_add_diagnostic(p, js_parse_error::expected_expression);
        } else {
            parse_ts_name(p).or_add_diagnostic(p, expected_identifier);
        }

        parse_ts_type_arguments(p).ok();

        elems.push(m.complete(p, TS_EXPR_WITH_TYPE_ARGS));
    }
    elems
}

pub(super) fn ts_enum(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.eat(T![const]);
    p.expect(T![enum]);
    parse_name(p).or_add_diagnostic(p, js_parse_error::expected_identifier);
    p.expect(T!['{']);
    let mut first = true;

    let members_list = p.start();
    let mut progress = ParserProgress::default();
    while !p.at(EOF) && !p.at(T!['}']) {
        progress.assert_progressing(p);
        if first {
            first = false;
        } else if p.at(T![,]) && p.nth_at(1, T!['}']) {
            p.eat(T![,]);
            break;
        } else {
            p.expect(T![,]);
        }

        let member = p.start();
        let err_occured = if !p.at_ts(token_set![T![ident], T![yield], T![await]])
            && !p.cur().is_keyword()
            && !p.at(JS_STRING_LITERAL)
        {
            let err = p
                .err_builder("expected an identifier or string for an enum variant, but found none")
                .primary(p.cur_tok().range(), "");

            #[allow(deprecated)]
            SingleTokenParseRecovery::with_error(
                token_set![T!['}'], T![ident], T![yield], T![await], T![=], T![,]],
                JS_UNKNOWN,
                err,
            )
            .recover(p);
            true
        } else {
            if !p.eat(JS_STRING_LITERAL) {
                parse_name(p).unwrap().undo_completion(p).abandon(p);
            }
            false
        };

        if p.eat(T![=]) {
            parse_assignment_expression_or_higher(p, ExpressionContext::default())
                .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
            member.complete(p, TS_ENUM_MEMBER);
        } else if err_occured {
            member.abandon(p);
        } else {
            member.complete(p, TS_ENUM_MEMBER);
        }
    }

    members_list.complete(p, TS_ENUM_MEMBER_LIST);

    p.expect(T!['}']);
    m.complete(p, TS_ENUM)
}

pub fn try_parse(p: &mut Parser, func: impl FnOnce(&mut Parser) -> ParsedSyntax) -> ParsedSyntax {
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
