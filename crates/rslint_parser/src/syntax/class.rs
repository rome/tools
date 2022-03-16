use crate::parser::{ParsedSyntax, ParserProgress, RecoveryResult};
use crate::state::{
    EnableStrictMode, EnterClassPropertyInitializer, EnterClassStaticInitializationBlock,
    EnterParameters, SignatureFlags,
};
use crate::syntax::binding::parse_binding;
use crate::syntax::expr::{
    parse_assignment_expression_or_higher, parse_lhs_expr, parse_private_name, ExpressionContext,
};
use crate::syntax::function::{
    parse_any_parameter, parse_formal_parameter, parse_function_body, parse_parameter_list,
    parse_parameters_list, parse_ts_type_annotation_or_error, ParameterContext,
};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
    expected_binding, modifier_already_seen, modifier_cannot_be_used_with_modifier,
    modifier_must_precede_modifier,
};
use crate::syntax::object::{
    is_at_literal_member_name, parse_computed_member_name, parse_literal_member_name,
};
use crate::syntax::stmt::{optional_semi, parse_statements, StatementContext};
use crate::syntax::typescript::ts_parse_error::{
    ts_accessibility_modifier_already_seen, ts_accessor_type_parameters_error,
    ts_constructor_type_parameters_error, ts_modifier_cannot_appear_on_a_constructor_declaration,
    ts_modifier_cannot_appear_on_a_parameter, ts_only_syntax_error,
    ts_set_accessor_return_type_error,
};
use crate::syntax::typescript::{
    is_reserved_type_name, parse_ts_implements_clause, parse_ts_return_type_annotation,
    parse_ts_type_annotation, parse_ts_type_arguments, parse_ts_type_parameters,
};

use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{
    CompletedMarker, Marker, ParseNodeList, ParseRecovery, Parser, StrictMode, SyntaxFeature,
};
use bitflags::bitflags;
use drop_bomb::DebugDropBomb;
use rome_js_syntax::JsSyntaxKind::*;
use rome_js_syntax::TextSize;
use rome_js_syntax::{JsSyntaxKind, T};
use rome_rowan::{SyntaxKind, TextRange};
use rslint_errors::Diagnostic;
use smallvec::SmallVec;
use std::fmt::Debug;
use std::ops::Add;
use std::slice::Iter;

use super::function::LineBreak;
use super::js_parse_error::unexpected_body_inside_ambient_context;
use super::typescript::ts_parse_error::{self, unexpected_abstract_member_with_body};
use super::typescript::{
    expect_ts_index_signature_member, is_at_ts_index_signature_member, MemberParent,
};

pub(crate) fn is_at_ts_abstract_class_declaration(
    p: &mut Parser,
    should_check_line_break: LineBreak,
) -> bool {
    let tokens = p.at(T![abstract]) && p.nth_at(1, T![class]);
    if should_check_line_break == LineBreak::DoCheck {
        tokens && !p.has_nth_preceding_line_break(1)
    } else {
        tokens
    }
}

/// Parses a class expression, e.g. let a = class {}
pub(super) fn parse_class_expression(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![class]) {
        return Absent;
    }

    let m = p.start();
    Present(parse_class(p, m, ClassKind::Expression))
}

// test class_declaration
// class foo {}
// class foo extends bar {}
// class foo extends foo.bar {}

// test_err class_decl_err
// class {}
// class extends bar {}
// class foo { set {} }
// class extends {}

// test ts ts_abstract_classes
// abstract class A {}
// abstract class ConcreteMembers {
//     name: string;
//     constructor(name: string) { this.name = name; }
//     display(): void { console.log(this.name); }
//     public get my_name() { return this.name; }
//     public set my_name(name) { this.name = name; }
//     #private_method() { }
// }
// abstract class AbstractMembers {
//     abstract name: string;
//     abstract display();
//     abstract get my_name();
//     abstract set my_name(val);
// }

// test_err ts typescript_abstract_classes_incomplete
// abstract class {};

// test_err ts typescript_abstract_classes_invalid_abstract_constructor
// abstract class A { abstract constructor();};

/// Parses a class declaration if it is valid and otherwise returns [Invalid].
///
/// A class can be invalid if
/// * It uses an illegal identifier name
pub(super) fn parse_class_declaration(p: &mut Parser, context: StatementContext) -> ParsedSyntax {
    let is_abstract_class = p.at(T![abstract]) && p.nth_at(1, T![class]);

    if !p.at(T![class]) && !is_abstract_class {
        return Absent;
    }

    let m = p.start();
    let mut class = parse_class(p, m, ClassKind::Declaration);

    if !class.kind().is_unknown() && context.is_single_statement() {
        // test_err class_in_single_statement_context
        // if (true) class A {}
        p.error(
            p.err_builder("Classes can only be declared at top level or inside a block")
                .primary(class.range(p), "wrap the class in a block statement"),
        );
        class.change_to_unknown(p)
    }

    Present(class)
}

// test export_default_class_clause
// export default class {}

// test ts typescript_export_default_abstract_class_case
// export default abstract class {}
pub(super) fn parse_class_export_default_declaration(p: &mut Parser) -> ParsedSyntax {
    let is_abstract_class = p.at(T![abstract]) && p.nth_at(1, T![class]);

    if !p.at(T![class]) && !is_abstract_class {
        return Absent;
    }

    let m = p.start();

    Present(parse_class(p, m, ClassKind::ExportDefault))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ClassKind {
    Declaration,
    Expression,
    ExportDefault,
}

impl ClassKind {
    fn is_id_optional(&self) -> bool {
        matches!(self, ClassKind::Expression | ClassKind::ExportDefault)
    }
}

impl From<ClassKind> for JsSyntaxKind {
    fn from(kind: ClassKind) -> Self {
        match kind {
            ClassKind::Declaration => JS_CLASS_DECLARATION,
            ClassKind::Expression => JS_CLASS_EXPRESSION,
            ClassKind::ExportDefault => JS_CLASS_EXPORT_DEFAULT_DECLARATION,
        }
    }
}

// test class_named_abstract_is_valid_in_js
// class abstract {}

// test ts ts_class_named_abstract_is_valid_in_ts
// class abstract {}
fn parse_class(p: &mut Parser, m: Marker, kind: ClassKind) -> CompletedMarker {
    let is_abstract = p.eat(T![abstract]);

    let class_token_range = p.cur_range();
    p.expect(T![class]);

    let p = &mut *p.with_scoped_state(EnableStrictMode(StrictMode::Class(p.cur_range())));

    // test_err ts class_decl_no_id
    // class {}
    // class implements B {}
    let id = match p.cur() {
        T![implements] if TypeScript.is_supported(p) => Absent,
        T![extends] => Absent,
        _ => parse_binding(p),
    };

    // parse class id
    match id {
        Present(id) => {
            let text = p.source(id.range(p));
            if TypeScript.is_supported(p) && is_reserved_type_name(text) {
                let err = p
                    .err_builder(&format!(
                            "`{}` cannot be used as a class name because it is already reserved as a type",
                            text
                        ))
                    .primary(id.range(p), "");

                p.error(err);
            }
        }
        Absent => {
            if !kind.is_id_optional() {
                let err = p
                    .err_builder("class declarations must have a name")
                    .primary(class_token_range.start()..p.cur_range().start(), "");

                p.error(err);
            }
        }
    }

    // test ts ts_class_type_parameters
    // class BuildError<A, B, C> {}
    TypeScript
        .parse_exclusive_syntax(p, parse_ts_type_parameters, |p, type_parameters| {
            ts_only_syntax_error(p, "class type parameters", type_parameters.range(p))
        })
        .ok();

    eat_class_heritage_clause(p);

    p.expect(T!['{']);
    ClassMembersList {
        inside_abstract_class: is_abstract,
    }
    .parse_list(p);
    p.expect(T!['}']);

    m.complete(p, kind.into())
}

// test_err class_extends_err
// class A extends bar extends foo {}
// class B extends bar, foo {}
// class C implements B {}
//
// test_err ts ts_class_heritage_clause_errors
// class A {}
// interface Int {}
// class B implements Int extends A {}
// class C implements Int implements Int {}
// class D implements {}
// class E extends {}
// class F extends E, {}
/// Eats a class's 'implements' and 'extends' clauses, attaching them to the current active node.
/// Implements error recovery in case a class has multiple extends/implements clauses or if they appear
/// out of order
fn eat_class_heritage_clause(p: &mut Parser) {
    let mut first_extends: Option<CompletedMarker> = None;
    let mut first_implements: Option<CompletedMarker> = None;

    loop {
        match p.cur() {
            T![extends] => {
                let current = parse_extends_clause(p).expect(
                    "Expected extends clause because parser is positioned at extends keyword",
                );

                match first_extends.as_ref() {
                    None => {
                        first_extends = {
                            if let Some(first_implements) = first_implements.as_ref() {
                                p.error(
                                    p.err_builder(
                                        "'extends' clause must precede 'implements' clause.",
                                    )
                                    .primary(current.range(p), "")
                                    .secondary(first_implements.range(p), ""),
                                )
                            }

                            Some(current)
                        }
                    }
                    Some(first_extends) => p.error(
                        p.err_builder("'extends' clause already seen.")
                            .primary(current.range(p), "")
                            .secondary(first_extends.range(p), "first 'extends' clause"),
                    ),
                }
            }
            T![implements] => {
                let mut current = parse_ts_implements_clause(p).expect("expected 'implements' clause because parser is positioned at 'implements' keyword.");

                match first_implements.as_ref() {
                    None => {
                        first_implements = {
                            if TypeScript.is_unsupported(p) {
                                p.error(
                                    p.err_builder(
                                        "classes can only implement interfaces in TypeScript files",
                                    )
                                    .primary(current.range(p), ""),
                                );
                                current.change_to_unknown(p);
                            }
                            Some(current)
                        }
                    }
                    Some(first_implements) => {
                        p.error(
                            p.err_builder("'implements' clause already seen.")
                                .primary(current.range(p), "")
                                .secondary(first_implements.range(p), "first 'implements' clause"),
                        );
                    }
                }
            }
            _ => break,
        }
    }
}

// test ts ts_extends_generic_type
// type IHasVisualizationModel = string;
// class D extends C<IHasVisualizationModel> {
//     x = "string";
// }
fn parse_extends_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    let extends_end = p.cur_range().end();
    p.expect(T![extends]);

    if parse_extends_expression(p).is_absent() {
        p.error(
            p.err_builder("'extends' list cannot be empty.")
                .primary(extends_end..extends_end, ""),
        )
    } else {
        TypeScript
            .parse_exclusive_syntax(p, parse_ts_type_arguments, |p, arguments| {
                ts_only_syntax_error(p, "type arguments", arguments.range(p))
            })
            .ok();
    }

    while p.at(T![,]) {
        let comma_range = p.cur_range();
        p.bump(T![,]);

        let extra = p.start();
        if parse_extends_expression(p).is_absent() {
            p.error(
                p.err_builder("Trailing comma not allowed.")
                    .primary(comma_range, ""),
            );
            extra.abandon(p);
            break;
        }

        parse_ts_type_arguments(p).ok();

        let extra_class = extra.complete(p, JS_UNKNOWN);

        p.error(
            p.err_builder("Classes can only extend a single class.")
                .primary(extra_class.range(p), ""),
        );
    }

    Present(m.complete(p, JS_EXTENDS_CLAUSE))
}

fn parse_extends_expression(p: &mut Parser) -> ParsedSyntax {
    if p.at(T!['{']) && p.nth_at(1, T!['}']) {
        // Don't eat the body of the class as an object expression except if we have
        // * extends {} {
        // * extends {} implements
        // * extends {},
        if !matches!(p.nth(2), T![extends] | T![implements] | T!['{'] | T![,]) {
            return Absent;
        }
    }

    parse_lhs_expr(p, ExpressionContext::default())
}

struct ClassMembersList {
    inside_abstract_class: bool,
}

impl ParseNodeList for ClassMembersList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_class_member(p, self.inside_abstract_class)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        // test_err invalid_method_recover
        // class {
        //   [1 + 1] = () => {
        //     let a=;
        //   };
        // };
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN_MEMBER,
                token_set![
                    T![;],
                    T![ident],
                    T![readonly],
                    T![private],
                    T![protected],
                    T![public],
                    T![override],
                    T![declare],
                    T![static],
                    T![async],
                    T![yield],
                    T!['}'],
                    T![#]
                ],
            ),
            js_parse_error::expected_class_member,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_CLASS_MEMBER_LIST
    }
}

// test class_declare
// class B { declare() {} }
// class B { declare = foo }

// test static_method
// class foo {
//  static foo(bar) {}
//  static *foo() {}
//  static async foo() {}
//  static async *foo() {}
// }
fn parse_class_member(p: &mut Parser, inside_abstract_class: bool) -> ParsedSyntax {
    let member_marker = p.start();
    // test class_empty_element
    // class foo { ;;;;;;;;;; get foo() {};;;;}
    if p.eat(T![;]) {
        return Present(member_marker.complete(p, JS_EMPTY_CLASS_MEMBER));
    }

    let mut modifiers = parse_class_member_modifiers(p, false);

    if is_at_static_initialization_block_class_member(p) {
        return Present(parse_static_initialization_block_class_member(
            p,
            member_marker,
            modifiers,
        ));
    }

    let member = parse_class_member_impl(p, member_marker, &mut modifiers);

    match member {
        Present(mut member) => {
            let mut valid = true;
            if !inside_abstract_class {
                // test_err ts ts_concrete_class_with_abstract_members
                // class A {
                //    abstract my_age: number;
                //    abstract name(): string;
                //    abstract get age(): number;
                //    abstract set age(v);
                // }
                if let Some(abstract_token_range) =
                    modifiers.get_first_range(ModifierKind::Abstract)
                {
                    let err = p
                        .err_builder("Only abstract classes can have abstract members")
                        .primary(abstract_token_range, "");
                    p.error(err);
                    valid = false;
                }
            }

            let modifiers_valid = modifiers.validate_and_complete(p, member.kind());

            if !valid || !modifiers_valid {
                member.change_to_unknown(p);
            }

            Present(member)
        }
        Absent => {
            debug_assert!(modifiers.is_empty());
            modifiers.abandon(p);
            Absent
        }
    }
}

// test ts ts_index_signature_class_member
// class A {
//     [a: number]: string;
// }
// class B {
//     [index: string]: { prop }
// }

// test ts ts_index_signature_class_member_can_be_static
// class A {
//     static [a: number]: string;
// }
// class B {
//     static readonly [a: number]: string;
// }

fn parse_index_signature_class_member(p: &mut Parser, member_marker: Marker) -> ParsedSyntax {
    TypeScript.parse_exclusive_syntax(
        p,
        |p| {
            Present(expect_ts_index_signature_member(
                p,
                member_marker,
                MemberParent::Class,
            ))
        },
        |p, member| ts_only_syntax_error(p, "Index signatures", member.range(p)),
    )
}

fn parse_class_member_impl(
    p: &mut Parser,
    member_marker: Marker,
    modifiers: &mut ClassMemberModifiers,
) -> ParsedSyntax {
    let start_token_pos = p.tokens.position();
    let generator_range = p.cur_range();

    // Seems like we're at a generator method
    if p.at(T![*]) {
        p.bump_any(); // bump * token
        if is_at_constructor(p, modifiers) {
            let err = p
                .err_builder("constructors can't be generators")
                .primary(generator_range, "");

            p.error(err);
        }

        return Present(parse_method_class_member(
            p,
            member_marker,
            modifiers,
            SignatureFlags::GENERATOR,
        ));
    };

    // Seems like we're at an async method
    if p.at(T![async])
        && !p.nth_at(1, T![?])
        && !is_at_method_class_member(p, 1)
        && !p.has_nth_preceding_line_break(1)
    {
        let async_range = p.cur_range();
        p.expect(T![async]);

        let mut flags = SignatureFlags::ASYNC;

        if p.eat(T![*]) {
            flags |= SignatureFlags::GENERATOR;
        }

        return Present(if is_at_constructor(p, modifiers) {
            let err = p
                .err_builder("constructors cannot be async")
                .primary(async_range, "");

            p.error(err);
            parse_class_member_name(p, modifiers).unwrap();
            parse_constructor_class_member_body(p, member_marker, modifiers)
        } else {
            parse_method_class_member(p, member_marker, modifiers, flags)
        });
    }

    // Seems like we're at an index member
    if is_at_ts_index_signature_member(p) {
        return parse_index_signature_class_member(p, member_marker);
    }

    // test getter_class_member
    // class Getters {
    //   get foo() {}
    //   get static() {}
    //   static get bar() {}
    //   get "baz"() {}
    //   get ["a" + "b"]() {}
    //   get 5() {}
    //   get #private() {}
    // }
    // class NotGetters {
    //   get() {}
    //   async get() {}
    //   static get() {}
    // }
    //
    // test_err method_getter_err
    // class foo {
    //  get {}
    // }
    //

    // test setter_class_member
    // class Setters {
    //   set foo(a) {}
    //   set static(a) {}
    //   static set bar(a) {}
    //   set "baz"(a) {}
    //   set ["a" + "b"](a) {}
    //   set 5(a) {}
    //   set #private(a) {}
    // }
    // class NotSetters {
    //   set(a) {}
    //   async set(a) {}
    //   static set(a) {}
    // }
    //
    // test_err setter_class_member
    // class Setters {
    //   set foo() {}
    // }
    if matches!(p.cur(), T![get] | T![set]) && is_at_class_member_name(p, 1) {
        let is_getter = p.at(T![get]);
        if is_getter {
            p.expect(T![get]);
        } else {
            p.expect(T![set]);
        }

        // So we've seen a get that now must be followed by a getter/setter name
        parse_class_member_name(p, modifiers)
            .or_add_diagnostic(p, js_parse_error::expected_class_member_name);

        // test_err ts ts_getter_setter_type_parameters
        // class Test {
        //  get a<A>(): A {}
        //  set a<A>(value: A) {}
        // }
        if let Present(type_parameters) = parse_ts_type_parameters(p) {
            p.error(ts_accessor_type_parameters_error(p, &type_parameters))
        }

        let completed = if is_getter {
            p.expect(T!['(']);
            p.expect(T![')']);
            parse_ts_type_annotation_or_error(p).ok();

            let member_kind = expect_accessor_body(p, &member_marker, modifiers);
            member_marker.complete(p, member_kind.as_getter_syntax_kind())
        } else {
            let has_l_paren = p.expect(T!['(']);
            p.with_state(EnterParameters(SignatureFlags::empty()), |p| {
                parse_formal_parameter(
                    p,
                    ParameterContext::Setter,
                    ExpressionContext::default().and_object_expression_allowed(has_l_paren),
                )
            })
            .or_add_diagnostic(p, js_parse_error::expected_parameter);
            p.expect(T![')']);

            // test_err ts ts_setter_return_type_annotation
            // class Test {
            //     set a(value: string): void {}
            // }
            if let Present(return_type_annotation) = parse_ts_return_type_annotation(p) {
                p.error(ts_set_accessor_return_type_error(
                    p,
                    &return_type_annotation,
                ));
            }

            let member_kind = expect_accessor_body(p, &member_marker, modifiers);
            member_marker.complete(p, member_kind.as_setter_syntax_kind())
        };

        return Present(completed);
    }

    let is_constructor = is_at_constructor(p, modifiers);
    let member_name = parse_class_member_name(p, modifiers)
        .or_add_diagnostic(p, js_parse_error::expected_class_member_name);

    if is_at_method_class_member(p, 0) {
        // test class_static_constructor_method
        // class B { static constructor() {} }

        // test constructor_class_member
        // class Foo {
        //   constructor(a) {
        //     this.a = a;
        //   }
        // }
        // class Bar {
        //   "constructor"(b) {
        //     this.b = b;
        //   }
        // }
        return if is_constructor {
            Present(parse_constructor_class_member_body(
                p,
                member_marker,
                modifiers,
            ))
        } else {
            // test method_class_member
            // class Test {
            //   method() {}
            //   async asyncMethod() {}
            //   async* asyncGeneratorMethod() {}
            //   * generatorMethod() {}
            //   "foo"() {}
            //   ["foo" + "bar"]() {}
            //   5() {}
            //   #private() {}
            // }
            // class ContextualKeywords {
            //    // Methods called static
            //   static() {}
            //   async static() {}
            //   * static() {}
            //   async* static() {}
            //   declare() {}
            //   get() {} // Method called get
            //   set() {} // Method called set
            // }
            // class Static {
            //   static method() {}
            //   static async asyncMethod() {}
            //   static async* asyncGeneratorMethod() {}
            //   static * generatorMethod() {}
            //   static static() {}
            //   static async static() {}
            //   static async* static() {}
            //   static * static() {}
            // }
            Present(parse_method_class_member_rest(
                p,
                member_marker,
                modifiers,
                SignatureFlags::empty(),
            ))
        };
    }

    match member_name {
        Some(_) => {
            // test property_class_member
            // class foo {
            //   property
            //   declare;
            //   initializedProperty = "a"
            //   "a";
            //   5
            //   ["a" + "b"]
            //   static staticProperty
            //   static staticInitializedProperty = 1
            //   #private
            //   #privateInitialized = "a"
            //   static #staticPrivate
            //   static #staticPrivateInitializedProperty = 1
            // }
            //
            // test_err class_declare_member
            // class B { declare foo }

            // test ts ts_property_class_member_can_be_named_set_or_get
            // class B { set: String; get: Number }
            let mut property = parse_property_class_member_body(p, member_marker, modifiers);

            if !property.kind().is_unknown() && is_constructor {
                let err = p
                    .err_builder("class properties may not be called `constructor`")
                    .primary(property.range(p), "");

                p.error(err);
                property.change_to_unknown(p);
            }

            Present(property)
        }
        None => {
            // test_err block_stmt_in_class
            // class S{{}}
            debug_assert_eq!(
                p.tokens.position(),
                start_token_pos,
                "Parser shouldn't be progressing when returning Absent"
            );

            member_marker.abandon(p);
            Absent
        }
    }
}

fn is_at_static_initialization_block_class_member(p: &mut Parser) -> bool {
    p.at(T![static]) && p.nth_at(1, T!['{'])
}

// test static_initialization_block_member
// class A {
//   static a;
//   static {
//     this.a = "test";
//   }
// }
//
fn parse_static_initialization_block_class_member(
    p: &mut Parser,
    member_marker: Marker,
    modifiers: ClassMemberModifiers,
) -> CompletedMarker {
    if modifiers.is_empty() {
        modifiers.abandon(p);
    } else {
        // test_err ts ts_class_initializer_with_modifiers
        // class A {
        //   public static { }
        // }
        p.error(
            p.err_builder("Static class blocks cannot have any modifier.")
                .primary(modifiers.list_marker.range(p), ""),
        );
        modifiers.validate_and_complete(p, JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER);
    }

    p.expect(T![static]);
    p.expect(T!['{']);
    p.with_state(EnterClassStaticInitializationBlock, |p| {
        parse_statements(p, true)
    });
    p.expect(T!['}']);

    member_marker.complete(p, JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER)
}

/// Parses the body of a property class member (anything after the member name). If the current member is abstract, the [ParsedSyntax]
/// will have kind TS_PROPERTY_SIGNATURE_CLASS_MEMBER, otehrwise will be JS_PROPERTY_CLASS_MEMBER.
///
///  # Arguments
///
/// * `p` - Parser being used
/// * `member_marker` - Marker that will be completed at the end of this function
/// * `modifiers` - All the member modifiers parsed previously. This will be used for validation and for the [ParsedSyntax::kind]
fn parse_property_class_member_body(
    p: &mut Parser,
    member_marker: Marker,
    modifiers: &ClassMemberModifiers,
) -> CompletedMarker {
    parse_ts_property_annotation(p, modifiers).ok();

    // test class_await_property_initializer
    // // SCRIPT
    // async function* test() {
    //   class A {
    //     prop = await;
    //   }
    // }

    // test_err class_yield_property_initializer
    // // SCRIPT
    // async function* test() {
    //   class A {
    //     prop = yield;
    //   }
    // }

    let initializer_syntax = p.with_state(EnterClassPropertyInitializer, |p| {
        parse_initializer_clause(p, ExpressionContext::default())
    });

    expect_member_semi(p, &member_marker, "class property");

    let is_signature = modifiers.is_signature() || p.state.in_ambient_context();
    let kind = if is_signature {
        TS_PROPERTY_SIGNATURE_CLASS_MEMBER
    } else {
        JS_PROPERTY_CLASS_MEMBER
    };

    let member = member_marker.complete(p, kind);

    if let Present(initializer) = &initializer_syntax {
        if modifiers.has(ModifierKind::Abstract) {
            // test_err ts ts_abstract_property_cannot_have_initiliazers
            // abstract class A {
            //     abstract name: string = "";
            // }
            p.error(
                p.err_builder("Property cannot have an initializer because it is marked abstract.")
                    .primary(initializer.range(p), ""),
            );
        } else if modifiers.has(ModifierKind::Declare) || p.state.in_ambient_context() {
            // test_err ts ts_property_initializer_ambient_context
            // declare class A { prop = "test" }
            // class B { declare prop = "test" }
            p.error(
                p.err_builder("Initializers are not allowed in ambient contexts.")
                    .primary(initializer.range(p), ""),
            );
        }
    }

    member
}

fn expect_member_semi(p: &mut Parser, member_marker: &Marker, name: &str) {
    if !optional_semi(p) {
        // Gets the start of the member
        let end = p
            .last_range()
            .map(|r| r.end())
            .unwrap_or_else(|| p.cur_range().start());

        let err = p
            .err_builder(&format!(
                "expected a semicolon to end the {name}, but found none"
            ))
            .primary(member_marker.start()..end, "");

        p.error(err);
    }
}

// test_err js_class_property_with_ts_annotation
// class A {
//  a: string;
//  b?: string;
//  c!: string
// }
//
// test ts ts_class_property_annotation
// class A {
//   a: string;
//   b?: string = "test";
//   c!: string;
// }
fn parse_ts_property_annotation(p: &mut Parser, modifiers: &ClassMemberModifiers) -> ParsedSyntax {
    if !p.at(T![?]) && !p.at(T![!]) {
        return parse_ts_type_annotation_or_error(p);
    }

    let m = p.start();
    let mut valid = true;

    // test ts ts_abstract_property_can_be_optional
    // abstract class A {
    //      abstract name?: string;
    // }
    let optional_range = match optional_member_token(p) {
        Ok(optional_range) => optional_range,
        Err(optional_range) => {
            valid = false;
            Some(optional_range)
        }
    };

    let definite_range = if p.at(T![!]) {
        let range = p.cur_range();
        p.bump(T![!]);

        if TypeScript.is_unsupported(p) {
            let error = p
                .err_builder("`!` modifiers can only be used in TypeScript files")
                .primary(range, "");

            p.error(error);
            valid = false;
        }
        // test_err ts ts_abstract_property_cannot_be_definite
        // abstract class A {
        //      abstract name!: string;
        // }
        else if modifiers.has(ModifierKind::Abstract) {
            p.error(
                p.err_builder(
                    "A definite assignment operator '!' cannot appear on an 'abstract' property.",
                )
                .primary(range, ""),
            );
            valid = false;
        } else if modifiers.has(ModifierKind::Declare) || p.state.in_ambient_context() {
            // test_err ts ts_definite_assignment_in_ambient_context
            // declare class A { prop!: string }
            // class B { declare prop!: string }
            p.error(
                p.err_builder(
                    "Definite assignment operators '!' aren't allowed in ambient contexts.",
                )
                .primary(range, ""),
            );
        }

        Some(range)
    } else {
        None
    };

    let mut annotation = match (optional_range, definite_range) {
        (Some(_), None) => {
            parse_ts_type_annotation(p).ok();
            m.complete(p, TS_OPTIONAL_PROPERTY_ANNOTATION)
        }
        (None, Some(_)) => {
            parse_ts_type_annotation(p).or_add_diagnostic(p, |p, range| {
                p.err_builder("Properties with definite assignment assertions must also have type annotations.").primary(range, "")
            });
            m.complete(p, TS_DEFINITE_PROPERTY_ANNOTATION)
        }
        (Some(optional_range), Some(definite_range)) => {
            parse_ts_type_annotation(p).ok();
            let error = p
                .err_builder("class properties cannot be both optional and definite")
                .primary(definite_range, "")
                .secondary(optional_range, "");

            p.error(error);

            m.complete(p, JS_UNKNOWN)
        }
        // handled by the test at the beginning of the function that returns if the parser isn't at a
        // ! or ? token.
        (None, None) => unreachable!(),
    };

    if !valid {
        annotation.change_to_unknown(p);
    }

    Present(annotation)
}

/// Eats the '?' token for optional member. Emits an error if this isn't typescript
fn optional_member_token(p: &mut Parser) -> Result<Option<TextRange>, TextRange> {
    if p.at(T![?]) {
        let range = p.cur_range();
        p.bump(T![?]);

        // test_err optional_member
        // class B { foo?; }
        if TypeScript.is_supported(p) {
            Ok(Some(range))
        } else {
            let err = p
                .err_builder("`?` modifiers can only be used in TypeScript files")
                .primary(range, "");

            p.error(err);
            Err(range)
        }
    } else {
        Ok(None)
    }
}

// test_err class_property_initializer
// class B { lorem = ; }
pub(crate) fn parse_initializer_clause(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    if p.at(T![=]) {
        let m = p.start();
        p.bump(T![=]);

        parse_assignment_expression_or_higher(p, context)
            .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

        Present(m.complete(p, JS_INITIALIZER_CLAUSE))
    } else {
        Absent
    }
}

fn parse_method_class_member(
    p: &mut Parser,
    m: Marker,
    modifiers: &mut ClassMemberModifiers,
    flags: SignatureFlags,
) -> CompletedMarker {
    parse_class_member_name(p, modifiers)
        .or_add_diagnostic(p, js_parse_error::expected_class_member_name);
    parse_method_class_member_rest(p, m, modifiers, flags)
}

// test_err class_member_method_parameters
// class B { foo(a {} }

// test ts ts_method_class_member
// class Test {
//   test<A, B extends A, R>(a: A, b: B): R {}
// }

/// Parses the body (everything after the identifier name) of a method class member
/// that includes: parameters and its types, return type and method body
fn parse_method_class_member_rest(
    p: &mut Parser,
    m: Marker,
    modifiers: &ClassMemberModifiers,
    flags: SignatureFlags,
) -> CompletedMarker {
    // test ts ts_optional_method_class_member
    // class A { test?() {} }
    let optional = optional_member_token(p);

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_type_parameters, |p, marker| {
            ts_only_syntax_error(p, "type parameters", marker.range(p))
        })
        .ok();

    let parameter_context = if modifiers.is_signature() {
        ParameterContext::Declaration
    } else {
        // Not perfect. It may turn out that this is a method overload without a body in which case
        // this isn't an implementation.
        ParameterContext::Implementation
    };

    parse_parameter_list(p, parameter_context, flags)
        .or_add_diagnostic(p, js_parse_error::expected_class_parameters);

    TypeScript
        .parse_exclusive_syntax(p, parse_ts_return_type_annotation, |p, annotation| {
            ts_only_syntax_error(p, "return type annotation", annotation.range(p))
        })
        .ok();

    let member_kind = expect_method_body(p, &m, modifiers, ClassMethodMemberKind::Method(flags));
    let mut member = m.complete(p, member_kind.as_method_syntax_kind());

    let is_async = flags.contains(SignatureFlags::ASYNC);

    // test_err ts typescript_abstract_classes_invalid_abstract_async_member
    // abstract class B { abstract async a(); }
    if modifiers.has(ModifierKind::Abstract) && is_async {
        let err = ts_parse_error::abstract_member_cannot_be_async(
            p,
            &modifiers.get_first_range_unchecked(ModifierKind::Abstract),
        );
        p.error(err);
        member.change_to_unknown(p);
    } else if flags.contains(SignatureFlags::GENERATOR) && member_kind.is_signature() {
        // test_err ts ts_method_signature_generator
        // declare class A { * method(); }
        // abstract class B { abstract * method(); }
        // class C {
        //      * overload();
        //      * overload() {}
        // }
        p.error(
            p.err_builder("A method signature cannot be declared as a generator.")
                .primary(member.range(p), ""),
        );
    } else if p.state.in_ambient_context() && is_async {
        // test_err ts ts_ambient_async_method
        // declare class A { async method(); }
        p.error(
            p.err_builder("'async' modifier cannot be used in an ambient context.")
                .primary(member.range(p), ""),
        );
        member.change_to_unknown(p);
    } else if optional.is_err() {
        // error already emitted by `optional_member_token()`
        member.change_to_unknown(p);
    }

    member
}

#[derive(Debug)]
enum MemberKind {
    Signature,
    Declaration,
}

impl MemberKind {
    const fn is_signature(&self) -> bool {
        matches!(self, MemberKind::Signature)
    }

    const fn as_method_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_METHOD_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_METHOD_CLASS_MEMBER,
        }
    }

    const fn as_constructor_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_CONSTRUCTOR_CLASS_MEMBER,
        }
    }

    const fn as_setter_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_SETTER_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_SETTER_CLASS_MEMBER,
        }
    }

    const fn as_getter_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_GETTER_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_GETTER_CLASS_MEMBER,
        }
    }
}

#[derive(Debug)]
enum ClassMethodMemberKind {
    /// `get` or `set`
    Accessor,

    /// A class's constructor function
    Constructor,

    /// Any other regular method
    Method(SignatureFlags),
}

impl ClassMethodMemberKind {
    /// The body of methods is optional if it's a method overload definition
    /// ```ts
    /// class Test {
    ///   method();
    ///   method() { ... }
    /// }
    /// ```
    const fn is_body_optional(&self) -> bool {
        matches!(
            self,
            ClassMethodMemberKind::Method(_) | ClassMethodMemberKind::Constructor
        )
    }

    const fn is_constructor(&self) -> bool {
        matches!(self, ClassMethodMemberKind::Constructor)
    }

    const fn signature_flags(&self) -> SignatureFlags {
        match self {
            ClassMethodMemberKind::Method(flags) => *flags,
            ClassMethodMemberKind::Constructor => SignatureFlags::CONSTRUCTOR,
            ClassMethodMemberKind::Accessor => SignatureFlags::empty(),
        }
    }
}

/// Parses the body of a method (constructor, getter, setter, or regular method).
///
/// TypeScript supports method signatures. These are methods without a body (and are terminated by a semicolon or ASI).
/// A method is a signature if one of the following applies
/// * The member has an `abstract` modifier (not supported by constructors)
/// * It's a declaration in an ambient context (`declare class { ... }` or `declare namespace { class { ... } }`).
/// * It's a method overload (doesn't apply to getters/setters)
///
/// The method determines which case applies to the current member and emits a diagnostic if:
/// * the body is absent for a method declaration
/// * the body is present for a method signature
/// * a method signature isn't terminate by a semicolon or ASI
///
/// The method returns the inferred kind (signature or declaration) of the parsed method body
fn expect_method_body(
    p: &mut Parser,
    member_marker: &Marker,
    modifiers: &ClassMemberModifiers,
    method_kind: ClassMethodMemberKind,
) -> MemberKind {
    let body = parse_function_body(p, method_kind.signature_flags());

    // test ts typescript_members_can_have_no_body_in_ambient_context
    // declare class Test {
    //     constructor();
    //     name();
    //     get test(): string;
    //     set test(v);
    // }
    // declare namespace n {
    //      class Test {
    //          constructor()
    //          name()
    //          get test(): string
    //          set test(v)
    //      }
    // }

    // test_err ts typescript_members_with_body_in_ambient_context_should_err
    // declare class Test {
    //     constructor() {}
    //     name() {}
    //     get test(): string { return ""; }
    //     set test(v) {}
    // }
    // declare namespace n {
    //      class Test {
    //          constructor() {}
    //          name() {}
    //          get test(): string { return ""; }
    //          set test(v) {}
    //      }
    // }
    if p.state.in_ambient_context() {
        match body {
            Present(body) => p.error(unexpected_body_inside_ambient_context(p, body.range(p))),
            Absent => {
                // test_err ts ts_ambient_context_semi
                // declare class A { method() method2() method3() }
                expect_member_semi(p, member_marker, "method declaration")
            }
        }
        MemberKind::Signature
    }
    // test_err ts typescript_abstract_class_member_should_not_have_body
    // abstract class AbstractMembers {
    //     abstract constructor() { }
    //     abstract display(): void { }
    //     abstract get my_name() { }
    //     abstract set my_name(name) { }
    //     abstract #private_name() { }
    // }
    else if modifiers.has(ModifierKind::Abstract) && !method_kind.is_constructor() {
        match body {
            Present(body) => p.error(unexpected_abstract_member_with_body(p, body.range(p))),
            Absent => {
                // test_err ts ts_abstract_member_ansi
                // abstract class A { abstract constructor() abstract method() abstract get getter() abstract set setter(value) abstract prop }
                expect_member_semi(p, member_marker, "method declaration")
            }
        }
        MemberKind::Signature
    }
    // test ts ts_method_and_constructor_overload
    // class Test {
    //      constructor();
    //      constructor(a: String) // ASI
    //      constructor(a?: String) {}
    //      async method(): Promise<String>;
    //      method(a: String): Promise<String> // ASI
    //      async method(a?: String): Promise<String> { return "test" }
    // }
    else if method_kind.is_body_optional()
        && TypeScript.is_supported(p)
        && body.is_absent()
        && optional_semi(p)
    {
        MemberKind::Signature
    } else {
        // test_err ts ts_method_members_with_missing_body
        // class Test {
        //      constructor() method() get test()
        //      set test(value)
        // }
        body.or_add_diagnostic(p, js_parse_error::expected_class_method_body);
        MemberKind::Declaration
    }
}

// test_err getter_class_no_body
// class Getters {
//   get foo()
// }

// test_err setter_class_no_body
// class Setters {
//   set foo(a)
// }
fn expect_accessor_body(
    p: &mut Parser,
    member_marker: &Marker,
    modifiers: &ClassMemberModifiers,
) -> MemberKind {
    expect_method_body(p, member_marker, modifiers, ClassMethodMemberKind::Accessor)
}

fn parse_constructor_class_member_body(
    p: &mut Parser,
    member_marker: Marker,
    modifiers: &ClassMemberModifiers,
) -> CompletedMarker {
    if let Ok(Some(range)) = optional_member_token(p) {
        let err = p
            .err_builder("constructors cannot be optional")
            .primary(range, "");

        p.error(err);
    }

    // test_err ts ts_constructor_type_parameters
    // class A { constructor<A>(b) {} }
    if let Present(type_parameters) = parse_ts_type_parameters(p) {
        p.error(ts_constructor_type_parameters_error(p, &type_parameters));
    }

    parse_constructor_parameter_list(p)
        .or_add_diagnostic(p, js_parse_error::expected_constructor_parameters);

    if let Present(marker) = parse_ts_type_annotation(p) {
        let err = p
            .err_builder("constructors cannot have type annotations")
            .primary(marker.range(p), "");

        p.error(err);
    }

    let constructor_kind = expect_method_body(
        p,
        &member_marker,
        modifiers,
        ClassMethodMemberKind::Constructor,
    );

    member_marker.complete(p, constructor_kind.as_constructor_syntax_kind())
}

fn parse_constructor_parameter_list(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    parse_parameters_list(
        p,
        SignatureFlags::empty(),
        parse_constructor_parameter,
        JS_CONSTRUCTOR_PARAMETER_LIST,
    );
    Present(m.complete(p, JS_CONSTRUCTOR_PARAMETERS))
}

// test_err js_constructor_parameter_reserved_names
// // SCRIPT
// class A { constructor(readonly, private, protected, public) {} }
fn parse_constructor_parameter(p: &mut Parser, context: ExpressionContext) -> ParsedSyntax {
    // test_err class_constructor_parameter
    // class B { constructor(protected b) {} }

    if is_nth_at_modifier(p, 0, true) {
        // test ts ts_property_parameter
        // class A { constructor(private x, protected y, public z) {} }
        // class B { constructor(readonly w, private readonly x, protected readonly y, public readonly z) {} }
        // class C { constructor(private x: string, readonly y?, z = "default", ...rest) {} }
        //
        // test_err ts ts_property_parameter_pattern
        // class A { constructor(private { x, y }, protected [a, b]) {} }
        let property_parameter = p.start();

        // test_err class_constructor_parameter_readonly
        // class B { constructor(readonly b) {} }

        let modifiers = parse_class_member_modifiers(p, true);

        parse_formal_parameter(p, ParameterContext::ParameterProperty, context)
            .or_add_diagnostic(p, expected_binding);

        let kind = if modifiers.validate_and_complete(p, TS_PROPERTY_PARAMETER) {
            TS_PROPERTY_PARAMETER
        } else {
            JS_UNKNOWN_PARAMETER
        };

        Present(property_parameter.complete(p, kind))
    } else {
        parse_any_parameter(p, ParameterContext::Implementation, context).map(|mut parameter| {
            // test_err ts ts_constructor_this_parameter
            // class C { constructor(this) {} }
            if parameter.kind() == TS_THIS_PARAMETER {
                p.error(
                    p.err_builder("A constructor cannot have a 'this' parameter.")
                        .primary(parameter.range(p), ""),
                );
                parameter.change_to_unknown(p);
            }
            parameter
        })
    }
}

fn is_at_class_member_name(p: &mut Parser, offset: usize) -> bool {
    matches!(p.nth(offset), T![#] | T!['[']) || is_at_literal_member_name(p, offset)
}

/// Parses a `JsAnyClassMemberName` and returns its completion marker
fn parse_class_member_name(p: &mut Parser, modifiers: &mut ClassMemberModifiers) -> ParsedSyntax {
    modifiers.set_private_member_name(p.at(T![#]));
    match p.cur() {
        T![#] => parse_private_class_member_name(p),
        T!['['] => parse_computed_member_name(p),
        _ => parse_literal_member_name(p),
    }
}

pub(crate) fn parse_private_class_member_name(p: &mut Parser) -> ParsedSyntax {
    parse_private_name(p).map(|mut name| {
        name.change_kind(p, JS_PRIVATE_CLASS_MEMBER_NAME);
        name
    })
}

fn is_at_method_class_member(p: &mut Parser, mut offset: usize) -> bool {
    if p.nth_at(offset, T![?]) {
        offset += 1;
    }

    p.nth_at(offset, T!['(']) || p.nth_at(offset, T![<])
}

pub(crate) fn is_nth_at_modifier(p: &mut Parser, n: usize, constructor_parameter: bool) -> bool {
    // Test if this modifier is followed by another modifier, member name or any other token that
    // starts a new member. If that's the case, then this is fairly likely a modifier. If not, then
    // this is probably not a modifier, but the name of the member. For example, all these are valid
    // members: `static() {}, private() {}, protected() {}`... but are modifiers if followed by another modifier or a name:
    // `static x() {} private static() {}`...
    if !matches!(
        p.nth(n),
        T![declare]
            | T![public]
            | T![protected]
            | T![private]
            | T![override]
            | T![static]
            | T![readonly]
            | T![abstract]
    ) {
        return false;
    }

    if p.has_nth_preceding_line_break(n + 1) {
        return false;
    }

    let followed_by_any_member = is_at_class_member_name(p, n + 1);
    let followed_by_class_member = !constructor_parameter && p.nth_at(n + 1, T![*]);
    let followed_by_parameter = constructor_parameter && matches!(p.nth(n + 1), T!['{'] | T!['[']);

    followed_by_any_member || followed_by_class_member || followed_by_parameter
}

// test static_generator_constructor_method
// class A {
// 	static async * constructor() {}
// 	static * constructor() {}
// }
fn is_at_constructor(p: &Parser, modifiers: &ClassMemberModifiers) -> bool {
    !modifiers.has(ModifierKind::Static)
        && (p.at(T![constructor]) || matches!(p.cur_src(), "\"constructor\"" | "'constructor'"))
}

// test class_member_modifiers
// class A { public() {} }
// class A { static protected() {} }
// class A { static }

/// Parses all possible modifiers regardless of what the current member is. It's up to the caller
/// to create diagnostics for not allowed modifiers.
fn parse_class_member_modifiers(
    p: &mut Parser,
    constructor_parameter: bool,
) -> ClassMemberModifiers {
    let mut modifiers = ClassMemberModifierList::default();
    let list = p.start();
    let mut progress = ParserProgress::default();
    let mut flags = ModifierFlags::empty();

    while let Some(modifier) = parse_modifier(p, constructor_parameter) {
        progress.assert_progressing(p);
        flags |= modifier.kind.as_flags();
        modifiers.add_modifier(modifier);
    }

    // It's unclear what kind of list this is supposed to be at this moment.
    // Create an `UNKNOWN` node. The list type gets changed later on by calling
    // `complete` or `abandon` when the member kind is known,
    let list = list.complete(p, JS_UNKNOWN);
    ClassMemberModifiers::new(modifiers, list, flags)
}

// test_err class_declare_method
// class B { declare fn() {} }
//
// test_err class_member_modifier
// class A { abstract foo; }
fn parse_modifier(p: &mut Parser, constructor_parameter: bool) -> Option<ClassMemberModifier> {
    if !is_nth_at_modifier(p, 0, constructor_parameter) {
        // all modifiers can also be valid member names. That's why we shouldn't parse a modifier
        // if it isn't followed by a valid member name or another modifier
        return None;
    }

    let modifier_kind = match p.cur() {
        T![declare] => ModifierKind::Declare,
        T![public] => ModifierKind::Public,
        T![protected] => ModifierKind::Protected,
        T![private] => ModifierKind::Private,
        T![override] => ModifierKind::Override,
        T![static] => ModifierKind::Static,
        T![readonly] => ModifierKind::Readonly,
        T![abstract] => ModifierKind::Abstract,
        _ => {
            return None;
        }
    };

    let m = p.start();
    let range = p.cur_range();
    p.bump_any();
    m.complete(p, modifier_kind.as_syntax_kind());

    Some(ClassMemberModifier {
        start: range.start(),
        length: u32::from(range.len()) as u8,
        kind: modifier_kind,
    })
}

bitflags! {
    /// Bitflag of class member modifiers.
    /// Useful to cheaply track all already seen modifiers of a member (instead of using a HashSet<ModifierKind>).
    #[derive(Default)]
    struct ModifierFlags: u16 {
        const DECLARE       = 1 << 0;
        const PRIVATE       = 1 << 1;
        const PROTECTED     = 1 << 2;
        const PUBLIC        = 1 << 3;
        const STATIC        = 1 << 4;
        const READONLY      = 1 << 5;
        const ABSTRACT      = 1 << 6;
        const OVERRIDE      = 1 << 7;
        const PRIVATE_NAME  = 1 << 8;

        const ACCESSIBILITY = ModifierFlags::PRIVATE.bits | ModifierFlags::PROTECTED.bits | ModifierFlags::PUBLIC.bits;
    }
}

/// The different modifiers a class member may have.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum ModifierKind {
    Declare,
    Abstract,
    Private,
    Protected,
    Public,
    Static,
    Readonly,
    Override,
}

impl ModifierKind {
    const fn is_ts_modifier(&self) -> bool {
        !matches!(self, ModifierKind::Static)
    }

    const fn as_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            ModifierKind::Declare => TS_DECLARE_MODIFIER,
            ModifierKind::Abstract => TS_ABSTRACT_MODIFIER,
            ModifierKind::Private | ModifierKind::Protected | ModifierKind::Public => {
                TS_ACCESSIBILITY_MODIFIER
            }
            ModifierKind::Static => JS_STATIC_MODIFIER,
            ModifierKind::Readonly => TS_READONLY_MODIFIER,
            ModifierKind::Override => TS_OVERRIDE_MODIFIER,
        }
    }

    const fn as_flags(&self) -> ModifierFlags {
        match self {
            ModifierKind::Declare => ModifierFlags::DECLARE,
            ModifierKind::Abstract => ModifierFlags::ABSTRACT,
            ModifierKind::Private => ModifierFlags::PRIVATE,
            ModifierKind::Protected => ModifierFlags::PROTECTED,
            ModifierKind::Public => ModifierFlags::PUBLIC,
            ModifierKind::Static => ModifierFlags::STATIC,
            ModifierKind::Readonly => ModifierFlags::READONLY,
            ModifierKind::Override => ModifierFlags::OVERRIDE,
        }
    }
}

/// Stores the range of a parsed modifier with its kind
#[derive(Debug, Clone)]
struct ClassMemberModifier {
    kind: ModifierKind,

    // The start position of the modifier in the source text
    start: TextSize,

    // The length of the modifier text. Storage optimization because none of the modifiers exceeds
    // a length of 128 (even if encoded)
    length: u8,
}

impl ClassMemberModifier {
    fn as_text_range(&self) -> TextRange {
        TextRange::new(
            self.start,
            self.start.add(TextSize::from(self.length as u32)),
        )
    }
}

// 4 is sufficient to store all valid modifiers without requiring any heap allocations.
#[derive(Debug, Default)]
struct ClassMemberModifierList(SmallVec<[ClassMemberModifier; 4]>);

impl ClassMemberModifierList {
    /// Sets the range of a parsed modifier
    fn add_modifier(&mut self, modifier: ClassMemberModifier) {
        self.0.push(modifier);
    }

    fn iter(&self) -> Iter<'_, ClassMemberModifier> {
        self.0.iter()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Stores the kind of parsed modifiers with their ranges.
#[derive(Debug)]
#[must_use]
struct ClassMemberModifiers {
    modifiers: ClassMemberModifierList,
    // Stores all added flags. Useful to determine with `O(1)` if a modifier exists in the list or not
    flags: ModifierFlags,
    list_marker: CompletedMarker,
    bomb: DebugDropBomb,
}

impl ClassMemberModifiers {
    fn new(
        modifiers: ClassMemberModifierList,
        list_marker: CompletedMarker,
        flags: ModifierFlags,
    ) -> Self {
        Self {
            modifiers,
            list_marker,
            flags,
            bomb: DebugDropBomb::new("list must either be 'completed' or 'abandoned' by calling 'complete' or 'abandon'.")
        }
    }

    fn set_private_member_name(&mut self, private_name: bool) {
        self.flags.set(ModifierFlags::PRIVATE_NAME, private_name);
    }

    /// Returns true if the passed in modifier is present in the source.
    fn has(&self, kind: ModifierKind) -> bool {
        self.flags.contains(kind.as_flags())
    }

    fn is_signature(&self) -> bool {
        self.has(ModifierKind::Abstract) || self.has(ModifierKind::Declare)
    }

    /// Returns the range for the passed in modifier or [None] if the modifier isn't set or is a missing marker
    fn get_first_range(&self, kind: ModifierKind) -> Option<TextRange> {
        if self.flags.contains(kind.as_flags()) {
            self.modifiers
                .iter()
                .find(|m| m.kind == kind)
                .map(|m| m.as_text_range())
        } else {
            None
        }
    }

    /// Returns the range of the passed in modifier.
    ///
    /// ## Safety
    /// Expected that the caller first checked that such a modifier is present (Either by using `has`
    /// or by iterating over all modifiers and keeping track of the modifier it has seen).
    fn get_first_range_unchecked(&self, kind: ModifierKind) -> TextRange {
        self.get_first_range(kind)
            .unwrap_or_else(|| panic!("Expected modifier of kind {:?} to be present", kind))
    }

    fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    /// Abandons the marker for the modifier list
    ///
    /// ## Panics
    /// If the modifier list isn't empty
    fn abandon(mut self, p: &mut Parser) {
        debug_assert!(self.is_empty());
        self.list_marker.undo_completion(p).abandon(p);
        self.bomb.defuse();
    }

    /// Validates if these modifiers are valid for a member of the given kind and
    /// completes the modifier list.
    ///
    /// Returns `true` if all modifiers are valid.
    fn validate_and_complete(mut self, p: &mut Parser, member_kind: JsSyntaxKind) -> bool {
        self.bomb.defuse();

        let list_kind = match member_kind {
            JS_PROPERTY_CLASS_MEMBER => JS_PROPERTY_MODIFIER_LIST,
            TS_PROPERTY_SIGNATURE_CLASS_MEMBER => TS_PROPERTY_SIGNATURE_MODIFIER_LIST,
            JS_GETTER_CLASS_MEMBER | JS_SETTER_CLASS_MEMBER | JS_METHOD_CLASS_MEMBER => {
                JS_METHOD_MODIFIER_LIST
            }
            TS_GETTER_SIGNATURE_CLASS_MEMBER
            | TS_SETTER_SIGNATURE_CLASS_MEMBER
            | TS_METHOD_SIGNATURE_CLASS_MEMBER => TS_METHOD_SIGNATURE_MODIFIER_LIST,
            JS_CONSTRUCTOR_CLASS_MEMBER | TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER => {
                JS_CONSTRUCTOR_MODIFIER_LIST
            }
            TS_INDEX_SIGNATURE_CLASS_MEMBER => TS_INDEX_SIGNATURE_MODIFIER_LIST,
            TS_PROPERTY_PARAMETER => TS_PROPERTY_PARAMETER_MODIFIER_LIST,
            JS_UNKNOWN_MEMBER | JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                // Error recovery kicked in. There's no "right" list to pick in this case, let's just remove it
                self.list_marker.undo_completion(p).abandon(p);
                return false;
            }
            t => panic!("Unknown member kind {:?}", t),
        };

        self.list_marker.change_kind(p, list_kind);

        let mut preceding_modifiers = ModifierFlags::empty();
        let mut valid = true;

        for modifier in self.modifiers.iter() {
            if let Some(diagnostic) =
                self.check_class_member_modifier(p, modifier, preceding_modifiers, member_kind)
            {
                p.error(diagnostic);
                valid = false;
            }

            preceding_modifiers |= modifier.kind.as_flags(); // Keep track of the seen modifiers
        }

        valid
    }

    // test ts ts_class_property_member_modifiers
    // class Base {
    //   base1;
    // }
    // abstract class Test extends Base {
    //     declare a: string;
    //     private declare c: string;
    //     declare private d: string;
    //     declare private readonly e: string;
    //     private readonly declare f: string;
    //     declare private static readonly g: string;
    //     private static readonly declare h: string;
    //     protected readonly abstract i: string;
    //     protected abstract readonly j: string;
    //     protected override readonly base1: string;
    // }

    // test_err ts ts_class_modifier_precedence
    // class Base { base1; base2; base3; base4;}
    // abstract class Test extends Base {
    //     // Accessibility
    //     readonly private a: string;
    //     override protected base1;
    //     static private b: string;
    //     abstract protected d: string;
    //     // Static
    //     readonly static c: string;
    //     override static base2: string;
    //     // abstract
    //     override abstract base3: string;
    //     // override
    //     readonly override base4: string;
    // }

    // test_err ts ts_class_invalid_modifier_combinations
    // class Base { base1; base2; }
    // abstract class Test extends Base {
    //     override override base1;
    //     declare declare a;
    //     private protected public c;
    //     private private d;
    //     protected protected e;
    //     public public f;
    //     abstract abstract g;
    //     static static h;
    //     readonly readonly i;
    //     override declare base2;
    //     private abstract j;
    //     abstract #j;
    //     abstract static k;
    //     abstract async l();
    //     declare async m();
    //     declare #l;
    // }

    // test_err class_invalid_modifiers
    // class A { public foo() {} }
    // class B { static static foo() {} }
    fn check_class_member_modifier(
        &self,
        p: &Parser,
        modifier: &ClassMemberModifier,
        preceding_modifiers: ModifierFlags,
        member_kind: JsSyntaxKind,
    ) -> Option<Diagnostic> {
        // test_err index_signature_class_member_in_js
        // class A {
        //     [a: number]: string;
        // }
        if TypeScript.is_unsupported(p) && modifier.kind.is_ts_modifier() {
            return Some(
                p.err_builder(&format!(
                    "'{}' modifier can only be used in TypeScript files",
                    p.source(modifier.as_text_range())
                ))
                .primary(modifier.as_text_range(), ""),
            );
        }

        // test_err ts ts_index_signature_class_member_cannot_have_visibility_modifiers
        // class A {
        //     public  [a: number]: string;
        // }
        // class B {
        //     private  [a: number]: string;
        // }
        // class C {
        //     protected  [a: number]: string;
        // }

        // test_err ts ts_index_signature_class_member_cannot_be_abstract
        // abstract class A {
        //     abstract [a: number]: string;
        // }
        if member_kind == TS_INDEX_SIGNATURE_CLASS_MEMBER
            && !matches!(modifier.kind, ModifierKind::Static | ModifierKind::Readonly)
        {
            return Some(
                p.err_builder(&format!(
                    "'{}' modifier cannot appear on an index signature.",
                    p.source(modifier.as_text_range())
                ))
                .primary(modifier.as_text_range(), ""),
            );
        } else if matches!(
            member_kind,
            JS_CONSTRUCTOR_CLASS_MEMBER | TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER
        ) && !matches!(
            modifier.kind,
            ModifierKind::Private | ModifierKind::Protected | ModifierKind::Public
        ) {
            return Some(ts_modifier_cannot_appear_on_a_constructor_declaration(
                p,
                modifier.as_text_range(),
            ));
        } else if
        // test ts class_constructor_parameter_modifiers
        // class Base { name!: string; other!: string }
        // class Sub extends Base {
        //  constructor(private priv: string, protected prot: string, public pub: string, override name: string, readonly read: string, protected override readonly other: string) {
        //      super();
        //  }
        // }
        member_kind == TS_PROPERTY_PARAMETER
            && !matches!(
                modifier.kind,
                ModifierKind::Private
                    | ModifierKind::Protected
                    | ModifierKind::Public
                    | ModifierKind::Override
                    | ModifierKind::Readonly
            )
        {
            return Some(ts_modifier_cannot_appear_on_a_parameter(
                p,
                modifier.as_text_range(),
            ));
        }

        match modifier.kind {
            ModifierKind::Readonly => {
                if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                } else if !matches!(
                    member_kind,
                    JS_PROPERTY_CLASS_MEMBER
                        | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                        | TS_INDEX_SIGNATURE_CLASS_MEMBER
                        | JS_UNKNOWN_MEMBER
                        | TS_PROPERTY_PARAMETER
                ) {
                    // test_err ts ts_readonly_modifier_non_class_or_indexer
                    // class Test {
                    //   readonly constructor() {}
                    //   readonly method() {}
                    //   readonly get test() { return "a"; }
                    //   readonly set test(value: string) {}
                    // }
                    return Some(
                        p.err_builder(
                            "Readonly can only appear on a property declaration or index signature."
                        )
                            .primary(modifier.as_text_range(), ""),
                    );
                }
            }
            ModifierKind::Declare => {
                // test_err ts ts_class_declare_modifier_error
                // class Test {
                //     declare method(): string;
                //     declare constructor(declare readonly prop) {}
                //     declare get test() { return "a" }
                //     declare set test(value: string) {}
                //     declare [name: string]: string;
                // }
                if preceding_modifiers.contains(ModifierFlags::DECLARE) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Declare),
                    ));
                } else if self.flags.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if !matches!(
                    member_kind,
                    JS_PROPERTY_CLASS_MEMBER | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                ) {
                    return Some(
                        p.err_builder("'declare' modifier is only allowed on properties.")
                            .primary(modifier.as_text_range(), ""),
                    );
                } else if self.flags.contains(ModifierFlags::PRIVATE_NAME) {
                    // test_err ts ts_declare_property_private_name
                    // class A { declare #name(); };
                    return Some(
                        p.err_builder(
                            "'declare' modifier cannot be used with a private identifier'.",
                        )
                        .primary(modifier.as_text_range(), ""),
                    );
                }
            }
            ModifierKind::Abstract => {
                if preceding_modifiers.contains(ModifierFlags::ABSTRACT) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Abstract),
                    ));
                } else if !matches!(
                    member_kind,
                    JS_METHOD_CLASS_MEMBER
                        | TS_METHOD_SIGNATURE_CLASS_MEMBER
                        | JS_PROPERTY_CLASS_MEMBER
                        | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                        | JS_SETTER_CLASS_MEMBER
                        | TS_SETTER_SIGNATURE_CLASS_MEMBER
                        | JS_GETTER_CLASS_MEMBER
                        | TS_GETTER_SIGNATURE_CLASS_MEMBER
                        | JS_UNKNOWN_MEMBER
                ) {
                    return Some(
                        p.err_builder("'abstract' modifier can only appear on a class, method or property declaration.")
                            .primary(modifier.as_text_range(), "")
                    );
                } else if self.flags.contains(ModifierFlags::STATIC) {
                    // test_err ts typescript_abstract_classes_invalid_static_abstract_member
                    // abstract class A { abstract static fn1(); }
                    // abstract class B { static abstract fn1(); }
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Static),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if self.flags.contains(ModifierFlags::PRIVATE_NAME) {
                    // test_err ts typescript_abstract_classes_invalid_abstract_private_member
                    // abstract class A { abstract #name(); };
                    return Some(
                        p.err_builder(
                            "'abstract' modifier cannot be used with a private identifier'.",
                        )
                        .primary(modifier.as_text_range(), ""),
                    );
                }
            }
            ModifierKind::Private | ModifierKind::Protected | ModifierKind::Public => {
                if preceding_modifiers.intersects(ModifierFlags::ACCESSIBILITY) {
                    let range = if preceding_modifiers.contains(ModifierFlags::PRIVATE) {
                        self.get_first_range_unchecked(ModifierKind::Private)
                    } else if preceding_modifiers.contains(ModifierFlags::PROTECTED) {
                        self.get_first_range_unchecked(ModifierKind::Protected)
                    } else {
                        self.get_first_range_unchecked(ModifierKind::Public)
                    };

                    return Some(ts_accessibility_modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        range,
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::STATIC) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Static),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                } else if self.flags.contains(ModifierFlags::ABSTRACT)
                    && modifier.kind == ModifierKind::Private
                {
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Abstract),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::ABSTRACT) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Abstract),
                    ));
                } else if self.flags.contains(ModifierFlags::PRIVATE_NAME) {
                    // test_err ts typescript_classes_invalid_accessibility_modifier_private_member
                    // class A { private #name; protected #other; public #baz; };
                    return Some(
                        p.err_builder(
                            "An accessibility modifier cannot be used with a private identifier.",
                        )
                        .primary(modifier.as_text_range(), ""),
                    );
                }
            }
            ModifierKind::Static => {
                if preceding_modifiers.contains(ModifierFlags::STATIC) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Static),
                    ));
                }
                // test_err ts ts_index_signature_class_member_static_readonly_precedence
                // class A {
                //     readonly static [a: number]: string;
                // }
                else if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                }
            }
            ModifierKind::Override => {
                if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                }
            }
        }

        None
    }
}
