pub mod jsx_parse_errors;

use rome_js_syntax::JsSyntaxKind::*;
use rslint_lexer::{JsSyntaxKind, LexContext, ReLexContext, T};

use crate::syntax::expr::parse_name;
use crate::syntax::js_parse_error::expected_identifier;
use crate::{Absent, Checkpoint, Marker, ParsedSyntax, Parser, Present};

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
    let syntax = parse_jsx_expression(&mut p);

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
fn parse_jsx_expression(p: &mut CheckpointedParser<'_, '_>) -> ParsedSyntax {
    parse_jsx_element(p, true).map(|element| {
        let m = element.precede(p);
        m.complete(p, JsSyntaxKind::JSX_ELEMENT_EXPRESSION)
    })
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

// test jsx jsx_text
// <a>test</a>;
// <a>   whitespace handling </a>;
// <a> multi
//    line
//          node
// </a>;
// <test>\u3333</test> // no error for invalid unicode escape

// test_err jsx jsx_invalid_text
// <a> test ></a>;
// <b> invalid }</b>;

/// Parses a JSX element
///
/// `in_expression` must be `true` if this element is a direct child of the `JsxElementExpression` (root of an expression).
/// It should be false when parsing any child node.
fn parse_jsx_element(p: &mut CheckpointedParser<'_, '_>, in_expression: bool) -> ParsedSyntax {
    let m = p.start();
    match parse_jsx_element_head(p, m, in_expression) {
        ParsedSyntax::Present(opening_marker)
            if opening_marker.kind() == JsSyntaxKind::JSX_OPENING_ELEMENT =>
        {
            let element_marker = opening_marker.precede(p);

            if p.at(JSX_TEXT) {
                let m = p.start();
                p.bump(JSX_TEXT);
                m.complete(p, JSX_TEXT_LITERAL);
            }

            let closing_marker = parse_jsx_closing_element(p, in_expression);
            if closing_marker.is_absent() {
                element_marker.abandon(p);
                ParsedSyntax::Absent
            } else {
                ParsedSyntax::Present(element_marker.complete(p, JsSyntaxKind::JSX_ELEMENT))
            }
        }
        ParsedSyntax::Present(self_closing_marker)
            if self_closing_marker.kind() == JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT =>
        {
            ParsedSyntax::Present(self_closing_marker)
        }
        ParsedSyntax::Absent => ParsedSyntax::Absent,
        _ => unreachable!("Unexpected present node returned"),
    }
}

// <a ...> or <a ... />
// ^          ^
fn parse_jsx_element_head(
    p: &mut CheckpointedParser<'_, '_>,
    m: Marker,
    in_expression: bool,
) -> ParsedSyntax {
    if !p.eat(T![<]) {
        return ParsedSyntax::Absent;
    }

    let _ = parse_jsx_any_element_name(p);

    let kind = if p.eat(T![/]) {
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
fn parse_jsx_closing_element(
    p: &mut CheckpointedParser<'_, '_>,
    in_expression: bool,
) -> ParsedSyntax {
    if !p.at(T![<]) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();

    if !p.expect(T![<]) || !p.expect(T![/]) {
        m.abandon(p);
        return Absent;
    }

    let _ = parse_jsx_any_element_name(p);

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
// <a-b.c></a-b.c>
//
// test_err jsx jsx_namespace_member_element_name
// <namespace:a.b></namespace:a.b>
fn parse_jsx_any_element_name(p: &mut CheckpointedParser<'_, '_>) -> ParsedSyntax {
    let left = parse_jsx_any_name(p);

    if let Present(mut left) = left {
        if left.kind() == JSX_NAMESPACE_NAME && p.at(T![.]) {
            let error = p
                .err_builder("JSX property access expressions cannot include JSX namespace names.")
                .primary(left.range(p), "");
            p.error(error)
        }

        while p.at(T![.]) {
            let m = left.precede(p);
            p.bump(T![.]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            left = m.complete(p, JSX_MEMBER_NAME)
        }

        Present(left)
    } else {
        left
    }
}

// test jsx jsx_any_name
// <a-b-c-d-e></a-b-c-d-e>;
// <a-b-c-d-e />;
// <if />;
// <namespace:name></namespace:name>;
// <dashed-namespaced:dashed-name />;
fn parse_jsx_any_name(p: &mut Parser) -> ParsedSyntax {
    parse_jsx_reference_identifier(p).map(|identifier| {
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

fn parse_jsx_reference_identifier(p: &mut Parser) -> ParsedSyntax {
    p.re_lex(ReLexContext::JsxIdentifier);

    if !p.at(JSX_IDENT) {
        return Absent;
    }

    let m = p.start();
    p.bump(JSX_IDENT);

    Present(m.complete(p, JSX_REFERENCE_IDENTIFIER))
}
