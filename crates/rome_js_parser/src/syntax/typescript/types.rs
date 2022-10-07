use crate::parser::{RecoveryError, RecoveryResult};
use crate::state::{EnterType, SignatureFlags};
use crate::syntax::expr::{
    is_at_identifier, is_nth_at_identifier, is_nth_at_identifier_or_keyword,
    parse_big_int_literal_expression, parse_identifier, parse_literal_expression, parse_name,
    parse_number_literal_expression, parse_reference_identifier, parse_template_elements,
    ExpressionContext,
};
use crate::syntax::function::{
    parse_formal_parameter, parse_parameter_list, skip_parameter_start, ParameterContext,
};
use crate::syntax::js_parse_error::{
    expected_identifier, expected_object_member_name, expected_parameter, expected_parameters,
    expected_property_or_signature,
};
use crate::syntax::object::{
    is_at_object_member_name, is_nth_at_type_member_name, parse_object_member_name,
};
use crate::syntax::stmt::optional_semi;
use crate::syntax::typescript::try_parse;
use crate::syntax::typescript::ts_parse_error::{expected_ts_type, expected_ts_type_parameter};

use crate::lexer::{LexContext, ReLexContext};
use crate::JsSyntaxFeature::TypeScript;
use crate::{
    Absent, CompletedMarker, ParseNodeList, ParseRecovery, ParseSeparatedList, ParsedSyntax,
    Parser, Present, SyntaxFeature,
};
use rome_diagnostics::Span;
use rome_js_syntax::JsSyntaxKind::TS_TYPE_ANNOTATION;
use rome_js_syntax::T;
use rome_js_syntax::{JsSyntaxKind::*, *};

use super::{expect_ts_index_signature_member, is_at_ts_index_signature_member, MemberParent};

pub(crate) fn is_reserved_type_name(name: &str) -> bool {
    name.len() <= 6
        && name.len() >= 3
        && matches!(
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

pub(crate) fn is_reserved_module_name(name: &str) -> bool {
    name.len() == 4 && matches!(name, "void" | "null")
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

// test ts ts_return_type_annotation
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

// test ts ts_type_parameters
// type A<X extends string, Y = number, Z extends string | number = number> = { x: X, y: Y, z: Z }
//
// test_err ts ts_type_parameters_incomplete
// type A<T
pub(crate) fn parse_ts_type_parameters(p: &mut Parser) -> ParsedSyntax {
    if !is_nth_at_ts_type_parameters(p, 0) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);
    if p.at(T![>]) {
        p.error(expected_ts_type_parameter(p, p.cur_range()));
    }
    TsTypeParameterList.parse_list(p);
    p.expect(T![>]);

    Present(m.complete(p, TS_TYPE_PARAMETERS))
}

struct TsTypeParameterList;

impl ParseSeparatedList for TsTypeParameterList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_type_parameter(p)
    }

    fn is_at_list_end(&self, p: &mut Parser) -> bool {
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

// test ts ts_type_constraint_clause
// type A<X extends number> = X;
// type B<X extends number | string> = { a: X }
fn parse_ts_type_constraint_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![extends]);

    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_TYPE_CONSTRAINT_CLAUSE))
}

// test ts ts_default_type_clause
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

fn is_nth_at_ts_type_parameters(p: &mut Parser, n: usize) -> bool {
    p.nth_at(n, T![<])
}

#[inline(always)]
pub(crate) fn parse_ts_type(p: &mut Parser) -> ParsedSyntax {
    parse_ts_type_impl(p, ConditionalType::Allowed)
}

enum ConditionalType {
    Allowed,
    Disallowed,
}

impl ConditionalType {
    fn is_allowed(&self) -> bool {
        matches!(self, ConditionalType::Allowed)
    }
}

fn parse_ts_type_impl(p: &mut Parser, conditional_type: ConditionalType) -> ParsedSyntax {
    p.with_state(EnterType, |p| {
        if is_at_constructor_type(p) {
            return parse_ts_constructor_type(p);
        }

        if is_at_function_type(p) {
            return parse_ts_function_type(p);
        }

        let left = parse_ts_union_type_or_higher(p);

        // test ts ts_conditional_type_call_signature_lhs
        // type X<V> = V extends (...args: any[]) => any ? (...args: Parameters<V>) => void : Function;
        if conditional_type.is_allowed() {
            left.map(|left| {
                // test ts ts_conditional_type
                // type A = number;
                // type B = string extends number ? string : number;
                // type C = A extends (B extends A ? number : string) ? void : number;
                if !p.has_preceding_line_break() && p.at(T![extends]) {
                    let m = left.precede(p);
                    p.expect(T![extends]);
                    parse_ts_type_impl(p, ConditionalType::Disallowed)
                        .or_add_diagnostic(p, expected_ts_type);
                    p.expect(T![?]);
                    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
                    p.expect(T![:]);
                    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
                    m.complete(p, TS_CONDITIONAL_TYPE)
                } else {
                    left
                }
            })
        } else {
            left
        }
    })
}

// test ts ts_union_type
// type A = string | number;
// type B = | A | void | null;
// type C = A & C | C;
fn parse_ts_union_type_or_higher(p: &mut Parser) -> ParsedSyntax {
    parse_ts_union_or_intersection_type(p, IntersectionOrUnionType::Union)
}

// test ts ts_intersection_type
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
    // Leading operator: `& A & B`
    if p.at(ty_kind.operator()) {
        let m = p.start();
        p.bump(ty_kind.operator());
        let list = p.start();
        ty_kind
            .parse_element(p)
            .or_add_diagnostic(p, expected_ts_type);

        eat_ts_union_or_intersection_type_elements(p, ty_kind);

        list.complete(p, ty_kind.list_kind());

        Present(m.complete(p, ty_kind.kind()))
    } else {
        let first = ty_kind.parse_element(p);

        if p.at(ty_kind.operator()) {
            let list = first.precede(p);

            eat_ts_union_or_intersection_type_elements(p, ty_kind);

            let completed_list = list.complete(p, ty_kind.list_kind());
            let m = completed_list.precede(p);
            Present(m.complete(p, ty_kind.kind()))
        } else {
            // Not a union or intersection type
            first
        }
    }
}

#[inline]
fn eat_ts_union_or_intersection_type_elements(p: &mut Parser, ty_kind: IntersectionOrUnionType) {
    while p.at(ty_kind.operator()) {
        p.bump(ty_kind.operator());

        ty_kind
            .parse_element(p)
            .or_add_diagnostic(p, expected_ts_type);
    }
}

fn parse_ts_primary_type(p: &mut Parser) -> ParsedSyntax {
    // test ts ts_inferred_type
    // type A = infer B;
    // type B = { a: infer U; b: infer U};
    if p.at(T![infer]) {
        let m = p.start();
        p.expect(T![infer]);
        parse_ts_type_parameter_name(p).or_add_diagnostic(p, expected_identifier);
        return Present(m.complete(p, TS_INFER_TYPE));
    }

    // test ts ts_type_operator
    // type A = { x: string, y: number };
    // type B = keyof A;
    // type C = readonly string[];
    // const d: unique symbol = Symbol();
    let is_type_operator = matches!(p.cur(), T![unique] | T![keyof] | T![readonly]);
    if is_type_operator {
        let m = p.start();
        p.bump_any();
        parse_ts_primary_type(p).or_add_diagnostic(p, expected_ts_type);
        return Present(m.complete(p, TS_TYPE_OPERATOR_TYPE));
    }

    parse_postfix_type_or_higher(p)
}

fn parse_postfix_type_or_higher(p: &mut Parser) -> ParsedSyntax {
    parse_ts_non_array_type(p).map(|primary_type| {
        let mut left = primary_type;

        while p.at(T!['[']) && !p.has_preceding_line_break() {
            let m = left.precede(p);
            p.bump(T!['[']);

            left = if parse_ts_type(p).is_present() {
                // test ts ts_indexed_access_type
                // type A = string[number];
                // type B = string[number][number][number][];
                p.expect(T![']']);
                m.complete(p, TS_INDEXED_ACCESS_TYPE)
            } else {
                // test ts ts_array_type
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
    // test ts ts_predefined_type
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
            p.expect(T![void]);
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
        _ => {
            if !p.nth_at(1, T![.]) {
                let mapping = match p.cur() {
                    T![any] => Some(TS_ANY_TYPE),
                    T![unknown] => Some(TS_UNKNOWN_TYPE),
                    T![number] => Some(TS_NUMBER_TYPE),
                    T![object] => Some(TS_NON_PRIMITIVE_TYPE),
                    T![boolean] => Some(TS_BOOLEAN_TYPE),
                    T![bigint] => Some(TS_BIGINT_TYPE),
                    T![string] => Some(TS_STRING_TYPE),
                    T![symbol] => Some(TS_SYMBOL_TYPE),
                    T![undefined] => Some(TS_UNDEFINED_TYPE),
                    T![never] => Some(TS_NEVER_TYPE),
                    _ => None,
                };

                if let Some(literal_type_kind) = mapping {
                    let m = p.start();
                    p.bump_any();
                    return Present(m.complete(p, literal_type_kind));
                }
            }

            parse_ts_reference_type(p)
        }
    }
}

// test ts ts_reference_type
// type A = object;
// type B = string;
// type C = A;
// type D = B.a;
// type E = D.c.b.a;
fn parse_ts_reference_type(p: &mut Parser) -> ParsedSyntax {
    parse_ts_name(p).map(|name| {
        let m = name.precede(p);

        if !p.has_preceding_line_break() {
            parse_ts_type_arguments(p).ok();
        }

        m.complete(p, TS_REFERENCE_TYPE)
    })
}

pub(crate) fn parse_ts_name(p: &mut Parser) -> ParsedSyntax {
    let mut left = if p.cur().is_non_contextual_keyword() && !p.cur().is_future_reserved_keyword() {
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

// test ts ts_typeof_type
// let a = "test";
// type B = typeof a;
// type T21 = typeof Array<string>;
// type A<U> = InstanceType<typeof Array<U>>;

// test tsx ts_typeof_type2
// type X = typeof Array
// <div>a</div>;
fn parse_ts_typeof_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![typeof]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![typeof]);
    parse_ts_name(p).or_add_diagnostic(p, expected_identifier);
    if !p.has_preceding_line_break() {
        parse_ts_type_arguments(p).ok();
    }

    Present(m.complete(p, TS_TYPEOF_TYPE))
}

// test ts ts_this_type
// class A {
//     method() {
//         type A = this;
//     }
//     predicate(): this is string {
//         return typeof this === "string"
//     }
// }
fn parse_ts_this_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![this]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![this]);
    Present(m.complete(p, TS_THIS_TYPE))
}

// test ts ts_parenthesized_type
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

fn is_at_start_of_mapped_type(p: &mut Parser) -> bool {
    if !p.at(T!['{']) {
        return false;
    }

    if p.nth_at(1, T![+]) || p.nth_at(1, T![-]) {
        return p.nth_at(2, T![readonly]);
    }

    let mut offset = 1;

    if p.nth_at(offset, T![readonly]) {
        offset += 1;
    }

    p.nth_at(offset, T!['['])
        && (is_nth_at_identifier(p, offset + 1) || p.nth(offset + 1).is_keyword())
        && p.nth_at(offset + 2, T![in])
}

// test ts issue_2790
// var x: {
//   readonly [A in keyof B]?: any;
// };

// test ts ts_mapped_type
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
    if !p.at(T![as]) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![as]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_MAPPED_TYPE_AS_CLAUSE))
}

fn parse_ts_mapped_type_readonly_modifier_clause(p: &mut Parser) -> ParsedSyntax {
    if p.at(T![readonly]) {
        let m = p.start();
        p.expect(T![readonly]);
        Present(m.complete(p, TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE))
    } else if p.at(T![+]) || p.at(T![-]) {
        let m = p.start();
        p.bump_any();
        p.expect(T![readonly]);
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

// test ts ts_import_type
// type A = typeof import("test");
// type B = import("test");
// type C = typeof import("test").a.b.c.d.e.f;
// type D = import("test")<string>;
// type E = import("test").C<string>;
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

    parse_ts_type_arguments(p).ok();

    Present(m.complete(p, TS_IMPORT_TYPE))
}

// test ts ts_object_type
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
    TypeMembers.parse_list(p);
    p.expect(T!['}']);
    Present(m.complete(p, TS_OBJECT_TYPE))
}

pub(crate) struct TypeMembers;

impl ParseNodeList for TypeMembers {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_type_member(p)
    }

    fn is_at_list_end(&self, p: &mut Parser) -> bool {
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
        TS_TYPE_MEMBER_LIST
    }
}

fn parse_ts_type_member(p: &mut Parser) -> ParsedSyntax {
    // test ts ts_index_signature_member
    // type A = { [a: number]: string }
    // type B = { readonly [a: number]: string }
    // // not an index signature
    // type C = { [a]: string }
    // type D = { readonly [a]: string }
    if is_at_ts_index_signature_member(p) {
        let m = p.start();
        return Present(expect_ts_index_signature_member(
            p,
            m,
            MemberParent::TypeOrInterface,
        ));
    }

    match p.cur() {
        T!['('] | T![<] => parse_ts_call_signature_type_member(p),
        T![new] if is_at_ts_construct_signature_type_member(p) => {
            parse_ts_construct_signature_type_member(p)
        }
        T![get] if is_nth_at_type_member_name(p, 1) => parse_ts_getter_signature_type_member(p),
        T![set] if is_nth_at_type_member_name(p, 1) => parse_ts_setter_signature_type_member(p),
        _ => parse_ts_property_or_method_signature_type_member(p),
    }
}

// test ts ts_property_or_method_signature_member
// type A = { a?: string; b?(): number }
// type B = { a: string, b(): number }
// type C = { m(a: string, b: number, c: string): any }
// type D = { readonly: string, readonly a: number }
// type E = { m<A, B>(a: A, b: B): never }
fn parse_ts_property_or_method_signature_type_member(p: &mut Parser) -> ParsedSyntax {
    if !is_at_object_member_name(p) {
        return Absent;
    }

    let m = p.start();
    let readonly_range = if p.at(T![readonly]) && is_nth_at_type_member_name(p, 1) {
        let range = p.cur_range();
        p.expect(T![readonly]);
        Some(range)
    } else {
        None
    };

    parse_object_member_name(p).unwrap();

    p.eat(T![?]);

    if p.at(T!['(']) || p.at(T![<]) {
        parse_ts_call_signature(p);
        parse_ts_type_member_semi(p);
        let method = m.complete(p, TS_METHOD_SIGNATURE_TYPE_MEMBER);

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
        parse_ts_type_member_semi(p);
        Present(m.complete(p, TS_PROPERTY_SIGNATURE_TYPE_MEMBER))
    }
}

// test ts ts_call_signature_member
// type A = { (): string; }
// type B = { (a, b, c): number }
// type C = { <A, B>(a: A, b: B): number }
fn parse_ts_call_signature_type_member(p: &mut Parser) -> ParsedSyntax {
    if !(p.at(T!['(']) || p.at(T![<])) {
        return Absent;
    }

    let m = p.start();
    parse_ts_call_signature(p);
    parse_ts_type_member_semi(p);
    Present(m.complete(p, TS_CALL_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_construct_signature_member
// type A = { new (): string; }
// type B = { new (a: string, b: number) }
// type C = { new <A, B>(a: A, b: B): string }
fn parse_ts_construct_signature_type_member(p: &mut Parser) -> ParsedSyntax {
    if !is_at_ts_construct_signature_type_member(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![new]);
    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    parse_ts_type_annotation(p).ok();
    parse_ts_type_member_semi(p);

    Present(m.complete(p, TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_getter_signature_member
// type A = { get a(): number }
// type B = { get a() }
// // members that look like getters but aren't getters
// type C = { get(): number }
// type D = { get: number }
// type E = { get }
fn parse_ts_getter_signature_type_member(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![get]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![get]);
    parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
    p.expect(T!['(']);
    p.expect(T![')']);
    parse_ts_type_annotation(p).ok();
    parse_ts_type_member_semi(p);
    Present(m.complete(p, TS_GETTER_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_setter_signature_member
// type A = { set a(b: number) }
// type B = { set a(b) }
// // members that look like setters but aren't setters
// type C = { set(a) }
// type D = { set: number }
// type E = { set }
fn parse_ts_setter_signature_type_member(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![set]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![set]);
    parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
    p.expect(T!['(']);
    parse_formal_parameter(p, ParameterContext::Setter, ExpressionContext::default())
        .or_add_diagnostic(p, expected_parameter);
    p.expect(T![')']);
    parse_ts_type_member_semi(p);
    Present(m.complete(p, TS_SETTER_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_tuple_type
// type A = [string, number, any]
// type B = [a: string, b: number, c: any]
// type C = [a: string, b: number, ...c: any[]]
// type D = [a?: string]
// type E = [...string[]]
// type F = [string?]
//
// test_err ts ts_tuple_type_incomplete
// type A = [string,
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
            let has_ellipsis = p.eat(T![...]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            let has_question_mark = p.eat(T![?]);
            p.bump(T![:]);
            parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);

            let mut syntax = m.complete(p, TS_NAMED_TUPLE_TYPE_ELEMENT);

            // test_err ts ts_tuple_type_cannot_be_optional_and_rest
            // type A = [ ...name?: string[] ]
            if has_ellipsis && has_question_mark {
                let err = p
                    .err_builder("A tuple member cannot be both optional and rest.")
                    .primary(syntax.range(p).as_range(), "");
                p.error(err);
                syntax.change_to_unknown(p);
            }

            return Present(syntax);
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

    fn is_at_list_end(&self, p: &mut Parser) -> bool {
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

fn is_at_named_tuple_type_element(p: &mut Parser) -> bool {
    let offset = if p.at(T![...]) { 1 } else { 0 };

    // a:
    let is_colon = p.nth_at(offset + 1, T![:]);
    // a?:
    let is_question_colon = p.nth_at(offset + 1, T![?]) && p.nth_at(offset + 2, T![:]);

    is_nth_at_identifier_or_keyword(p, offset) && (is_colon || is_question_colon)
}

// test ts ts_literal_type
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

        let type_kind = match number_expr.kind() {
            JS_NUMBER_LITERAL_EXPRESSION => TS_NUMBER_LITERAL_TYPE,
            JS_BIG_INT_LITERAL_EXPRESSION => TS_BIG_INT_LITERAL_TYPE,
            _ => unreachable!(),
        };

        // Inline the number or big int literal into the number/big int literal type
        number_expr.undo_completion(p).abandon(p);

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

// test ts ts_template_literal_type
// type A = `abcd`
// type B = `a${A}`
// type C<X extends string> = `c${X}`
//
// test_err ts ts_template_literal_error
// type A = "a";
// type B = "b"
// type C = `${A B}bcd`
// type D = `${A B`
fn parse_ts_template_literal_type(p: &mut Parser) -> ParsedSyntax {
    if !p.at(BACKTICK) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(BACKTICK, LexContext::TemplateElement { tagged: false });

    let elements = p.start();
    parse_template_elements(
        p,
        TS_TEMPLATE_CHUNK_ELEMENT,
        TS_TEMPLATE_ELEMENT,
        false,
        |p| parse_ts_type(p).or_add_diagnostic(p, expected_ts_type),
    );
    elements.complete(p, TS_TEMPLATE_ELEMENT_LIST);
    p.expect(BACKTICK);
    Present(m.complete(p, TS_TEMPLATE_LITERAL_TYPE))
}

fn is_at_ts_construct_signature_type_member(p: &mut Parser) -> bool {
    p.at(T![new]) && (p.nth_at(1, T!['(']) || is_nth_at_ts_type_parameters(p, 1))
}

// test ts ts_constructor_type
// type A = new(a: string, b: number) => string;
// type B = abstract new(a: string, b: number) => string;
// type C = new<A, B>(a: A, b: B) => string;
// type D = abstract new<A, B>(a: A, b: B) => string;
fn parse_ts_constructor_type(p: &mut Parser) -> ParsedSyntax {
    if !is_at_constructor_type(p) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![abstract]);
    p.expect(T![new]);

    parse_ts_type_parameters(p).ok();
    parse_parameter_list(p, ParameterContext::Declaration, SignatureFlags::empty())
        .or_add_diagnostic(p, expected_parameters);
    p.expect(T![=>]);
    parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_CONSTRUCTOR_TYPE))
}

fn is_at_constructor_type(p: &mut Parser) -> bool {
    p.at(T![new]) || (p.at(T![abstract]) && p.nth_at(1, T![new]))
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

// test ts ts_function_type
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

// test ts ts_return_type_asi
// interface I {
//  foo(test: string): I
//  is(): boolean;
//  bar(test: string): I
//  asserts(): boolean;
// }
fn parse_ts_return_type(p: &mut Parser) -> ParsedSyntax {
    let is_asserts_predicate =
        p.at(T![asserts]) && (is_nth_at_identifier(p, 1) || p.nth_at(1, T![this]));
    let is_is_predicate = (is_at_identifier(p) || p.at(T![this])) && p.nth_at(1, T![is]);

    if !p.has_nth_preceding_line_break(1) && (is_asserts_predicate || is_is_predicate) {
        parse_ts_type_predicate(p)
    } else {
        parse_ts_type(p)
    }
}

// test ts ts_type_predicate
// type A = (a) => a is string;
// type B = (a) => asserts a is string;
// type C = (a) => asserts a;
// type asserts = string;
// type D = () => asserts;
fn parse_ts_type_predicate(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    let is_asserts = p.eat(T![asserts]);

    parse_ts_this_type(p)
        .or_else(|| parse_reference_identifier(p))
        .unwrap();

    if is_asserts && p.at(T![is]) {
        let condition = p.start();
        p.expect(T![is]);
        parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
        condition.complete(p, TS_ASSERTS_CONDITION);
    } else if !is_asserts {
        p.expect(T![is]);
        parse_ts_type(p).or_add_diagnostic(p, expected_ts_type);
    }

    let kind = if is_asserts {
        TS_ASSERTS_RETURN_TYPE
    } else {
        TS_PREDICATE_RETURN_TYPE
    };

    Present(m.complete(p, kind))
}

// test ts ts_instantiation_expressions
// let f1 = fx<string>;
// let f2 = fx<string, number>;
// let f3 = fx['test']<string>;
// const a2 = f.g<number>;  // () => number
// const a3 = f<number>.g;  // <U>() => U
// const a4 = f<number>.g<number>;  // () => number
// const a5 = f['g']<number>;  // () => number
// const a7 = (f<number>)['g'];
// const a6 = f<number>['g'];  // type Error
// const b2 = f?.<number>();
// const b3 = f<number>?.();
// const b4 = f<number>?.<number>();  // Type Error, expected no type arguments
// const x1 = f<true>
// (true);
// // Parsed as relational expression
// const x2 = f<true>
// true;
// // Parsed as instantiation expression
// const x3 = f<true>;
// true;

// test ts ts_type_instantiation_expression
// type StringBox = Box<string>;
// // Parsed as instantiation expression
// const x4 = f<true>
// if (true) {}

// test_err ts ts_instantiation_expressions1
// const a8 = f<number><number>;  // Relational operator error
// const b1 = f?.<number>;  // Error, `(` expected

pub(crate) fn parse_ts_type_arguments_in_expression(p: &mut Parser) -> ParsedSyntax {
    // Don't parse type arguments in JS because the syntax is ambiguous
    // https://github.com/microsoft/TypeScript/issues/36662

    // test type_arguments_like_expression
    // ((0)<5>(6))

    if TypeScript.is_unsupported(p) || !matches!(p.cur(), T![<] | T![<<]) {
        return Absent;
    }

    try_parse(p, |p| {
        p.re_lex(ReLexContext::TypeArgumentLessThan);
        let arguments = parse_ts_type_arguments_impl(p, false);

        if p.last() == Some(T![>]) && can_follow_type_arguments_in_expr(p) {
            Ok(Present(arguments))
        } else {
            Err(())
        }
    })
    .unwrap_or(Absent)
}

#[inline]
fn can_follow_type_arguments_in_expr(p: &mut Parser) -> bool {
    let cur_kind = p.cur();
    match cur_kind {
        T!['('] | BACKTICK => true,
        _ => !is_start_of_expr(p),
    }
}

/// You could refer to https://github.com/microsoft/TypeScript/blob/42b1049aee8c655631cb4f0065de86ec1023d20a/src/compiler/parser.ts#L4475
fn is_start_of_expr(p: &mut Parser) -> bool {
    if is_start_of_left_hand_side_expression(p) {
        return true;
    }
    match p.cur() {
        T![+]
        | T![-]
        | T![~]
        | T![!]
        | T![delete]
        | T![typeof]
        | T![void]
        | T![++]
        | T![--]
        | T![<]
        | T![await]
        | T![yield] => true,
        // TODO: how to represent private identifier
        _ => is_binary_operator(p) || is_at_identifier(p),
    }
}

/// Please refer to https://github.com/microsoft/TypeScript/blob/42b1049aee8c655631cb4f0065de86ec1023d20a/src/compiler/parser.ts#L5141-L5147
fn is_binary_operator(p: &mut Parser) -> bool {
    // TODO: https://github.dev/microsoft/TypeScript/blob/42b1049aee8c655631cb4f0065de86ec1023d20a/src/compiler/parser.ts#L5142-L5144 Optional variance

    // In typescript, the operatorPrecedence of `Comma` is 0(https://github.com/microsoft/TypeScript/blob/42b1049aee8c655631cb4f0065de86ec1023d20a/src/compiler/utilities.ts#L3555), so https://github.com/microsoft/TypeScript/blob/42b1049aee8c655631cb4f0065de86ec1023d20a/src/compiler/parser.ts#L5146 means we need to ensure the `OperatorPrecedence` is bigger than `Comma`
    matches!(OperatorPrecedence::try_from_binary_operator(p.cur()), Some(precedence) if precedence > OperatorPrecedence::Comma)
}

/// You could refer to https://github.com/microsoft/TypeScript/blob/42b1049aee8c655631cb4f0065de86ec1023d20a/src/compiler/parser.ts#L4446
fn is_start_of_left_hand_side_expression(p: &mut Parser) -> bool {
    match p.cur() {
        T![super]
        | T![null]
        | T![true]
        | T![false]
        | JS_NUMBER_LITERAL
        | JS_BIG_INT_LITERAL
        | JS_STRING_LITERAL
        | BACKTICK
        | T!['(']
        | T!['{']
        | T!['[']
        | T![function]
        | T![class]
        | T![new]
        | T![/]
        | T![/=] => true,
        T![import] => p.nth_at(1, T!['(']) || p.nth_at(1, T![<]),

        _ => is_at_identifier(p),
    }
}

pub(crate) fn parse_ts_type_arguments(p: &mut Parser) -> ParsedSyntax {
    // test ts ts_type_arguments_left_shift
    // type A<T> = T;
    // type B = A<<C>(c: C) => undefined>;
    let current = p.re_lex(ReLexContext::TypeArgumentLessThan);
    if current != T![<] {
        return Absent;
    }

    Present(parse_ts_type_arguments_impl(p, true))
}

// test_err ts type_arguments_incomplete
// func<T,
pub(crate) fn parse_ts_type_arguments_impl(
    p: &mut Parser,
    recover_on_errors: bool,
) -> CompletedMarker {
    let m = p.start();
    p.bump(T![<]);

    if p.at(T![>]) {
        p.error(expected_ts_type_parameter(p, p.cur_range()));
    }
    TypeArgumentsList { recover_on_errors }.parse_list(p);
    p.expect(T![>]);
    m.complete(p, TS_TYPE_ARGUMENTS)
}

struct TypeArgumentsList {
    recover_on_errors: bool,
}

impl ParseSeparatedList for TypeArgumentsList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_type(p)
    }

    fn is_at_list_end(&self, p: &mut Parser) -> bool {
        p.at(T![>])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        if parsed_element.is_absent() && !self.recover_on_errors {
            // test ts type_arguments_no_recovery
            // for (let i = 0 ; i < 3; ++i) {
            //     verify.completions({
            //         marker: `${i + 1}`,
            //         exact: [
            //             { name: "foo", replacementSpan: test.ranges()[i] },
            //             { name: "bar", replacementSpan: test.ranges()[i] },
            //         ]
            //     });
            // }

            // Parse conditional expression speculatively tries to parse a list of type arguments
            // The parser shouldn't perform error recovery in that case and simply bail out of parsing
            RecoveryResult::Err(RecoveryError::AlreadyRecovered)
        } else {
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

fn parse_ts_type_member_semi(p: &mut Parser) {
    // type members can either be separated by a comma
    if p.eat(T![,]) {
        return;
    }

    // or a semicolon (possibly ASI)
    if !optional_semi(p) {
        let err = p.err_builder("';' expected'").primary(
            p.cur_range(),
            "An explicit or implicit semicolon is expected here...",
        );

        p.error(err);
    }
}
