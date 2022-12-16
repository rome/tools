use rome_analyze::QueryMatch;
use rome_analyze::{AddVisitor, Phases, Queryable, ServiceBag};
use rome_js_syntax::AnyJsRoot;
use rome_js_syntax::JsLanguage;
use rome_js_syntax::TextRange;

pub type JsControlFlowGraph = rome_control_flow::ControlFlowGraph<JsLanguage>;
pub(crate) type FunctionBuilder = rome_control_flow::builder::FunctionBuilder<JsLanguage>;

mod nodes;
mod visitor;

pub(crate) use self::visitor::make_visitor;
pub(crate) use self::visitor::AnyJsControlFlowRoot;

pub struct ControlFlowGraph {
    pub graph: JsControlFlowGraph,
    pub range: TextRange,
}

impl QueryMatch for ControlFlowGraph {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl Queryable for ControlFlowGraph {
    type Input = ControlFlowGraph;
    type Output = JsControlFlowGraph;

    type Language = JsLanguage;
    type Services = ();

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, make_visitor);
    }

    fn unwrap_match(_: &ServiceBag, query: &ControlFlowGraph) -> Self::Output {
        query.graph.clone()
    }
}
