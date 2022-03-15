pub mod jsx_parse_errors;

use rslint_lexer::{JsSyntaxKind, T};

use crate::{Absent, Checkpoint, Marker, ParsedSyntax, Parser};

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
    parse_jsx_element(p).map(|element| {
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
fn parse_jsx_element(p: &mut CheckpointedParser<'_, '_>) -> ParsedSyntax {
    let m = p.start();
    match parse_jsx_element_head(p, m) {
        ParsedSyntax::Present(opening_marker)
            if opening_marker.kind() == JsSyntaxKind::JSX_OPENING_ELEMENT =>
        {
            let element_marker = opening_marker.precede(p);
            let closing_marker = parse_jsx_closing_element(p);
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
fn parse_jsx_element_head(p: &mut CheckpointedParser<'_, '_>, m: Marker) -> ParsedSyntax {
    if !p.eat(T![<]) {
        return ParsedSyntax::Absent;
    }

    let _ = parse_jsx_any_element_name(p);

    let kind = if p.eat(T![/]) {
        JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT
    } else {
        JsSyntaxKind::JSX_OPENING_ELEMENT
    };

    if !p.expect(T![>]) {
        m.abandon(p);
        return Absent;
    }

    ParsedSyntax::Present(m.complete(p, kind))
}

// <a/>
// ^
fn parse_jsx_closing_element(p: &mut CheckpointedParser<'_, '_>) -> ParsedSyntax {
    if !p.at(T![<]) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();

    if !p.expect(T![<]) || !p.expect(T![/]) {
        m.abandon(p);
        return Absent;
    }

    let _ = parse_jsx_any_element_name(p);

    if !p.eat(T![>]) {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_CLOSING_ELEMENT))
}

fn parse_jsx_any_element_name(p: &mut CheckpointedParser<'_, '_>) -> ParsedSyntax {
    let m = p.start();

    if !p.eat(T![ident]) {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_REFERENCE_IDENTIFIER))
}
