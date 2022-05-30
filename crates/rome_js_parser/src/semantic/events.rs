use rome_js_syntax::{JsLanguage, JsSyntaxNode, TextRange};
use rome_rowan::syntax::Preorder;

#[derive(Debug)]
pub enum SemanticEvent {
    DeclarationFound { range: TextRange },
}

impl SemanticEvent {
    pub fn range(&self) -> &TextRange {
        match self {
            SemanticEvent::DeclarationFound { range } => &range,
        }
    }

    pub fn str<'a>(&self, code: &'a str) -> &'a str {
        let range = self.range();
        let start: u32 = range.start().into();
        let end: u32 = range.end().into();
        &code[start as usize..end as usize]
    }
}

struct SemanticEventIterator {
    iter: Preorder<JsLanguage>,
}

impl SemanticEventIterator {
    fn extract_event(&self, node: &JsSyntaxNode) -> Option<SemanticEvent> {
        use rome_js_syntax::JsSyntaxKind::*;
        use SemanticEvent::*;
        match node.kind() {
            JS_IDENTIFIER_BINDING => Some(DeclarationFound {
                range: node.text_trimmed_range(),
            }),
            _ => None,
        }
    }
}

impl Iterator for SemanticEventIterator {
    type Item = SemanticEvent;

    fn next(&mut self) -> Option<Self::Item> {
        use rome_js_syntax::WalkEvent::*;
        loop {
            match self.iter.next() {
                Some(Enter(node)) => {
                    if let Some(e) = self.extract_event(&node) {
                        break Some(e);
                    }
                }
                Some(_) => {}
                None => break None,
            }
        }
    }
}

pub fn semantic_events(root: JsSyntaxNode) -> impl IntoIterator<Item = SemanticEvent> {
    let i = SemanticEventIterator {
        iter: root.preorder(),
    };
    i
}
