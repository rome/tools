use rome_js_syntax::{JsSyntaxNode, TextRange};

#[derive(Debug)]
pub struct Symbol {}

impl Symbol {
    pub fn range(&self) -> TextRange {
        todo!()
    }

    pub fn name(&self) -> &str {
        todo!()
    }
}

pub fn symbols(_root: JsSyntaxNode) -> impl IntoIterator<Item = Symbol> {
    vec![]
}
