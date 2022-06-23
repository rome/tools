use std::ops::ControlFlow;

use rome_diagnostics::file::FileId;
use rome_rowan::{AstNode, Language, SyntaxNode, TextRange, WalkEvent};

use crate::{registry::NodeLanguage, LanguageRoot, QueryMatch, RuleRegistry};

/// Mutable context objects shared by all visitors
pub struct VisitorContext<'a, L: Language, B> {
    pub file_id: FileId,
    pub root: LanguageRoot<L>,
    pub range: Option<TextRange>,
    pub registry: RuleRegistry<'a, L, B>,
}

impl<'a, L: Language, B> VisitorContext<'a, L, B> {
    pub fn match_query(&mut self, query_match: &QueryMatch<L>) -> ControlFlow<B> {
        self.registry
            .match_query(self.file_id, &self.root, query_match)
    }
}

/// Visitors are the main building blocks of the analyzer: they receive syntax
/// [WalkEvent]s, process these events to build secondary data structures from
/// the syntax tree, and emit rule query matches through the [RuleRegistry]
pub trait Visitor<B> {
    type Language: Language;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        ctx: &mut VisitorContext<Self::Language, B>,
    ) -> ControlFlow<B>;
}

/// A node visitor is a special kind of visitor that does not have a persistent
/// state for the entire run of the analyzer. Instead these visitors are
/// transient, they get instantiated when the traversal enters the
/// corresponding node type and destroyed when the corresponding node exits
///
/// Due to these specificities node visitors do not implement [Visitor]
/// directly, instead one or more of these must the merged into a single
/// visitor type using the [merge_node_visitors] macro
pub trait NodeVisitor<V, B>: Sized {
    type Node: AstNode;

    fn enter(
        node: Self::Node,
        ctx: &mut VisitorContext<NodeLanguage<Self::Node>, B>,
        stack: &mut V,
    ) -> ControlFlow<B, Self>;

    fn exit(
        self,
        node: Self::Node,
        ctx: &mut VisitorContext<NodeLanguage<Self::Node>, B>,
        stack: &mut V,
    ) -> ControlFlow<B>;
}

/// Create a single unified [Visitor] type from a set of [NodeVisitor]s
///
/// # Example
///
/// ```ignore
/// struct BinaryVisitor;
///
/// impl NodeVisitor for BinaryVisitor {
///     type Node = BinaryExpression;
/// }
///
/// struct UnaryVisitor;
///
/// impl NodeVisitor for UnaryVisitor {
///     type Node = UnaryExpression;
/// }
///
/// merge_node_visitors! {
///     // This declares a new `ExpressionVisitor` struct that implements
///     // `Visitor` and manages instances of `BinaryVisitor` and
///     // `UnaryVisitor`
///     pub(crate) ExpressionVisitor {
///         binary: BinaryVisitor,
///         unary: UnaryVisitor,
///     }
/// }
/// ```
#[macro_export]
macro_rules! merge_node_visitors {
    ( $vis:vis $name:ident { $( $id:ident: $visitor:ty, )+ } ) => {
        $vis struct $name<B> {
            stack: Vec<(::std::any::TypeId, usize)>,
            $( $vis $id: Vec<(usize, $visitor)>, )*
            _lang: ::std::marker::PhantomData<B>,
        }

        impl<B> $name<B> {
            $vis fn new() -> Self {
                Self {
                    stack: Vec::new(),
                    $( $id: Vec::new(), )*
                    _lang: ::std::marker::PhantomData,
                }
            }
        }

        impl<B> $crate::Visitor<B> for $name<B> {
            type Language = <( $( <$visitor as $crate::NodeVisitor<$name<B>, B>>::Node, )* ) as ::rome_rowan::macros::UnionLanguage>::Language;

            fn visit(
                &mut self,
                event: &::rome_rowan::WalkEvent<::rome_rowan::SyntaxNode<Self::Language>>,
                ctx: &mut $crate::VisitorContext<Self::Language, B>,
            ) -> ::std::ops::ControlFlow<B> {
                match event {
                    ::rome_rowan::WalkEvent::Enter(node) => {
                        let kind = node.kind();

                        $(
                            if <<$visitor as $crate::NodeVisitor<$name<B>, B>>::Node as ::rome_rowan::AstNode>::can_cast(kind) {
                                let node = <<$visitor as $crate::NodeVisitor<$name<B>, B>>::Node as ::rome_rowan::AstNode>::unwrap_cast(node.clone());
                                return match <$visitor as $crate::NodeVisitor<$name<B>, B>>::enter(node, ctx, self) {
                                    ::std::ops::ControlFlow::Continue(state) => {
                                        let stack_index = self.stack.len();
                                        let ty_index = self.$id.len();

                                        self.$id.push((stack_index, state));
                                        self.stack.push((::std::any::TypeId::of::<$visitor>(), ty_index));

                                        ::std::ops::ControlFlow::Continue(())
                                    }
                                    ::std::ops::ControlFlow::Break(value) => {
                                        ::std::ops::ControlFlow::Break(value)
                                    }
                                };
                            }
                        )*
                    }
                    ::rome_rowan::WalkEvent::Leave(node) => {
                        let kind = node.kind();

                        $(
                            if <<$visitor as $crate::NodeVisitor<$name<B>, B>>::Node as ::rome_rowan::AstNode>::can_cast(kind) {
                                self.stack.pop().unwrap();
                                let (_, state) = self.$id.pop().unwrap();

                                let node = <<$visitor as $crate::NodeVisitor<$name<B>, B>>::Node as ::rome_rowan::AstNode>::unwrap_cast(node.clone());
                                return <$visitor as $crate::NodeVisitor<$name<B>, B>>::exit(state, node, ctx, self);
                            }
                        )*
                    }
                }

                ::std::ops::ControlFlow::Continue(())
            }
        }
    };
}
