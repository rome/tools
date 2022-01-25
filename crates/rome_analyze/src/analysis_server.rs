use std::{collections::HashMap, sync::Arc};

use rslint_parser::{parse_script, AstNode, SyntaxNode, TextRange};
use tracing::trace;

use crate::{
    analyzers, assists,
    suppressions::{self, Suppressions},
    Analysis, AnalyzerContext, AssistContext,
};

pub type FileId = usize;

#[derive(Default)]
pub struct AnalysisServer {
    file_map: HashMap<FileId, Arc<String>>,
}

impl AnalysisServer {
    pub fn new() -> Self {
        Self {
            file_map: Default::default(),
        }
    }

    pub fn set_file_text(&mut self, file_id: FileId, text: impl Into<Arc<String>>) {
        self.file_map.insert(file_id, text.into());
    }

    pub fn get_file_text(&self, file_id: FileId) -> Option<Arc<String>> {
        self.file_map.get(&file_id).cloned()
    }

    pub fn parse(&self, file_id: FileId) -> SyntaxNode {
        let text = self
            .get_file_text(file_id)
            .expect("File contents missing while parsing");
        parse_script(&text, file_id).syntax()
    }

    pub fn suppressions(&self, file_id: FileId) -> Suppressions {
        let tree = self.parse(file_id);
        suppressions::compute(tree)
    }

    pub fn query_nodes<T: AstNode>(&self, file_id: FileId) -> impl Iterator<Item = T> {
        trace!("Query nodes: {:?}", std::any::type_name::<T>());
        let tree = self.parse(file_id);
        tree.descendants().filter_map(|n| T::cast(n))
    }

    pub fn find_node_at_range<T: AstNode>(&self, file_id: FileId, range: TextRange) -> Option<T> {
        trace!("Find {:?} range: {:?}", std::any::type_name::<T>(), range);
        let tree = self.parse(file_id);
        tree.covering_element(range).ancestors().find_map(T::cast)
    }

    pub fn assists(&self, file_id: FileId, cursor_range: TextRange) -> Analysis {
        trace!("Assists range: {:?}", cursor_range);

        let mut signals = vec![];

        for provider in assists::all() {
            let ctx = AssistContext::new(self, file_id, cursor_range, provider);
            let analyze_fn = provider.analyze;
            if let Some(analysis) = analyze_fn(&ctx) {
                signals.extend(analysis.signals.into_iter())
            }
        }
        signals.into()
    }

    pub fn analyze(&self, file_id: FileId) -> Analysis {
        let suppressions = self.suppressions(file_id);

        let mut signals = vec![];

        for analyzer in analyzers::all() {
            let ctx = AnalyzerContext::new(self, file_id, analyzer);
            let analyze_fn = analyzer.analyze;
            if let Some(analysis) = analyze_fn(&ctx) {
                for s in analysis.signals {
                    if let Some(range) = s.range() {
                        if s.is_diagnostic() && suppressions.match_range(analyzer.name, range) {
                            continue;
                        }
                        signals.push(s);
                    }
                }
            }
        }
        signals.into()
    }
}
