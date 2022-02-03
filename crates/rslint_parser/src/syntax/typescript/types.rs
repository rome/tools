use crate::parser::RecoveryResult;
use crate::state::{EnterType, SignatureFlags};
use crate::syntax::binding::parse_identifier_binding;
use crate::syntax::expr::{
    is_at_identifier, is_nth_at_identifier, is_nth_at_identifier_or_keyword,
    parse_big_int_literal_expression, parse_identifier, parse_literal_expression, parse_name,
    parse_number_literal_expression, parse_reference_identifier, ExpressionContext,
};
use crate::syntax::function::{
    parse_any_formal_parameter, parse_parameter_list, skip_parameter_start, ParameterContext,
};
use crate::syntax::js_parse_error::{
    expected_identifier, expected_object_member_name, expected_parameter, expected_parameters,
    expected_property_or_signature, expected_ts_type, expected_ts_type_parameter,
};
use crate::syntax::object::{
    is_at_object_member_name, is_nth_at_object_member_name, parse_object_member_name,
};
use crate::syntax::stmt::{optional_semi, semi};
use crate::syntax::typescript::{parse_ts_identifier_binding, try_parse};
use crate::syntax::util::{
    eat_contextual_keyword, expect_contextual_keyword, is_at_contextual_keyword,
    is_nth_at_contextual_keyword,
};
use crate::JsSyntaxFeature::TypeScript;
use crate::{Absent, ParsedSyntax, Parser};
use crate::{JsSyntaxKind::*, *};
use rslint_syntax::T;

pub(crate) fn is_reserved_type_name(name: &str) -> bool {
    matches!(
        name,
        "string"
            | "null"
            | "number"
            | "object"
            | "any"
            | "unknown"
            | "boolean"
            | "bigint"
            | "symbol"
            | "void"
            | "never"
    )
}

pub(crate) fn parse_ts_type_annotation(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![:]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_TYPE_ANNOTATION))
}

// test ts_return_type_annotation
// // TYPESCRIPT
// type A = (a) => a is string;
// type B = { test(a): a is string }
// type C = { (a): a is string }
// const a = { test(x): x is string { return typeof x === "string" } }
// class D { test(x): x is string { return typeof x === "string"; } }
pub(crate) fn parse_ts_return_type_annotation(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![:]);
    parse_ts_return_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_RETURN_TYPE_ANNOTATION))
}

fn parse_ts_call_signature(p: &mut Parser) {
    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    parse_ts_return_type_annotation(p).ok();
}

fn parse_ts_type_parameter_name(p: &mut Parser) -> ParsedSyntax {
    parse_identifier(p, TS_TYPE_PARAMETER_NAME)
}

// test ts_type_parameters
// // TYPESCRIPT
// type A<X extends string, Y = number, Z extends string | number = number> = { x: X, y: Y, z: Z }
pub(crate) fn parse_ts_type_parameters(p: &mut Parser) -> ParsedSyntax {
    if !is_nth_at_ts_type_parameters(p, 0) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);
    TsTypeParameterList.parse_list(p);
    p.expect(T![>]);

    Present(m.complete(p, TS_TYPE_PARAMETERS))
}

struct TsTypeParameterList;

impl ParseSeparatedList for TsTypeParameterList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_type_parameter(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T![>])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN,
                token_set![T![>], T![,], T![ident], T![yield], T![await]],
            )
            .enable_recovery_on_line_break(),
            expected_ts_type_parameter,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        TS_TYPE_PARAMETER_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_ts_type_parameter(p: &mut Parser) -> ParsedSyntax {
    parse_ts_type_parameter_name(p).map(|name| {
        let m = name.precede(p);
        parse_ts_type_constraint_clause(p).ok();
        parse_ts_default_type_clause(p).ok();
        m.complete(p, TS_TYPE_PARAMETER)
    })
}

// test ts_type_constraint_clause
// // TYPESCRIPT
// type A<X extends number> = X;
// type B<X extends number | string> = { a: X }
fn parse_ts_type_constraint_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![extends]);

    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_TYPE_CONSTRAINT_CLAUSE))
}

// test ts_default_type_clause
// // TYPESCRIPT
// type A<X=string> = X;
// type B<X extends number | string = string> = { a: X }
fn parse_ts_default_type_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![=]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_DEFAULT_TYPE_CLAUSE))
}

fn is_nth_at_ts_type_parameters(p: &Parser, n: usize) -> bool {
    p.nth_at(n, T![<])
}

pub(crate) fn parse_ts_type_alias(p: &mut Parser) -> ParsedSyntax {
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

    Present(m.complete(p, TS_TYPE_ALIAS_STATEMENT))
}

pub(crate) fn parse_ts_type(p: &mut Parser) -> ParsedSyntax {
    p.with_state(EnterType, |p| {
        if is_at_constructor_type(p) {
            return parse_ts_constructor_type(p);
        }

        if is_at_function_type(p) {
            return parse_ts_function_type(p);
        }

        parse_ts_union_type_or_higher(p).map(|left| {
            // test ts_conditional_type
            // // TYPESCRIPT
            // type A = number;
            // type B = string extends number ? string : number;
            // type C = A extends (B extends A ? number : string) ? void : number;
            if !p.has_linebreak_before_n(0) && p.at(T![extends]) {
                let m = left.precede(p);
                p.bump(T![extends]);
                parse_ts_union_type_or_higher(p).or_add_diagnostic(p, expected_ts_type);
                p.expect(T![?]);
                parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
                p.expect(T![:]);
                parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
                m.complete(p, TS_CONDITIONAL_TYPE)
            } else {
                left
            }
        })
    })
}

// test ts_union_type
// // TYPESCRIPT
// type A = string | number;
// type B = | A | void | null;
// type C = A & C | C;
fn parse_ts_union_type_or_higher(p: &mut Parser) -> ParsedSyntax {
    parse_ts_union_or_intersection_type(p, IntersectionOrUnionType::Union)
}

// test ts_intersection_type
// // TYPESCRIPT
// type A = string & number;
// type B = & A & void & null;
fn parse_ts_intersection_type_or_higher(p: &mut Parser) -> ParsedSyntax {
    parse_ts_union_or_intersection_type(p, IntersectionOrUnionType::Intersection)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum IntersectionOrUnionType {
    Intersection,
    Union,
}

impl IntersectionOrUnionType {
    #[inline]
    fn operator(&self) -> JsSyntaxKind {
        match self {
            IntersectionOrUnionType::Union => T![|],
            IntersectionOrUnionType::Intersection => T![&],
        }
    }

    #[inline]
    fn list_kind(&self) -> JsSyntaxKind {
        match self {
            IntersectionOrUnionType::Union => TS_UNION_TYPE_VARIANT_LIST,
            IntersectionOrUnionType::Intersection => TS_INTERSECTION_TYPE_ELEMENT_LIST,
        }
    }

    #[inline]
    fn kind(&self) -> JsSyntaxKind {
        match self {
            IntersectionOrUnionType::Union => TS_UNION_TYPE,
            IntersectionOrUnionType::Intersection => TS_INTERSECTION_TYPE,
        }
    }

    #[inline]
    fn parse_element(&self, p: &mut Parser) -> ParsedSyntax {
        match self {
            IntersectionOrUnionType::Union => parse_ts_intersection_type_or_higher(p),
            IntersectionOrUnionType::Intersection => parse_ts_primary_type(p),
        }
    }
}

#[inline]
fn parse_ts_union_or_intersection_type(
    p: &mut Parser,
    ty_kind: IntersectionOrUnionType,
) -> ParsedSyntax {
    let m = p.start();

    let has_leading = p.at(ty_kind.operator());
    if has_leading {
        p.bump(ty_kind.operator());
    };

    let list = p.start();
    let first = ty_kind.parse_element(p);

    if has_leading || p.at(ty_kind.operator()) {
        first.or_add_diagnostic(p, expected_ts_type);

        while p.at(ty_kind.operator()) {
            p.bump(ty_kind.operator());

            ty_kind
                .parse_element(p)
                .or_add_diagnostic(p, expected_ts_type);
        }

        list.complete(p, ty_kind.list_kind());

        Present(m.complete(p, ty_kind.kind()))
    } else {
        list.abandon(p);
        m.abandon(p);
        first
    }
}

fn parse_ts_primary_type(p: &mut Parser) -> ParsedSyntax {
    if p.at(T![ident]) {
        // test ts_inferred_type
        // // TYPESCRIPT
        // type A = infer B;
        // type B = { a: infer U; b: infer U};
        if p.cur_src() == "infer" {
            let m = p.start();
            p.bump_remap(T![infer]);
            parse_ts_type_parameter_name(p).or_add_diagnostic(p, expected_identifier);
            return Present(m.complete(p, TS_INFER_TYPE));
        }

        // test ts_type_operator
        // // TYPESCRIPT
        // type A = { x: string, y: number };
        // type B = keyof A;
        // type C = readonly string[];
        // const d: unique symbol = Symbol();
        let type_operator_kind = match p.cur_src() {
            "unique" => Some(UNIQUE_KW),
            "keyof" => Some(KEYOF_KW),
            "readonly" => Some(READONLY_KW),
            _ => None,
        };

        if let Some(type_operator_kind) = type_operator_kind {
            let m = p.start();
            p.bump_remap(type_operator_kind);
            parse_ts_primary_type(p).or_add_diagnostic(p, expected_ts_type);
            return Present(m.complete(p, TS_TYPE_OPERATOR_TYPE));
        }
    }

    parse_postfix_type_or_higher(p)
}

fn parse_postfix_type_or_higher(p: &mut Parser) -> ParsedSyntax {
    parse_ts_non_array_type(p).map(|primary_type| {
        let mut left = primary_type;

        while p.at(T!['[']) && !p.has_linebreak_before_n(0) {
            let m = left.precede(p);
            p.bump(T!['[']);

            left = if parse_ts_type(p).is_present() {
                // test ts_indexed_access_type
                // // TYPESCRIPT
                // type A = string[number];
                // type B = string[number][number][number][];
                p.expect(T![']']);
                m.complete(p, TS_INDEXED_ACCESS_TYPE)
            } else {
                // test ts_array_type
                // // TYPESCRIPT
                // type A = string[];
                // type B = { a: number } [];
                p.expect(T![']']);
                m.complete(p, TS_ARRAY_TYPE)
            }
        }

        left
    })
}

fn parse_ts_non_array_type(p: &mut Parser) -> ParsedSyntax {
    // test ts_predefined_type
    // // TYPESCRIPT
    // type A = any
    // type B = number;
    // type C = object;
    // type D = boolean;
    // type E = bigint;
    // type F = string;
    // type G = symbol;
    // type H = void;
    // type I = undefined;
    // type J = null;
    // type K = never
    match p.cur() {
        T!['('] => parse_ts_parenthesized_type(p),
        T!['{'] => {
            if is_at_start_of_mapped_type(p) {
                parse_ts_mapped_type(p)
            } else {
                parse_ts_object_type(p)
            }
        }
        T!['['] => parse_ts_tuple_type(p),
        T![void] => {
            let m = p.start();
            p.bump(T![void]);
            Present(m.complete(p, TS_VOID_TYPE))
        }
        JS_NUMBER_LITERAL | JS_STRING_LITERAL | TRUE_KW | FALSE_KW | T![null] => {
            parse_ts_literal_type(p)
        }
        BACKTICK => parse_ts_template_literal_type(p),
        T![-] if p.nth_at(1, JS_NUMBER_LITERAL) => parse_ts_literal_type(p),
        T![this] => parse_ts_this_type(p),
        T![typeof] => {
            if p.nth_at(1, T![import]) {
                parse_ts_import_type(p)
            } else {
                parse_ts_typeof_type(p)
            }
        }
        T![import] => parse_ts_import_type(p),
        T![ident] if !p.nth_at(1, T![.]) => {
            let (token_kind, node_kind) = match p.cur_src() {
                "any" => (T![any], TS_ANY_TYPE),
                "unknown" => (T![unknown], TS_UNKNOWN_TYPE),
                "number" => (T![number], TS_NUMBER_TYPE),
                "object" => (T![object], TS_NON_PRIMITIVE_TYPE),
                "boolean" => (T![boolean], TS_BOOLEAN_TYPE),
                "bigint" => (T![bigint], TS_BIGINT_TYPE),
                "string" => (T![string], TS_STRING_TYPE),
                "symbol" => (T![symbol], TS_SYMBOL_TYPE),
                "undefined" => (T![undefined], TS_UNDEFINED_TYPE),
                "never" => (T![never], TS_NEVER_TYPE),
                _ => {
                    return parse_ts_reference_type(p);
                }
            };

            let m = p.start();
            p.bump_remap(token_kind);
            Present(m.complete(p, node_kind))
        }
        T![ident] => parse_ts_reference_type(p),
        _ => parse_ts_reference_type(p),
    }
}

// test ts_reference_type
// // TYPESCRIPT
// type A = object;
// type B = string;
// type C = A;
// type D = B.a;
// type E = D.c.b.a;
fn parse_ts_reference_type(p: &mut Parser) -> ParsedSyntax {
    parse_ts_name(p).map(|name| {
        let m = name.precede(p);

        if !p.has_linebreak_before_n(0) && p.at(T![<]) {
            parse_ts_type_arguments(p).ok();
        }

        m.complete(p, TS_REFERENCE_TYPE)
    })
}

pub(crate) fn parse_ts_name(p: &mut Parser) -> ParsedSyntax {
    let mut left = if p.cur().is_keyword() {
        let m = p.start();
        p.bump_remap(T![ident]);
        Present(m.complete(p, JS_REFERENCE_IDENTIFIER))
    } else {
        parse_reference_identifier(p)
    };

    while p.at(T![.]) {
        let m = left.precede_or_add_diagnostic(p, expected_identifier);
        p.bump(T![.]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        left = Present(m.complete(p, TS_QUALIFIED_NAME));
    }

    left
}

// test ts_typeof_type
// // TYPESCRIPT
// let a = "test";
// type B = typeof a;
fn parse_ts_typeof_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![typeof]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![typeof]);
    parse_ts_name(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, TS_TYPEOF_TYPE))
}

// test ts_this_type
// // TYPESCRIPT
// class A {
//     method() {
//         type A = this;
//     }
//     predicate(): this is string {
//         return typeof this === "string"
//     }
// }
fn parse_ts_this_type(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![this]);
    Present(m.complete(p, TS_THIS_TYPE))
}

// test ts_parenthesized_type
// // TYPESCRIPT
// type A = (string)
fn parse_ts_parenthesized_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    p.expect(T![')']);
    Present(m.complete(p, TS_PARENTHESIZED_TYPE))
}

fn is_at_start_of_mapped_type(p: &Parser) -> bool {
    if !p.at(T!['{']) {
        return false;
    }

    if p.nth_at(1, T![+]) || p.nth_at(1, T![-]) {
        return is_nth_at_contextual_keyword(p, 2, "readonly");
    }

    let mut offset = 1;

    if is_nth_at_contextual_keyword(p, offset, "readonly") {
        offset += 1;
    }

    p.nth_at(offset, T!['['])
        && (is_nth_at_identifier(p, offset + 1) || p.nth(offset + 1).is_keyword())
        && p.nth_at(offset + 2, T![in])
}

// test ts_mapped_type
// // TYPESCRIPT
// type A = { [test in "a" | "b"] }
// type OptionsFlags<Type> = {
//   [Property in keyof Type]: boolean;
// };
// type CreateMutable<Type> = {
// 	-readonly [Property in keyof Type]: Type[Property];
// };
// type Concrete<Type> = {
//   [Property in keyof Type]-?: Type[Property]
// };
// type Getters<Type> = {
//     [Property in keyof Type as `get${Capitalize<string & Property>}`]: () => Type[Property]
// };
fn parse_ts_mapped_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    parse_ts_mapped_type_readonly_modifier_clause(p).ok();
    p.expect(T!['[']);
    parse_ts_type_parameter_name(p).or_add_diagnostic(p, expected_ts_type_parameter);
    p.expect(T![in]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    parse_ts_mapped_type_as_clause(p).ok();
    p.expect(T![']']);
    parse_ts_mapped_type_optional_modifier_clause(p).ok();
    parse_ts_type_annotation(p).ok();
    p.eat(T![;]);
    p.expect(T!['}']);

    Present(m.complete(p, TS_MAPPED_TYPE))
}

fn parse_ts_mapped_type_as_clause(p: &mut Parser) -> ParsedSyntax {
    if !is_at_contextual_keyword(p, "as") {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![as]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_MAPPED_TYPE_AS_CLAUSE))
}

fn parse_ts_mapped_type_readonly_modifier_clause(p: &mut Parser) -> ParsedSyntax {
    if is_at_contextual_keyword(p, "readonly") {
        let m = p.start();
        p.bump_remap(T![readonly]);
        Present(m.complete(p, TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE))
    } else if p.at(T![+]) || p.at(T![-]) {
        let m = p.start();
        p.bump_any();
        expect_contextual_keyword(p, "readonly", T![readonly]);
        Present(m.complete(p, TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE))
    } else {
        Absent
    }
}

fn parse_ts_mapped_type_optional_modifier_clause(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        T![?] => {
            let m = p.start();
            p.bump(T![?]);
            Present(m.complete(p, TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE))
        }
        T![-] | T![+] => {
            let m = p.start();
            p.bump_any();
            p.expect(T![?]);
            Present(m.complete(p, TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE))
        }
        _ => Absent,
    }
}

// test ts_import_type
// // TYPESCRIPT
// type A = typeof import("test");
// type B = import("test");
// type C = typeof import("test").a.b.c.d.e.f;
fn parse_ts_import_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![typeof]) && !p.at(T![import]) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![typeof]);
    p.expect(T![import]);
    p.expect(T!['(']);
    p.expect(JS_STRING_LITERAL);
    p.expect(T![')']);

    if p.at(T![.]) {
        let qualifier = p.start();
        p.bump(T![.]);
        parse_ts_name(p).or_add_diagnostic(p, expected_identifier);
        qualifier.complete(p, TS_IMPORT_TYPE_QUALIFIER);
    }

    Present(m.complete(p, TS_IMPORT_TYPE))
}

// test ts_object_type
// // TYPESCRIPT
// type A = { a: string, b: number };
// type B = { a: string; b: number };
// type C = { a: string, b: number; c: string };
// type D = {
// 	a: string
//  b: number
// }
fn parse_ts_object_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    ObjectTypeMembers.parse_list(p);
    p.expect(T!['}']);
    Present(m.complete(p, TS_OBJECT_TYPE))
}

struct ObjectTypeMembers;

impl ParseNodeList for ObjectTypeMembers {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_object_type_member(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, member: ParsedSyntax) -> RecoveryResult {
        member.or_recover(
            p,
            &ParseRecovery::new(JS_UNKNOWN, token_set![T!['}'], T![,], T![;]])
                .enable_recovery_on_line_break(),
            expected_property_or_signature,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        TS_OBJECT_TYPE_MEMBER_LIST
    }
}

fn parse_ts_object_type_member(p: &mut Parser) -> ParsedSyntax {
    if is_at_ts_index_signature_object_type_member(p) {
        return parse_ts_index_signature_object_type_member(p);
    }

    match p.cur() {
        T!['('] | T![<] => parse_ts_call_signature_object_type_member(p),
        T![new] if is_at_ts_construct_signature_object_type_member(p) => {
            parse_ts_construct_signature_object_type_member(p)
        }
        T![ident] if p.cur_src() == "get" && is_nth_at_object_member_name(p, 1) => {
            parse_ts_getter_signature_object_type_member(p)
        }
        T![ident] if p.cur_src() == "set" && is_nth_at_object_member_name(p, 1) => {
            parse_ts_setter_signature_object_type_member(p)
        }
        _ => parse_ts_property_or_method_signature_object_type_member(p),
    }
}

fn parse_ts_object_type_member_semi(p: &mut Parser) {
    // object type members can either be separated by a comma
    if p.eat(T![,]) {
        return;
    }

    // or a semicolon (possibly ASI)
    if !optional_semi(p) {
        let err = p.err_builder("';' expected'").primary(
            p.cur_tok().range(),
            "An explicit or implicit semicolon is expected here...",
        );

        p.error(err);
    }
}

// test ts_property_or_method_signature_member
// // TYPESCRIPT
// type A = { a?: string; b?(): number }
// type B = { a: string, b(): number }
// type C = { m(a: string, b: number, c: string): any }
// type D = { readonly: string, readonly a: number }
// type E = { m<A, B>(a: A, b: B): never }
fn parse_ts_property_or_method_signature_object_type_member(p: &mut Parser) -> ParsedSyntax {
    if !is_at_object_member_name(p) {
        return Absent;
    }

    let m = p.start();
    let readonly_range =
        if p.at(T![ident]) && p.cur_src() == "readonly" && is_nth_at_object_member_name(p, 1) {
            let range = p.cur_tok().range();
            p.bump_remap(T![readonly]);
            Some(range)
        } else {
            None
        };

    parse_object_member_name(p).unwrap();

    p.eat(T![?]);

    if p.at(T!['(']) || p.at(T![<]) {
        parse_ts_call_signature(p);
        parse_ts_object_type_member_semi(p);
        let method = m.complete(p, TS_METHOD_SIGNATURE_OBJECT_TYPE_MEMBER);

        if let Some(readonly_range) = readonly_range {
            p.error(
                p.err_builder(
                    "readonly modifier can only appear on a property or signature declaration",
                )
                .primary(readonly_range, ""),
            );
        }

        Present(method)
    } else {
        parse_ts_type_annotation(p).ok();
        parse_ts_object_type_member_semi(p);
        Present(m.complete(p, TS_PROPERTY_SIGNATURE_OBJECT_TYPE_MEMBER))
    }
}

// test ts_call_signature_member
// // TYPESCRIPT
// type A = { (): string; }
// type B = { (a, b, c): number }
// type C = { <A, B>(a: A, b: B): number }
fn parse_ts_call_signature_object_type_member(p: &mut Parser) -> ParsedSyntax {
    if !(p.at(T!['(']) || p.at(T![<])) {
        return Absent;
    }

    let m = p.start();
    parse_ts_call_signature(p);
    parse_ts_object_type_member_semi(p);
    Present(m.complete(p, TS_CALL_SIGNATURE_OBJECT_TYPE_MEMBER))
}

// test ts_construct_signature_member
// // TYPESCRIPT
// type A = { new (): string; }
// type B = { new (a: string, b: number) }
// type C = { new <A, B>(a: A, b: B): string }
fn parse_ts_construct_signature_object_type_member(p: &mut Parser) -> ParsedSyntax {
    if !is_at_ts_construct_signature_object_type_member(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![new]);
    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    parse_ts_type_annotation(p).ok();
    parse_ts_object_type_member_semi(p);

    Present(m.complete(p, TS_CONSTRUCT_SIGNATURE_OBJECT_TYPE_MEMBER))
}

// test ts_getter_signature_member
// // TYPESCRIPT
// type A = { get a(): number }
// type B = { get a() }
// // members that look like getters but aren't getters
// type C = { get(): number }
// type D = { get: number }
// type E = { get }
fn parse_ts_getter_signature_object_type_member(p: &mut Parser) -> ParsedSyntax {
    if !(p.at(T![ident]) && p.cur_src() == "get") {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![get]);
    parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
    p.expect(T!['(']);
    p.expect(T![')']);
    parse_ts_type_annotation(p).ok();
    parse_ts_object_type_member_semi(p);
    Present(m.complete(p, TS_GETTER_SIGNATURE_OBJECT_TYPE_MEMBER))
}

// test ts_setter_signature_member
// // TYPESCRIPT
// type A = { set a(b: number) }
// type B = { set a(b) }
// // members that look like setters but aren't setters
// type C = { set(a) }
// type D = { set: number }
// type E = { set }
fn parse_ts_setter_signature_object_type_member(p: &mut Parser) -> ParsedSyntax {
    if !(p.at(T![ident]) && p.cur_src() == "set") {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![set]);
    parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
    p.expect(T!['(']);
    parse_any_formal_parameter(
        p,
        ParameterContext::Implementation,
        ExpressionContext::default(),
    )
    .or_add_diagnostic(p, expected_parameter);
    p.expect(T![')']);
    parse_ts_object_type_member_semi(p);
    Present(m.complete(p, TS_SETTER_SIGNATURE_OBJECT_TYPE_MEMBER))
}

/// Must be at `[ident:]` or `readonly [ident:]`
fn is_at_ts_index_signature_object_type_member(p: &Parser) -> bool {
    let mut offset = 0;

    if p.at(T![ident]) && p.cur_src() == "readonly" {
        offset = 1;
    }

    if !p.nth_at(offset, T!['[']) {
        return false;
    }

    if !is_nth_at_identifier(p, offset + 1) {
        return false;
    }

    p.nth_at(offset + 2, T![:])
}

// test ts_index_signature_member
// // TYPESCRIPT
// type A = { [a: number]: string }
// type B = { readonly [a: number]: string }
// // not an index signature
// type C = { [a]: string }
// type D = { readonly [a]: string }
fn parse_ts_index_signature_object_type_member(p: &mut Parser) -> ParsedSyntax {
    if !is_at_ts_index_signature_object_type_member(p) {
        return Absent;
    }

    let m = p.start();

    if p.at(T![ident]) && p.cur_src() == "readonly" {
        p.bump_remap(T![readonly]);
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
    parse_ts_object_type_member_semi(p);

    Present(m.complete(p, TS_INDEX_SIGNATURE_OBJECT_TYPE_MEMBER))
}

// test ts_tuple_type
// // TYPESCRIPT
// type A = [string, number, any]
// type B = [a: string, b: number, c: any]
// type C = [a: string, b: number, ...c: any[]]
// type D = [a?: string]
// type E = [...string[]]
// type F = [string?]
fn parse_ts_tuple_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['[']);
    TsTupleTypeElementList.parse_list(p);
    p.expect(T![']']);

    Present(m.complete(p, TS_TUPLE_TYPE))
}

struct TsTupleTypeElementList;

impl ParseSeparatedList for TsTupleTypeElementList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        if is_at_named_tuple_type_element(p) {
            let m = p.start();
            p.eat(T![...]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            p.eat(T![?]);
            p.bump(T![:]);
            parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
            return Present(m.complete(p, TS_NAMED_TUPLE_TYPE_ELEMENT));
        }

        if p.at(T![...]) {
            let m = p.start();
            p.bump(T![...]);
            parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
            return Present(m.complete(p, TS_REST_TUPLE_TYPE_ELEMENT));
        }

        let ty = parse_ts_type(p);

        if p.at(T![?]) {
            let m = ty.precede_or_add_diagnostic(p, expected_ts_type);
            p.bump(T![?]);
            return Present(m.complete(p, TS_OPTIONAL_TUPLE_TYPE_ELEMENT));
        }

        ty
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T![']'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN,
                token_set![
                    T![']'],
                    T![...],
                    T![ident],
                    T!['['],
                    T!['{'],
                    T![void],
                    T![null]
                ],
            ),
            expected_ts_type,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        TS_TUPLE_TYPE_ELEMENT_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn is_at_named_tuple_type_element(p: &Parser) -> bool {
    let offset = if p.at(T![...]) { 1 } else { 0 };

    // a:
    let is_colon = p.nth_at(offset + 1, T![:]);
    // a?:
    let is_question_colon = p.nth_at(offset + 1, T![?]) && p.nth_at(offset + 2, T![:]);

    is_nth_at_identifier_or_keyword(p, offset) && (is_colon || is_question_colon)
}

// test ts_literal_type
// // TYPESCRIPT
// type A = 5;
// type B = -5;
// type C = 5n;
// type D = -5n;
// type E = "abvcd";
// type F = true;
// type G = false;
// type H = null;
fn parse_ts_literal_type(p: &mut Parser) -> ParsedSyntax {
    if p.cur() == JS_REGEX_LITERAL {
        return Absent;
    }

    if p.at(T![-]) && p.nth_at(1, JS_NUMBER_LITERAL) {
        let m = p.start();
        p.bump(T![-]);

        let number_expr = parse_number_literal_expression(p)
            .or_else(|| parse_big_int_literal_expression(p))
            .unwrap();

        // Inline the number or big int literal into the number/big int literal type
        number_expr.undo_completion(p).abandon(p);

        let type_kind = match number_expr.kind() {
            JS_NUMBER_LITERAL_EXPRESSION => TS_NUMBER_LITERAL_TYPE,
            JS_BIG_INT_LITERAL_EXPRESSION => TS_BIG_INT_LITERAL_TYPE,
            _ => unreachable!(),
        };

        return Present(m.complete(p, type_kind));
    }

    parse_literal_expression(p).map(|mut expression| {
        let type_kind = match expression.kind() {
            JS_NUMBER_LITERAL_EXPRESSION => TS_NUMBER_LITERAL_TYPE,
            JS_BIG_INT_LITERAL_EXPRESSION => TS_BIG_INT_LITERAL_TYPE,
            JS_NULL_LITERAL_EXPRESSION => TS_NULL_LITERAL_TYPE,
            JS_BOOLEAN_LITERAL_EXPRESSION => TS_BOOLEAN_LITERAL_TYPE,
            JS_STRING_LITERAL_EXPRESSION => TS_STRING_LITERAL_TYPE,
            kind => unreachable!("Not a valid kind {:?}", kind),
        };

        expression.change_kind(p, type_kind);
        expression
    })
}

// test ts_template_literal_type
// // TYPESCRIPT
// type A = `abcd`
// type B = `a${A}`
// type C<X extends string> = `c${X}`
fn parse_ts_template_literal_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(BACKTICK) {
        return Absent;
    }

    let m = p.start();
    p.bump(BACKTICK);

    let elements = p.start();
    while !p.at(EOF) && !p.at(BACKTICK) {
        match p.cur() {
			TEMPLATE_CHUNK => {
				let m = p.start();
				p.bump_any();
				m.complete(p, TS_TEMPLATE_CHUNK_ELEMENT);
			}
			DOLLAR_CURLY => {
				let m = p.start();
				p.bump_any();
				parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
				p.expect(T!['}']);
				m.complete(p, TS_TEMPLATE_ELEMENT);
			}
			ERROR_TOKEN => {
				let err = p
					.err_builder("Invalid template literal")
					.primary(p.cur_tok().range(), "");
				p.err_and_bump(err, JS_UNKNOWN);
			},
			t => unreachable!("Anything not template chunk or dollarcurly should have been eaten by the lexer, but {:?} was found", t),
		}
    }
    elements.complete(p, TS_TEMPLATE_ELEMENT_LIST);
    p.expect(BACKTICK);
    Present(m.complete(p, TS_TEMPLATE_LITERAL_TYPE))
}

fn is_at_ts_construct_signature_object_type_member(p: &Parser) -> bool {
    p.at(T![new]) && (p.nth_at(1, T!['(']) || is_nth_at_ts_type_parameters(p, 1))
}

// test ts_constructor_type
// // TYPESCRIPT
// type A = new(a: string, b: number) => string;
// type B = abstract new(a: string, b: number) => string;
// type C = new<A, B>(a: A, b: B) => string;
// type D = abstract new<A, B>(a: A, b: B) => string;
fn parse_ts_constructor_type(p: &mut Parser) -> ParsedSyntax {
    if !is_at_constructor_type(p) {
        return Absent;
    }

    let m = p.start();
    eat_contextual_keyword(p, "abstract", T![abstract]);
    p.expect(T![new]);

    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    p.expect(T![=>]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_CONSTRUCTOR_TYPE))
}

fn is_at_constructor_type(p: &mut Parser) -> bool {
    p.at(T![new]) || (is_at_contextual_keyword(p, "abstract") && p.nth_at(1, T![new]))
}

/// Determines if the parser's currently located at a function type. Performs a lookahead of at most a single character.
fn is_at_function_type(p: &mut Parser) -> bool {
    if p.at(T![<]) {
        // <
        return true;
    }

    if !p.at(T!['(']) {
        return false;
    }

    p.lookahead(|p| {
        p.bump(T!['(']);

        if p.at(T![')']) || p.at(T![...]) {
            // () not a valid parenthesized type
            // (... rest parameters are only valid in function types
            return true;
        }

        if skip_parameter_start(p) {
            // We're passed the start of the parameter. Now we can verify if anything indicates that
            // this is a parameter

            if p.at_ts(token_set![T![:], T![=], T![,], T![?]]) {
                return true;
            }

            return p.at(T![')']) && p.nth_at(1, T![=>]);
        }

        false
    })
}

// test ts_function_type
// // TYPESCRIPT
// type A = () => string;
// type B = (a: string) => string;
// type C = (b = "test") => string;
// type D = (c, d) => string
// type E = ([a]) => string
// type F = ({a}) => string
// type G = <A, B>(a: A, b: B) => string
// type H = (a: any) => a is string;
// type I = ({ a, b }?) => string;
fn parse_ts_function_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![<]) && !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    p.expect(T![=>]);
    parse_ts_return_type(p).or_add_diagnostic(p, expected_ts_type);

    Present(m.complete(p, TS_FUNCTION_TYPE))
}

fn parse_ts_return_type(p: &mut Parser) -> ParsedSyntax {
    let is_asserts_predicate = is_at_contextual_keyword(p, "asserts")
        && (is_nth_at_identifier(p, 1) || p.nth_at(1, T![this]));
    let is_is_predicate =
        (is_at_identifier(p) || p.at(T![this])) && is_nth_at_contextual_keyword(p, 1, "is");

    if is_asserts_predicate || is_is_predicate {
        parse_ts_type_predicate(p)
    } else {
        parse_ts_type(p)
    }
}

// test ts_type_predicate
// // TYPESCRIPT
// type A = (a) => a is string;
// type B = (a) => asserts a is string;
// type asserts = string;
// type C = () => asserts;
fn parse_ts_type_predicate(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    eat_contextual_keyword(p, "asserts", T![asserts]);
    parse_reference_identifier(p)
        .or_else(|| parse_ts_this_type(p))
        .unwrap();
    eat_contextual_keyword(p, "is", T![is]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_TYPE_PREDICATE))
}

pub fn parse_ts_type_arguments_in_expression(p: &mut Parser) -> ParsedSyntax {
    // Don't parse type arguments in JS because the syntax is ambiguous
    // https://github.com/microsoft/TypeScript/issues/36662

    // test type_arguments_like_expression
    // ((0)<5>(6))

    if TypeScript.is_unsupported(p) || !p.at(T![<]) {
        return Absent;
    }

    try_parse(p, |p| {
        let arguments = parse_ts_type_arguments(p);

        if p.tokens.last_tok().map(|t| t.kind) == Some(T![>]) {
            arguments
        } else {
            Absent
        }
    })
}

pub(crate) fn parse_ts_type_arguments(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);
    TypeArgumentsList.parse_list(p);
    p.expect(T![>]);
    Present(m.complete(p, TS_TYPE_ARGUMENTS))
}

struct TypeArgumentsList;

impl ParseSeparatedList for TypeArgumentsList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_type(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T![>])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN,
                token_set![T![>], T![,], T![ident], T![yield], T![await]],
            )
            .enable_recovery_on_line_break(),
            expected_ts_type_parameter,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        TS_TYPE_ARGUMENT_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }
}
