use std::collections::VecDeque;

use rome_js_syntax::{JsLanguage, JsSyntaxNode, TextRange};
use rome_rowan::syntax::Preorder;

#[derive(Debug)]
pub enum SemanticEvent {
    DeclarationFound { range: TextRange },
}

impl SemanticEvent {
    pub fn range(&self) -> &TextRange {
        match self {
            SemanticEvent::DeclarationFound { range } => range,
        }
    }

    pub fn str<'a>(&self, code: &'a str) -> &'a str {
        let range = self.range();
        let start: u32 = range.start().into();
        let end: u32 = range.end().into();
        &code[start as usize..end as usize]
    }
}

#[derive(Default)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
}

impl SemanticEventExtractor {
    pub fn extract_from(&mut self, node: &JsSyntaxNode) {
        use rome_js_syntax::JsSyntaxKind::*;
        use SemanticEvent::*;
        match node.kind() {
            JS_IDENTIFIER_BINDING => self.stash.push_back(DeclarationFound {
                range: node.text_trimmed_range(),
            }),
            _ => {}
        }
    }
}
struct SemanticEventIterator {
    iter: Preorder<JsLanguage>,
    extractor: SemanticEventExtractor,
}

impl Iterator for SemanticEventIterator {
    type Item = SemanticEvent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.extractor.stash.pop_front() {
                break Some(e);
            } else {
                use rome_js_syntax::WalkEvent::*;
                match self.iter.next() {
                    Some(Enter(node)) => {
                        self.extractor.extract_from(&node);
                    }
                    Some(_) => {}
                    None => break None,
                }
            }
        }
    }
}

pub fn semantic_events(root: JsSyntaxNode) -> impl IntoIterator<Item = SemanticEvent> {
    SemanticEventIterator {
        iter: root.preorder(),
        extractor: SemanticEventExtractor::default(),
    }
}
