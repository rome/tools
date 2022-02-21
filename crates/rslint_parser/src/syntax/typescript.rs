//! TypeScript specific functions.

mod statement;
pub mod ts_parse_error;
mod types;

use crate::parser::{expected_token, expected_token_any};
use crate::syntax::expr::{parse_identifier, parse_unary_expr, ExpressionContext};
use crate::syntax::js_parse_error::{expected_expression, expected_ts_type};

use crate::syntax::util::{expect_contextual_keyword, is_at_contextual_keyword};
use crate::{JsSyntaxKind::*, *};
use rome_rowan::SyntaxKind;

pub(crate) use self::statement::*;
use self::ts_parse_error::ts_member_cannot_be;
pub(crate) use self::types::*;

use super::binding::parse_identifier_binding;
use super::class::is_nth_at_modifier;
use super::expr::is_nth_at_identifier;
use super::js_parse_error::expected_identifier;
use super::stmt::optional_semi;

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
// var d = <Error>({ name: "foo", message: "bar" });
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
    expect_ts_type_list(p, "implements");

    Present(m.complete(p, TS_IMPLEMENTS_CLAUSE))
}

fn expect_ts_type_list(p: &mut Parser, clause_name: &str) -> CompletedMarker {
    let list = p.start();

    if parse_ts_name_with_type_arguments(p).is_absent() {
        p.error(
            p.err_builder(&format!("'{}' list cannot be empty.", clause_name))
                .primary(p.cur_tok().start()..p.cur_tok().start(), ""),
        )
    }

    while p.at(T![,]) {
        let comma_range = p.cur_tok().range();
        p.bump(T![,]);
        // test_err ts ts_extends_trailing_comma
        // interface A {}
        // interface B extends A, {}
        if parse_ts_name_with_type_arguments(p).is_absent() {
            p.error(
                p.err_builder("Trailing comma not allowed.")
                    .primary(comma_range, ""),
            );
            break;
        }
    }

    list.complete(p, TS_TYPE_LIST)
}

fn parse_ts_name_with_type_arguments(p: &mut Parser) -> ParsedSyntax {
    parse_ts_name(p).map(|name| {
        let m = name.precede(p);
        parse_ts_type_arguments(p).ok();
        m.complete(p, TS_NAME_WITH_TYPE_ARGUMENTS)
    })
}

pub(crate) fn try_parse<T, E>(
    p: &mut Parser,
    func: impl FnOnce(&mut Parser) -> Result<T, E>,
) -> Result<T, E> {
    let checkpoint = p.checkpoint();

    let old_value = std::mem::replace(&mut p.state.speculative_parsing, true);
    let res = func(p);
    p.state.speculative_parsing = old_value;

    if res.is_err() {
        p.rewind(checkpoint);
    }

    res
}

/// Must be at `[ident:]` or `readonly [ident:]`
pub(crate) fn is_at_ts_index_signature_member(p: &Parser) -> bool {
    let mut offset = 0;

    while is_nth_at_modifier(p, offset) {
        offset += 1;
    }

    if !p.nth_at(offset, T!['[']) {
        return false;
    }

    if !is_nth_at_identifier(p, offset + 1) {
        return false;
    }

    p.nth_at(offset + 2, T![:])
}

bitflags::bitflags! {
    /// Flags describing possible members separators
    pub(crate) struct MembersSeparator: u8 {
        /// Members can be separated by ','
        const COMMA 		= 1 << 0;
        /// Members can be separated by ';'
        const SEMICOLON 	= 1 << 1;
    }
}

pub(crate) fn expect_ts_index_signature_member(
    p: &mut Parser,
    m: Marker,
    kind: JsSyntaxKind,
    possible_separators: MembersSeparator,
) -> CompletedMarker {
    while is_nth_at_modifier(p, 0) {
        if is_at_contextual_keyword(p, "readonly") {
            p.bump_remap(T![readonly]);
        } else {
            p.error(ts_member_cannot_be(
                p,
                p.cur_tok().range(),
                "index signature",
                p.cur_src(),
            ));
            p.bump_any();
        }
    }

    p.bump(T!['[']);

    let parameter = p.start();
    parse_identifier_binding(p).or_add_diagnostic(p, expected_identifier);
    parse_ts_type_annotation(p).unwrap(); // It's a computed member name if the type annotation is missing
    parameter.complete(p, TS_INDEX_SIGNATURE_PARAMETER);

    p.expect(T![']']);

    parse_ts_type_annotation(p).or_add_diagnostic(p, |p, range| {
        p.err_builder("An index signature must have a type annotation")
            .primary(range, "")
    });

    eat_members_separator(p, possible_separators);

    m.complete(p, kind)
}

fn eat_members_separator(p: &mut Parser, possible_separators: MembersSeparator) {
    let separator_eaten = possible_separators.contains(MembersSeparator::COMMA) && p.eat(T![,]);
    let separator_eaten = separator_eaten
        || (possible_separators.contains(MembersSeparator::SEMICOLON) && optional_semi(p));

    if !separator_eaten {
        let qty = possible_separators.bits().count_ones();

        if qty > 1 {
            let mut tokens = vec![];
            if possible_separators.contains(MembersSeparator::COMMA) {
                tokens.push(T![,]);
            }
            if possible_separators.contains(MembersSeparator::SEMICOLON) {
                tokens.push(T![;]);
            }

            p.error(expected_token_any(&tokens));
        } else {
            let token = if possible_separators.contains(MembersSeparator::COMMA) {
                T![,]
            } else {
                T![;]
            };
            p.error(expected_token(token));
        }
    }
}
