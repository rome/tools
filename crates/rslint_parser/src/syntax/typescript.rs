//! TypeScript specific functions.

mod statement;
pub mod ts_parse_error;
mod types;

use crate::parser::expected_token_any;
use crate::syntax::expr::{parse_identifier, parse_unary_expr, ExpressionContext};
use crate::syntax::js_parse_error::expected_expression;

use crate::syntax::typescript::ts_parse_error::expected_ts_type;
use crate::{Absent, CompletedMarker, Marker, ParsedSyntax, Parser, Present};
use rome_js_syntax::{JsSyntaxKind::*, *};
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
    if !p.at(T![implements]) {
        return Absent;
    }

    // test_err class_implements
    // class B implements C {}

    let m = p.start();
    p.expect(T![implements]);
    expect_ts_type_list(p, "implements");

    Present(m.complete(p, TS_IMPLEMENTS_CLAUSE))
}

fn expect_ts_type_list(p: &mut Parser, clause_name: &str) -> CompletedMarker {
    let list = p.start();

    if parse_ts_name_with_type_arguments(p).is_absent() {
        p.error(
            p.err_builder(&format!("'{}' list cannot be empty.", clause_name))
                .primary(p.cur_range().start()..p.cur_range().start(), ""),
        )
    }

    while p.at(T![,]) {
        let comma_range = p.cur_range();
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

        if !p.has_preceding_line_break() {
            parse_ts_type_arguments(p).ok();
        }

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

/// Must be at `[ident:` or `<modifiers> [ident:`
pub(crate) fn is_at_ts_index_signature_member(p: &mut Parser) -> bool {
    let mut offset = 0;
    while is_nth_at_modifier(p, offset, false) {
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

#[derive(Clone, Copy)]
pub(crate) enum MemberParent {
    Class,
    TypeOrInterface,
}

pub(crate) fn expect_ts_index_signature_member(
    p: &mut Parser,
    m: Marker,
    parent: MemberParent,
) -> CompletedMarker {
    while is_nth_at_modifier(p, 0, false) {
        if p.eat(T![readonly]) {
            continue;
        } else {
            p.error(ts_member_cannot_be(
                p,
                p.cur_range(),
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

    eat_members_separator(p, parent);

    m.complete(
        p,
        match parent {
            MemberParent::Class => TS_INDEX_SIGNATURE_CLASS_MEMBER,
            MemberParent::TypeOrInterface => TS_INDEX_SIGNATURE_TYPE_MEMBER,
        },
    )
}

fn eat_members_separator(p: &mut Parser, parent: MemberParent) {
    let (comma, semi_colon) = match parent {
        MemberParent::Class => (false, true),
        MemberParent::TypeOrInterface => (true, true),
    };
    debug_assert!(comma || semi_colon);

    let separator_eaten = comma && p.eat(T![,]);
    let separator_eaten = separator_eaten || (semi_colon && optional_semi(p));

    if !separator_eaten {
        if semi_colon {
            let err = p.err_builder("';' expected'").primary(
                p.cur_range(),
                "An explicit or implicit semicolon is expected here...",
            );
            p.error(err);
        } else {
            let mut tokens = vec![];
            if comma {
                tokens.push(T![,]);
            }
            if semi_colon {
                tokens.push(T![;]);
            }
            p.error(expected_token_any(&tokens));
        }
    }
}
