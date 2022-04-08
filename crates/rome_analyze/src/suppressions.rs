//! A low-effort WIP implementation of suppressions.

use std::collections::HashMap;

use rome_js_syntax::{JsSyntaxNode, JsSyntaxToken, TextRange};

const ROME_IGNORE: &str = "rome-ignore";

#[derive(Debug)]
pub struct Suppressions {
    pub suppressions: HashMap<String, Vec<TextRange>>,
}

impl Suppressions {
    pub fn ranges(&self, key: &str) -> Vec<TextRange> {
        match self.suppressions.get(key) {
            Some(r) => r.clone(),
            None => Vec::new(),
        }
    }

    pub fn match_range(&self, key: &str, range: TextRange) -> bool {
        match self.suppressions.get(key) {
            Some(ranges) => ranges.iter().any(|r| r.contains_range(range)),
            None => false,
        }
    }
}

pub fn compute(node: JsSyntaxNode) -> Suppressions {
    let mut suppressions: HashMap<String, Vec<TextRange>> = HashMap::new();

    for token in node.descendants_tokens() {
        for comment in token
            .leading_trivia()
            .pieces()
            .filter_map(|p| p.as_comments())
        {
            let mut splits = comment.text().split_whitespace();

            if splits.nth(1) != Some(ROME_IGNORE) {
                continue;
            }

            if let Some(range) = suppressed_range(&token) {
                for split in splits {
                    match suppressions.get_mut(split) {
                        Some(ranges) => ranges.push(range),
                        None => {
                            suppressions.insert(split.to_string(), vec![range]);
                        }
                    }
                }
            }
        }
    }
    Suppressions { suppressions }
}

// TODO: Improve this logic
fn suppressed_range(token: &JsSyntaxToken) -> Option<TextRange> {
    let mut node = token.parent()?;

    while !node.parent()?.text_trimmed().to_string().contains('\n') {
        node = node.parent()?;
    }
    Some(node.text_trimmed_range())
}
