use crate::parser::MarkdownParser;
use rome_markdown_syntax::MdSyntaxKind::{EOF, MD_ELEMENT_LIST, MD_HEADING, MD_ROOT};
use rome_markdown_syntax::T;
use rome_parser::prelude::ParsedSyntax;
use rome_parser::prelude::ParsedSyntax::Absent;
use rome_parser::prelude::*;
use rome_parser::{Parser, ParserProgress};

pub(crate) fn parse_root(p: &mut MarkdownParser) {
    let m = p.start();

    parse_elements(p);

    m.complete(p, MD_ROOT);
}

pub(crate) fn parse_elements(p: &mut MarkdownParser) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        T![#] => {
            parse_heading(p);
        }
        _ => {}
    };

    ParsedSyntax::Present(m.complete(p, MD_ELEMENT_LIST))
}

pub(crate) fn parse_heading(p: &mut MarkdownParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }
    let m = p.start();

    let mut progress = ParserProgress::default();
    while p.at(T![#]) && !p.at(EOF) {
        p.bump(T![#]);

        progress.assert_progressing(p);
    }

    ParsedSyntax::Present(m.complete(p, MD_HEADING))
}
