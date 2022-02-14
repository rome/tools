use crate::parser::RecoveryResult;
use crate::syntax::binding::{is_nth_at_identifier_binding, parse_binding};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::ExpressionContext;

use super::ts_parse_error::expected_ts_enum_member;
use crate::state::SignatureFlags;
use crate::syntax::auxiliary::{is_nth_at_declaration_clause, parse_declaration_clause};
use crate::syntax::function::{parse_function_body, parse_parameter_list, ParameterContext};
use crate::syntax::js_parse_error::{
    expected_binding, expected_identifier, expected_parameters, expected_ts_type,
};
use crate::syntax::stmt::{semi, STMT_RECOVERY_SET};
use crate::syntax::typescript::{
    expect_ts_type_list, parse_ts_identifier_binding, parse_ts_implements_clause,
    parse_ts_return_type_annotation, parse_ts_type, parse_ts_type_parameters, TypeMembers,
};
use crate::syntax::util::{expect_contextual_keyword, is_at_contextual_keyword};
use crate::{JsSyntaxKind::*, *};

fn parse_literal_as_ts_enum_member(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        JS_STRING_LITERAL | T![ident] => {
            p.bump_any();
        }
        t if t.is_keyword() => {
            p.bump_remap(T![ident]);
        }
        JS_NUMBER_LITERAL => {
            let err = p
                .err_builder("An enum member cannot have a numeric name")
                .primary(p.cur_tok().range(), "");
            p.error(err);
            p.bump_any()
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    }
    Present(m.complete(p, JS_LITERAL_MEMBER_NAME))
}

/// An individual enum member
fn parse_ts_enum_member(p: &mut Parser) -> ParsedSyntax {
    let member = p.start();

    let name = match p.cur() {
        T!['['] => syntax::object::parse_computed_member_name(p),
        T![#] => {
            let err = p
                .err_builder("An `enum` member cannot be private")
                .primary(p.cur_tok().range(), "");
            p.error(err);
            syntax::class::parse_private_class_member_name(p).map(|mut x| {
                x.change_to_unknown(p);
                x
            })
        }
        _ => parse_literal_as_ts_enum_member(p),
    };

    if name.is_absent() {
        member.abandon(p);
        return Absent;
    }

    let _ = parse_initializer_clause(p, ExpressionContext::default());

    Present(member.complete(p, TS_ENUM_MEMBER))
}
struct TsEnumMembersList;

impl ParseSeparatedList for TsEnumMembersList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_enum_member(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN_MEMBER,
                STMT_RECOVERY_SET.union(token_set![JsSyntaxKind::IDENT, T![,], T!['}']]),
            )
            .enable_recovery_on_line_break(),
            expected_ts_enum_member,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        TS_ENUM_MEMBER_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline(always)]
fn is_reserved_enum_name(name: &str) -> bool {
    super::is_reserved_type_name(name)
}

fn parse_ts_enum_id(p: &mut Parser, enum_token_range: Range<usize>) {
    match parse_binding(p) {
        Present(id) => {
            let text = p.span_text(id.range(p));
            if is_reserved_enum_name(text) {
                let err = p
                    .err_builder(&format!(
                        "`{}` cannot be used as a enum name because it is already reserved",
                        text
                    ))
                    .primary(id.range(p), "");

                p.error(err);
            }
        }
        // test_err ts enum_decl_no_id
        // enum {A,B,C}
        // enum 1 {A,B,C}
        Absent => {
            if p.nth_at(1, L_CURLY) {
                let range = p.cur_tok().range();

                let m = p.start();
                p.bump_any();
                let _ = m.complete(p, JS_UNKNOWN_BINDING);

                let err = p.err_builder("invalid `enum` name").primary(range, "");
                p.error(err);
            } else {
                let err = p
                    .err_builder("`enum` statements must have a name")
                    .primary(enum_token_range.start..p.cur_tok().start(), "");
                p.error(err);
            }
        }
    }
}

pub(crate) fn is_at_ts_enum_declaration(p: &Parser) -> bool {
    is_nth_at_ts_enum_declaration(p, 0)
}

pub(crate) fn is_nth_at_ts_enum_declaration(p: &Parser, n: usize) -> bool {
    match p.nth(n) {
        T![enum] => true,
        T![const] => p.nth_at(n + 1, T![enum]),
        _ => false,
    }
}

// test ts typescript_enum
// enum A {}
// enum B { a, b, c }
// const enum C { A = 1, B = A * 2, ["A"] = 3, }
pub(crate) fn parse_ts_enum_declaration(p: &mut Parser) -> ParsedSyntax {
    if !is_at_ts_enum_declaration(p) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![const]);

    let enum_token_range = p.cur_tok().range();
    p.expect(T![enum]);
    parse_ts_enum_id(p, enum_token_range);

    // test_err ts enum_no_l_curly
    // enum;
    // enum A;
    p.expect(T!['{']);
    TsEnumMembersList.parse_list(p);

    // test_err ts enum_no_r_curly
    // enum {;
    // enum A {;
    p.expect(T!['}']);

    Present(m.complete(p, TS_ENUM_DECLARATION))
}

pub(crate) fn parse_ts_type_alias_declaration(p: &mut Parser) -> ParsedSyntax {
    if !is_at_contextual_keyword(p, "type") {
        return Absent;
    }

    let start = p.cur_tok().range().start;
    let m = p.start();
    expect_contextual_keyword(p, "type", T![type]);
    parse_ts_identifier_binding(p).or_add_diagnostic(p, expected_identifier);
    parse_ts_type_parameters(p).ok();
    p.expect(T![=]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);

    semi(p, start..p.cur_tok().range().end);

    Present(m.complete(p, TS_TYPE_ALIAS_DECLARATION))
}

pub(crate) fn parse_ts_declare_statement(p: &mut Parser) -> ParsedSyntax {
    if !is_at_ts_declare_statement(p) {
        return Absent;
    }

    let m = p.start();
    expect_contextual_keyword(p, "declare", T![declare]);

    parse_declaration_clause(p, true)
        .expect("Expected a declaration as guaranteed by is_at_ts_declare_statement");

    Present(m.complete(p, TS_DECLARE_STATEMENT))
}

pub(crate) fn is_at_ts_declare_statement(p: &Parser) -> bool {
    if !is_at_contextual_keyword(p, "declare") || p.has_linebreak_before_n(1) {
        return false;
    }

    is_nth_at_declaration_clause(p, 1)
}

// test ts ts_declare_function
// declare function test<A, B, R>(a: A, b: B): R;
// declare function test2({ a }?: { a: "string" })
// declare
// function not_a_declaration() {}
//
// test_err ts ts_declare_function_with_body
// declare function test<A>(a: A): string { return "ambient function with a body"; }
pub(crate) fn parse_ts_declare_function_declaration(p: &mut Parser) -> ParsedSyntax {
    let is_async = is_at_contextual_keyword(p, "async");

    if !is_async && !p.at(T![function]) {
        return Absent;
    }

    let m = p.start();
    let stmt_start = p.cur_tok().start();

    // test_err ts ts_declare_async_function
    // declare async function test();
    if is_async {
        p.error(
            p.err_builder("'async' modifier cannot be used in an ambient context.")
                .primary(p.cur_tok().range(), ""),
        );
        p.bump_remap(T![async]);
    }

    p.expect(T![function]);
    parse_binding(p).or_add_diagnostic(p, expected_binding);
    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    parse_ts_return_type_annotation(p).ok();

    if let Present(body) = parse_function_body(p, SignatureFlags::empty()) {
        p.error(
            p.err_builder("A 'declare' function cannot have a function body")
                .primary(body.range(p), "remove this body"),
        );
    }

    semi(p, stmt_start..p.cur_tok().start());

    Present(if is_async {
        m.complete(p, JS_UNKNOWN_STATEMENT)
    } else {
        m.complete(p, TS_DECLARE_FUNCTION_DECLARATION)
    })
}

pub(crate) fn is_at_ts_interface_declaration(p: &Parser) -> bool {
    if !is_at_contextual_keyword(p, "interface") || p.has_linebreak_before_n(1) {
        return false;
    }

    is_nth_at_identifier_binding(p, 1) || p.nth_at(1, T!['{'])
}

// test ts ts_interface
// interface A {}
// interface B { prop: string, method(): string, [index: number]: string, new(): B }
pub(crate) fn parse_ts_interface_declaration(p: &mut Parser) -> ParsedSyntax {
    if !is_at_ts_interface_declaration(p) {
        return Absent;
    }

    let m = p.start();
    expect_contextual_keyword(p, "interface", T![interface]);
    parse_ts_identifier_binding(p).or_add_diagnostic(p, expected_identifier);
    parse_ts_type_parameters(p).ok();
    eat_interface_heritage_clause(p);
    p.expect(T!['{']);
    TypeMembers.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, TS_INTERFACE_DECLARATION))
}

// test_err ts ts_interface_heritage_clause_error
// interface A {}
// interface B implements A {}
// interface C extends A extends B {}
// interface D extends {}
// interface E extends A, {}
/// Eats an interface's `extends` or an `extends` (not allowed but for better recovery) clauses
/// Attaches the clauses to the currently active node
fn eat_interface_heritage_clause(p: &mut Parser) {
    let mut first_extends: Option<CompletedMarker> = None;
    loop {
        if p.at(T![extends]) {
            let extends = parse_ts_extends_clause(p).expect(
                "expected an extends clause because parser is positioned at the extends keyword",
            );

            if let Some(first_extends) = first_extends.as_ref() {
                p.error(
                    p.err_builder("'extends' clause already seen.")
                        .primary(extends.range(p), "")
                        .secondary(first_extends.range(p), "first 'extends' clause"),
                )
            } else {
                first_extends = Some(extends);
            }
        } else if is_at_contextual_keyword(p, "implements") {
            let implements =
                parse_ts_implements_clause(p).expect("positioned at the implements keyword");
            p.error(
                p.err_builder("Interface declaration cannot have 'implements' clause.")
                    .primary(implements.range(p), ""),
            );
        } else {
            break;
        }
    }
}

// test ts ts_interface_extends_clause
// interface A<Prop> { prop: Prop }
// interface B extends A<string> {}
// interface C extends A<number>, B {}
fn parse_ts_extends_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![extends]);
    expect_ts_type_list(p, "extends");
    Present(m.complete(p, TS_EXTENDS_CLAUSE))
}
