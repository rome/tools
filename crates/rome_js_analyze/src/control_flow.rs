use rome_js_syntax::JsLanguage;

pub(crate) type ControlFlowGraph = rome_control_flow::ControlFlowGraph<JsLanguage>;
pub(crate) type FunctionBuilder = rome_control_flow::builder::FunctionBuilder<JsLanguage>;

mod nodes;
mod visitor;

pub(crate) use self::visitor::make_visitor;
pub(crate) use self::visitor::JsAnyControlFlowRoot;
