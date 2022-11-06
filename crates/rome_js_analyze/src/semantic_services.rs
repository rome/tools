use rome_analyze::{
    FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch, Queryable,
    RuleKey, ServiceBag, Visitor, VisitorContext, VisitorFinishContext,
};
use rome_js_semantic::{SemanticEventExtractor, SemanticModel, SemanticModelBuilder};
use rome_js_syntax::JsSyntaxKind::{
    JSX_REFERENCE_IDENTIFIER, JS_ARROW_FUNCTION_EXPRESSION, JS_BLOCK_STATEMENT, JS_CALL_EXPRESSION,
    JS_CATCH_CLAUSE, JS_CLASS_DECLARATION, JS_CLASS_EXPORT_DEFAULT_DECLARATION,
    JS_CLASS_EXPRESSION, JS_CONSTRUCTOR_CLASS_MEMBER, JS_FOR_IN_STATEMENT, JS_FOR_OF_STATEMENT,
    JS_FOR_STATEMENT, JS_FUNCTION_BODY, JS_FUNCTION_DECLARATION,
    JS_FUNCTION_EXPORT_DEFAULT_DECLARATION, JS_FUNCTION_EXPRESSION, JS_GETTER_CLASS_MEMBER,
    JS_GETTER_OBJECT_MEMBER, JS_IDENTIFIER_ASSIGNMENT, JS_IDENTIFIER_BINDING,
    JS_METHOD_CLASS_MEMBER, JS_METHOD_OBJECT_MEMBER, JS_MODULE, JS_REFERENCE_IDENTIFIER, JS_SCRIPT,
    JS_SETTER_CLASS_MEMBER, JS_SETTER_OBJECT_MEMBER, JS_SWITCH_STATEMENT, TS_ENUM_DECLARATION,
    TS_FUNCTION_TYPE, TS_IDENTIFIER_BINDING, TS_INTERFACE_DECLARATION, TS_TYPE_ALIAS_DECLARATION,
};
use rome_js_syntax::{JsAnyRoot, JsLanguage, WalkEvent};
use rome_rowan::{AstNode, SyntaxNode};

pub struct SemanticServices {
    model: SemanticModel,
}

impl SemanticServices {
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl FromServices for SemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let model = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"])
        })?;
        Ok(Self { model })
    }
}

impl Phase for SemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// The [SemanticServices] types can be used as a queryable to get an instance
/// of the whole [SemanticModel] without matching on a specific AST node
impl Queryable for SemanticServices {
    type Output = SemanticModel;
    type Language = JsLanguage;
    type Services = Self;

    const KEY: QueryKey<Self::Language> = QueryKey::SemanticModel;

    fn unwrap_match(services: &ServiceBag, query: &QueryMatch<Self::Language>) -> Self::Output {
        match query {
            QueryMatch::SemanticModel(..) => services
                .get_service::<SemanticModel>()
                .expect("SemanticModel service is not registered"),
            _ => panic!("tried to unwrap unsupported QueryMatch kind, expected SemanticModel"),
        }
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub struct Semantic<N>(pub N);

impl<N> Queryable for Semantic<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Output = N;
    type Language = JsLanguage;
    type Services = SemanticServices;

    /// Match on [QueryMatch::Syntax] if the kind of the syntax node matches
    /// the kind set of `N`
    const KEY: QueryKey<Self::Language> = QueryKey::Syntax(N::KIND_SET);

    fn unwrap_match(_: &ServiceBag, query: &QueryMatch<Self::Language>) -> Self::Output {
        match query {
            QueryMatch::Syntax(node) => N::unwrap_cast(node.clone()),
            _ => panic!("tried to unwrap unsupported QueryMatch kind, expected Syntax"),
        }
    }
}

pub(crate) struct SemanticModelBuilderVisitor {
    extractor: SemanticEventExtractor,
    builder: SemanticModelBuilder,
}

impl SemanticModelBuilderVisitor {
    pub(crate) fn new(root: &JsAnyRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root.clone()),
        }
    }
}

impl Visitor for SemanticModelBuilderVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<JsLanguage>>,
        _ctx: VisitorContext<JsLanguage>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                match node.kind() {
                    JS_IDENTIFIER_BINDING | TS_IDENTIFIER_BINDING => {
                        self.builder.push_node(node);
                    }
                    JS_REFERENCE_IDENTIFIER | JSX_REFERENCE_IDENTIFIER => {
                        self.builder.push_node(node);
                    }
                    JS_IDENTIFIER_ASSIGNMENT => {
                        self.builder.push_node(node);
                    }
                    JS_CALL_EXPRESSION => {
                        self.builder.push_node(node);
                    }

                    JS_MODULE | JS_SCRIPT => self.builder.push_node(node),
                    JS_FUNCTION_DECLARATION
                    | JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
                    | JS_FUNCTION_EXPRESSION
                    | JS_ARROW_FUNCTION_EXPRESSION
                    | JS_CLASS_DECLARATION
                    | JS_CLASS_EXPORT_DEFAULT_DECLARATION
                    | JS_CLASS_EXPRESSION
                    | JS_CONSTRUCTOR_CLASS_MEMBER
                    | JS_METHOD_CLASS_MEMBER
                    | JS_GETTER_CLASS_MEMBER
                    | JS_SETTER_CLASS_MEMBER
                    | JS_METHOD_OBJECT_MEMBER
                    | JS_GETTER_OBJECT_MEMBER
                    | JS_SETTER_OBJECT_MEMBER
                    | JS_FUNCTION_BODY
                    | TS_INTERFACE_DECLARATION
                    | TS_ENUM_DECLARATION
                    | TS_TYPE_ALIAS_DECLARATION
                    | TS_FUNCTION_TYPE => {
                        self.builder.push_node(node);
                    }

                    JS_BLOCK_STATEMENT | JS_FOR_STATEMENT | JS_FOR_OF_STATEMENT
                    | JS_FOR_IN_STATEMENT | JS_SWITCH_STATEMENT | JS_CATCH_CLAUSE => {
                        self.builder.push_node(node);
                    }
                    _ => {}
                }

                self.extractor.enter(node);
            }
            WalkEvent::Leave(node) => {
                self.extractor.leave(node);
            }
        }

        while let Some(e) = self.extractor.pop() {
            self.builder.push_event(e);
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        let model = self.builder.build();
        ctx.services.insert_service(model);
    }
}

pub(crate) struct SemanticModelVisitor;

impl Visitor for SemanticModelVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<JsLanguage>>,
        mut ctx: VisitorContext<JsLanguage>,
    ) {
        let root = match event {
            WalkEvent::Enter(node) => {
                if node.parent().is_some() {
                    return;
                }

                node
            }
            WalkEvent::Leave(_) => return,
        };

        let text_range = root.text_range();
        ctx.match_query(QueryMatch::SemanticModel(text_range));
    }
}
