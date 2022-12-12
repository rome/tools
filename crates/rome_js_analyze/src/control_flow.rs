use rome_analyze::{AddVisitor, Phases, QueryKey, QueryMatch, Queryable, ServiceBag};
use rome_js_syntax::AnyJsRoot;
use rome_js_syntax::JsLanguage;

pub(crate) type JsControlFlowGraph = rome_control_flow::ControlFlowGraph<JsLanguage>;
pub(crate) type FunctionBuilder = rome_control_flow::builder::FunctionBuilder<JsLanguage>;

mod nodes;
mod visitor;

pub(crate) use self::visitor::make_visitor;
pub(crate) use self::visitor::AnyJsControlFlowRoot;

pub(crate) struct ControlFlowGraph(JsControlFlowGraph);

impl Queryable for ControlFlowGraph {
    type Output = JsControlFlowGraph;
    type Language = JsLanguage;
    type Services = ();

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, make_visitor());
    }

    const KEY: QueryKey<Self::Language> = QueryKey::ControlFlowGraph;

    fn unwrap_match(_: &ServiceBag, query: &QueryMatch<Self::Language>) -> Self::Output {
        match query {
            QueryMatch::ControlFlowGraph(cfg, _) => cfg.clone(),
            _ => panic!("tried to unwrap unsupported QueryMatch kind, expected Syntax"),
        }
    }
}
