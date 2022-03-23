pub mod jsx_parse_errors;

use rome_js_syntax::JsSyntaxKind::*;

use crate::lexer::{JsSyntaxKind, LexContext, ReLexContext, T};
use crate::syntax::expr::{parse_expression, parse_name, ExpressionContext};
use crate::syntax::js_parse_error::{expected_expression, expected_identifier};
use crate::syntax::jsx::jsx_parse_errors::{
    jsx_expected_attribute, jsx_expected_attribute_value, jsx_expected_children,
};
use crate::{parser::RecoveryResult, ParseNodeList, ParseRecovery, ParsedSyntax, Parser};
use crate::{Absent, Checkpoint, Present};

use super::typescript::parse_ts_type_arguments;

// Constraints function to be inside a checkpointed parser
// allowing them advancing and abandoning the parser.
struct CheckpointedParser<'a, 'b> {
    parser: &'a mut Parser<'b>,
    checkpoint: Checkpoint,
}

impl<'a, 'b> CheckpointedParser<'a, 'b> {
    pub fn new(p: &'a mut Parser<'b>) -> CheckpointedParser<'a, 'b> {
        let checkpoint = p.checkpoint();
        Self {
            parser: p,
            checkpoint,
        }
    }

    pub fn rewind(self) -> &'a mut Parser<'b> {
        self.parser.rewind(self.checkpoint);
        self.parser
    }
}

impl<'a, 'b> std::ops::Deref for CheckpointedParser<'a, 'b> {
    type Target = Parser<'b>;

    fn deref(&self) -> &Self::Target {
        self.parser
    }
}

impl<'a, 'b> std::ops::DerefMut for CheckpointedParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parser
    }
}

// It is impossible to lookahead and guarantee that we are at a JSX expression,
// so this function will checkpoint and rewind the parser on failures.
pub(super) fn maybe_parse_jsx_expression(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return ParsedSyntax::Absent;
    }

    let mut p = CheckpointedParser::new(p);
    let syntax = parse_jsx_tag_expression(&mut p);

    if syntax.is_absent() {
        p.rewind();
    }

    syntax
}

// test jsx jsx_element_on_return
// function f() {
//     return <div></div>
// }

// test jsx jsx_element_on_arrow_function
// const f = () => <div></div>;
// const f = () => (<div></div>);

// test jsx jsx_element_as_statements
// <div />

// test_err jsx_or_type_assertion
// function f() {
//     let a = <div>a</div>; // JSX
//     let b = <string>b; //type assertion
//     let c = <string>b<a>d; // type assertion
//     let d = <div>a</div>/; // ambigous: JSX or "type assertion a less than regex /div>/". Probably JSX.
//     let d = <string>a</string>/;
// }
fn parse_jsx_tag_expression(p: &mut CheckpointedParser<'_, '_>) -> ParsedSyntax {
    parse_jsx_element(p, true).map(|element| {
        let m = element.precede(p);
        m.complete(p, JSX_TAG_EXPRESSION)
    })
}

struct JsxChildrenList;

impl ParseNodeList for JsxChildrenList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        match p.cur() {
            // test jsx jsx_element_children
            // <a>
            //     <b>
            //        <d></d>
            //        <e></e>
            //     </b>
            //     <c></c>
            // </a>
            T![<] => parse_jsx_element(p, false),
            T!['{'] => parse_jsx_expression_block(p, ExpressionBlock::Children),
            // test jsx jsx_text
            // <a>test</a>;
            // <a>   whitespace handling </a>;
            // <a> multi
            //    line
            //          node
            // </a>;
            // <test>\u3333</test> // no error for invalid unicode escape
            JsSyntaxKind::JSX_TEXT_LITERAL => {
                let m = p.start();
                p.bump(JSX_TEXT_LITERAL);
                ParsedSyntax::Present(m.complete(p, JSX_TEXT))
            }
            _ => ParsedSyntax::Absent,
        }
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        let at_l_angle0 = p.at(T![<]);
        let at_slash1 = p.nth_at(1, T![/]);
        at_l_angle0 && at_slash1
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JsSyntaxKind::JS_UNKNOWN,
                token_set![T![<], T![>], T!['{'], T!['}']],
            ),
            jsx_expected_children,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JsSyntaxKind::JSX_CHILD_LIST
    }
}

// test jsx jsx_element_open_close
// function f() {
//     return <div></div>
// }

// test jsx jsx_element_self_close
// function f() {
//     return <div />
// }

// test jsx jsx_closing_token_trivia
// <closing / /* some comment */ >;
// <open><
// /* some comment */ / open>;

// test_err jsx jsx_invalid_text
// <a> test ></a>;
// <b> invalid }</b>;

/// Parses a JSX element
///
/// `in_expression` must be `true` if this element is a direct child of the `JsxElementExpression` (root of an expression).
/// It should be false when parsing any child node.
fn parse_jsx_element(p: &mut Parser, in_expression: bool) -> ParsedSyntax {
    match parse_jsx_element_head_or_fragment(p, in_expression) {
        Present(opening_marker) if opening_marker.kind() == JsSyntaxKind::JSX_OPENING_ELEMENT => {
            let element_marker = opening_marker.precede(p);

            parse_jsx_element_children(p);

            let closing_marker = parse_jsx_closing_element(p, in_expression);
            if closing_marker.is_absent() {
                element_marker.abandon(p);
                Absent
            } else {
                Present(element_marker.complete(p, JsSyntaxKind::JSX_ELEMENT))
            }
        }
        Present(self_closing_marker)
            if self_closing_marker.kind() == JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT =>
        {
            Present(self_closing_marker)
        }
        Present(fragment) if fragment.kind() == JSX_FRAGMENT => Present(fragment),
        Absent => Absent,
        _ => unreachable!("Unexpected present node returned"),
    }
}

#[inline]
fn parse_jsx_element_children(p: &mut Parser) {
    JsxChildrenList.parse_list(p);
}

// <a ...> or <a ... />
// ^          ^

// test jsx jsx_fragments
// <></>;
// <>abcd</>;
// <>   whitespace
// </>;
// <
//   /*comment */
//   >
//   <
//   /
// >;
fn parse_jsx_element_head_or_fragment(p: &mut Parser, in_expression: bool) -> ParsedSyntax {
    if !p.at(T![<]) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();
    p.bump(T![<]);
    let name = parse_jsx_any_element_name(p);

    if name.is_absent() {
        if !p.at(T![>]) {
            m.abandon(p);
            return Absent;
        }

        p.bump_with_context(T![>], LexContext::JsxChild);
        parse_jsx_element_children(p);

        if !p.expect(T![<]) || !p.expect(T![/]) || !p.expect(T![>]) {
            m.abandon(p);
            return Absent;
        }

        return Present(m.complete(p, JSX_FRAGMENT));
    }

    // test tsx tsx_element_generics_type
    // <NonGeneric />;
    // <Generic<true> />;
    // <Generic<true>></Generic>;
    let _ = parse_ts_type_arguments(p);

    JsxAttributeList.parse_list(p);

    let kind = if p.at(T![/]) {
        p.bump(T![/]);
        JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT
    } else {
        JsSyntaxKind::JSX_OPENING_ELEMENT
    };

    if !p.at(T![>]) {
        m.abandon(p);
        return Absent;
    } else if in_expression && kind == JSX_SELF_CLOSING_ELEMENT {
        p.bump(T![>]);
    } else {
        p.bump_with_context(T![>], LexContext::JsxChild);
    }

    ParsedSyntax::Present(m.complete(p, kind))
}

// <a/>
// ^
fn parse_jsx_closing_element(p: &mut Parser, in_expression: bool) -> ParsedSyntax {
    if !p.at(T![<]) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();

    if !p.expect(T![<]) || !p.expect(T![/]) {
        m.abandon(p);
        return Absent;
    }

    let closing_name = parse_jsx_any_element_name(p);
    if closing_name.is_absent() {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    if !p.at(T![>]) {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    if in_expression {
        p.bump(T![>]);
    } else {
        p.bump_with_context(T![>], LexContext::JsxChild);
    }

    ParsedSyntax::Present(m.complete(p, JSX_CLOSING_ELEMENT))
}

// test jsx jsx_member_element_name
// <a.b.c.d></a.b.c.d>;
// <a-b.c></a-b.c>;
// <Abcd></Abcd>;
//
// test_err jsx jsx_namespace_member_element_name
// <namespace:a></namespace:a>;
// <namespace:a.b></namespace:a.b>;
fn parse_jsx_any_element_name(p: &mut Parser) -> ParsedSyntax {
    let name = parse_jsx_name_or_namespace(p);
    name.map(|mut name| {
        if name.kind() == JSX_NAME && (p.at(T![.]) || !is_intrinsic_element(name.text(p))) {
            name.change_kind(p, JSX_REFERENCE_IDENTIFIER)
        } else if name.kind() == JSX_NAMESPACE_NAME && p.at(T![.]) {
            let error = p
                .err_builder("JSX property access expressions cannot include JSX namespace names.")
                .primary(name.range(p), "");
            p.error(error);
            name.change_to_unknown(p);
        }

        while p.at(T![.]) {
            let m = name.precede(p);
            p.bump(T![.]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            name = m.complete(p, JSX_MEMBER_NAME)
        }

        name
    })
}

/// Tests if this is an intrinsic element name. Intrinsic elements are such elements
/// that are built in, for example HTML elements. This implementation uses React's semantic
/// and assumes that anything starting with a lower case character is an intrinsic element, and
/// that custom components start with an uper case character.
///
/// Resources: [TypeScript's documentation on intrinsic elements](https://www.typescriptlang.org/docs/handbook/jsx.html#intrinsic-elements)
fn is_intrinsic_element(element_name: &str) -> bool {
    if let Some(first) = element_name.chars().next() {
        first.is_lowercase()
    } else {
        false
    }
}

// test jsx jsx_any_name
// <a-b-c-d-e></a-b-c-d-e>;
// <a-b-c-d-e />;
// <if />;
// <namespace:name></namespace:name>;
// <dashed-namespaced:dashed-name />;
fn parse_jsx_name_or_namespace(p: &mut Parser) -> ParsedSyntax {
    parse_jsx_name(p).map(|identifier| {
        if p.at(T![:]) {
            let m = identifier.precede(p);
            p.bump(T![:]);
            parse_jsx_name(p).or_add_diagnostic(p, expected_identifier);
            m.complete(p, JSX_NAMESPACE_NAME)
        } else {
            identifier
        }
    })
}

fn parse_jsx_name(p: &mut Parser) -> ParsedSyntax {
    p.re_lex(ReLexContext::JsxIdentifier);

    if p.at(JSX_IDENT) {
        let name = p.start();
        p.bump(JSX_IDENT);
        Present(name.complete(p, JSX_NAME))
    } else {
        Absent
    }
}

struct JsxAttributeList;
// test jsx jsx_element_attributes
// function f() {
//     return <div string_literal="a" expression={1} novalue el=<a/>></div>;
// }
// <div dashed-name='test' use:validate="abcd" />;
// <div use-dashed_underscore:validate="ahaha" />;
// <div multiline-string='test
//   continues here' />;
// <div invalid-unicode-escape="\u10000\u20000" />;
impl ParseNodeList for JsxAttributeList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        if matches!(p.cur(), T!['{'] | T![...]) {
            parse_jsx_spread_attribute(p)
        } else {
            parse_jsx_attribute(p)
        }
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T![>]) || p.at(T![/])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JsSyntaxKind::JS_UNKNOWN_MEMBER,
                token_set![T![/], T![>], T![<], T!['{'], T!['}'], T![...], T![ident]],
            ),
            jsx_expected_attribute,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JsSyntaxKind::JSX_ATTRIBUTE_LIST
    }
}

fn parse_jsx_attribute(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();

    let name = parse_jsx_name_or_namespace(p);
    if name.is_absent() {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    let _ = parse_jsx_attribute_initializer_clause(p);

    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_ATTRIBUTE))
}

// test jsx jsx_spread_attribute
// let obj = {};
// <a {...obj} />;
//
// test_err jsx jsx_spread_attribute_error
// let obj = {};
// <a {...obj, other} />;
// <a ...obj} />;
// <a {obj} />;
// <div
//       {...{} /*
//       // @ts-ignore */ /* prettier-ignore */
//       invalidProp="HelloWorld"
//     />;
fn parse_jsx_spread_attribute(p: &mut Parser) -> ParsedSyntax {
    if !matches!(p.cur(), T![...] | T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.expect(T!['{']);
    p.expect(T![...]);

    let argument = parse_expression(p, ExpressionContext::default()).map(|mut expr| {
        if expr.kind() == JS_SEQUENCE_EXPRESSION {
            p.error(
                p.err_builder("Comma operator isn't a valid value for a JSX spread argument.")
                    .primary(expr.range(p), ""),
            );
            expr.change_to_unknown(p);
        }

        expr
    });

    argument.or_add_diagnostic(p, expected_expression);

    p.expect(T!['}']);

    Present(m.complete(p, JSX_SPREAD_ATTRIBUTE))
}

fn parse_jsx_attribute_initializer_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![=], LexContext::JsxAttributeValue);

    // test_err jsx jsx_element_attribute_missing_value
    // function f() {
    //     return <div string_literal= ></div>;
    // }
    parse_jsx_attribute_value(p).or_add_diagnostic(p, jsx_expected_attribute_value);

    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE))
}

fn parse_jsx_attribute_value(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        // test jsx jsx_element_attribute_expression
        // <div id={1} />;
        // <div className={prefix`none`} />;
        T!['{'] => parse_jsx_expression_block(p, ExpressionBlock::Attribute),
        // test jsx jsx_element_attribute_element
        // <div id=<a/> />;
        T![<] => parse_jsx_element(p, true),
        // test jsx jsx_element_attribute_string_literal
        // <div id="a" />;
        JsSyntaxKind::JSX_STRING_LITERAL => {
            let m = p.start();
            p.bump(JSX_STRING_LITERAL);
            ParsedSyntax::Present(m.complete(p, JSX_STRING))
        }
        _ => ParsedSyntax::Absent,
    }
}

#[derive(PartialEq, Eq)]
enum ExpressionBlock {
    Attribute,
    Children,
}

// test jsx jsx_children_expression
// let x;
// let a;
// let b;
// let key;
// let f = () => {};
// <div>
//   {1}
//   {9007199254740991n}
//   {""}
//   {true}
//   {null}
//   {undefined}
//   {/a/}
//   {[]}
//   {x => console.log(x)}
//   {x = 1}
//   {await x}
//   {1 + 1}
//   {f()}
//   {a[b]}
//   {a?1:2}
//   {function f() {}}
//   {function () {}}
//   {a}
//   {import("a.js")}
//   {key in a}
//   {a instanceof Object}
//   {a && b}
//   {new f()}
//   {{}}
//   {(a)}
//   {a++}
//   {++a}
//   {a,b}
//   {a.b}
//   {super.a()}
//   {this}
//   {delete a.a}
//   {void a}
//   {typeof a}
//   {+a}
//   {-a}
//   {!a}
//   {~a}
//   {``}
//   {/* A JSX comment */}
//   {/* Multi
//       line
//   */}
// </div>
// function *f() {
//     return <div>
//         {yield a}
//     </div>;
// }

// test_err jsx jsx_children_expressions_not_accepted
// <div>
//   {import.meta}
//   {class A{}}
//   {super()}
//   {new.target}
// </div>

fn parse_jsx_expression_block(p: &mut Parser, kind: ExpressionBlock) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();

    p.bump(T!['{']);

    // test jsx jsx_children_spread
    // <div>{...a}</div>
    if kind == ExpressionBlock::Children && p.at(T![...]) {
        p.expect(T![...]);
        let expr = super::expr::parse_expression(p, ExpressionContext::default()).map(|mut m| {
            match m.kind() {
                JsSyntaxKind::IMPORT_META
                | JsSyntaxKind::NEW_TARGET
                | JsSyntaxKind::JS_CLASS_EXPRESSION
                | JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                    let err = p
                        .err_builder("This expression is not valid as a JSX spread expression")
                        .primary(m.range(p), "");
                    p.error(err);
                    m.change_to_unknown(p);
                    m
                }
                _ => m,
            }
        });
        expr.or_add_diagnostic(p, expected_expression);
        p.expect(T!['}']);
        return Present(m.complete(p, JSX_SPREAD_CHILD));
    }

    let expr = super::expr::parse_expression(p, ExpressionContext::default());
    let _ = expr.map(|mut m| match m.kind() {
        JsSyntaxKind::IMPORT_META
        | JsSyntaxKind::NEW_TARGET
        | JsSyntaxKind::JS_CLASS_EXPRESSION => {
            let err = p
                .err_builder("This expression is not valid as a JSX expression.")
                .primary(m.range(p), "");
            p.error(err);
            m.change_to_unknown(p);
            m
        }
        _ => m,
    });

    // test jsx jsx_children_expression_then_text
    // <test>
    //     {/* comment */}
    //      some
    //      text
    // </text>
    if p.at(T!['}']) {
        let context = match kind {
            ExpressionBlock::Attribute => LexContext::Regular,
            ExpressionBlock::Children => LexContext::JsxChild,
        };
        p.bump_with_context(T!['}'], context);
    } else {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    let kind = match kind {
        ExpressionBlock::Attribute => JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE,
        ExpressionBlock::Children => JsSyntaxKind::JSX_EXPRESSION_CHILD,
    };
    ParsedSyntax::Present(m.complete(p, kind))
}
