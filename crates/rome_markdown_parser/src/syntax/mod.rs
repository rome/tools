use crate::parser::MarkdownParser;
use rome_markdown_syntax::MdSyntaxKind::MD_ROOT;
use rome_parser::Parser;

pub(crate) fn parse_root(p: &mut MarkdownParser) {
    let m = p.start();

    m.complete(p, MD_ROOT);
}
