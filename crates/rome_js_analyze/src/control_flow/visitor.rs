use std::{any::TypeId, ops::ControlFlow};

use rome_analyze::{merge_node_visitors, QueryMatch, Visitor, VisitorContext};
use rome_js_syntax::{
    JsAnyFunction, JsConstructorClassMember, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsModule, JsScript, JsSetterClassMember,
    JsSetterObjectMember,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxError, SyntaxResult};

use super::{nodes::*, FunctionBuilder};

/// Return a new instance of the [ControlFlowVisitor]
pub(crate) fn make_visitor<B>() -> impl Visitor<B, Language = JsLanguage> {
    ControlFlowVisitor::new()
}

/// Wrapper macro for [merge_node_visitors], implements additional control
/// flow-related utilities on top of the generated visitor
macro_rules! declare_visitor {
    ( $vis:vis $name:ident { $( $id:ident: $visitor:ty, )* } ) => {
        merge_node_visitors! {
            $vis $name {
                function: FunctionVisitor,
                $( $id: VisitorAdapter<$visitor>, )*
            }
        }

        /// Slice of the merged visitor state stack cut off at the current function
        pub(super) struct StatementStack<'a> {
            pub(super) stack: &'a mut [(TypeId, usize)],
            $( $id: &'a mut [(usize, VisitorAdapter<$visitor>)], )*
        }

        impl<'a> StatementStack<'a> {
            /// Split the visitor state at the topmost function, returning the
            /// corresponding function visitor and the rest of the stack above it
            fn new<B>(visitor: &'a mut $name<B>) -> Option<(&mut FunctionVisitor, Self)> {
                let (index, builder) = visitor.function.last_mut()?;

                Some((builder, Self {
                    stack: &mut visitor.stack[*index + 1..],
                    $(
                        // For safety, cut off the stack slices below the start
                        // of the current function in debug mode
                        #[cfg(debug_assertions)]
                        $id: {
                            let index = visitor
                                .$id
                                .iter()
                                .rposition(|(stack_index, _)| *stack_index < *index)
                                .map_or(0, |index| (index + 1).min(visitor.$id.len().saturating_sub(1)));
                            &mut visitor.$id[index..]
                        },
                        #[cfg(not(debug_assertions))]
                        $id: &mut visitor.$id,
                    )*
                }))
            }
        }

        $( impl<'a> MergedVisitor<'a, $visitor> for StatementStack<'a> {
            fn read_top(self) -> SyntaxResult<&'a mut $visitor> {
                let (_, visitor) =
                    self.$id.last_mut().ok_or(::rome_rowan::SyntaxError::MissingRequiredChild)?;
                let VisitorAdapter(visitor) = visitor;
                let visitor = visitor.as_mut().map_err(|err| *err)?;
                Ok(visitor)
            }

            fn try_downcast(&'a self, type_id: TypeId, index: usize) -> Option<&'a $visitor> {
                if type_id != TypeId::of::<VisitorAdapter<$visitor>>() {
                    return None;
                }

                let (_, visitor) = &self.$id[index];
                let VisitorAdapter(visitor) = visitor;
                let visitor = visitor.as_ref().ok()?;
                Some(visitor)
            }
        } )*
    };
}

declare_visitor! {
    ControlFlowVisitor {
        statement: StatementVisitor,
        block: BlockVisitor,
        try_stmt: TryVisitor,
        catch: CatchVisitor,
        finally: FinallyVisitor,
        if_stmt: IfVisitor,
        else_stmt: ElseVisitor,
        switch: SwitchVisitor,
        case: CaseVisitor,
        for_stmt: ForVisitor,
        for_in: ForInVisitor,
        for_of: ForOfVisitor,
        while_stmt: WhileVisitor,
        do_while: DoWhileVisitor,
        break_stmt: BreakVisitor,
        continue_stmt: ContinueVisitor,
        return_stmt: ReturnVisitor,
        throw: ThrowVisitor,
        variable: VariableVisitor,
    }
}

/// Utility implemented for [StatementStack] in the [declare_visitor] macro,
/// allows type checked access into the visitor state stack
pub(super) trait MergedVisitor<'a, N> {
    fn read_top(self) -> SyntaxResult<&'a mut N>;
    fn try_downcast(&'a self, type_id: TypeId, index: usize) -> Option<&'a N>;
}

// Wrapper methods on top of the `MergedVisitor` trait to support for the
// "turbofish" (`::<>`) syntax
impl<'a> StatementStack<'a> {
    pub(super) fn read_top<N>(self) -> SyntaxResult<&'a mut N>
    where
        Self: MergedVisitor<'a, N>,
    {
        MergedVisitor::read_top(self)
    }

    pub(super) fn try_downcast<N>(&'a self, type_id: TypeId, index: usize) -> Option<&'a N>
    where
        Self: MergedVisitor<'a, N>,
    {
        MergedVisitor::try_downcast(self, type_id, index)
    }
}

pub(super) struct FunctionVisitor {
    builder: Option<FunctionBuilder>,
}

declare_node_union! {
    JsAnyControlFlowRoot = JsModule
        | JsScript
        | JsAnyFunction
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsMethodObjectMember
        | JsConstructorClassMember
        | JsMethodClassMember
        | JsGetterClassMember
        | JsSetterClassMember
}

impl<B> rome_analyze::NodeVisitor<ControlFlowVisitor<B>, B> for FunctionVisitor {
    type Node = JsAnyControlFlowRoot;

    fn enter(
        _: Self::Node,
        _: &mut VisitorContext<JsLanguage, B>,
        _: &mut ControlFlowVisitor<B>,
    ) -> ControlFlow<B, Self> {
        ControlFlow::Continue(Self {
            builder: Some(FunctionBuilder::default()),
        })
    }

    fn exit(
        self,
        _: Self::Node,
        ctx: &mut VisitorContext<JsLanguage, B>,
        _: &mut ControlFlowVisitor<B>,
    ) -> ControlFlow<B> {
        if let Some(builder) = self.builder {
            return ctx.match_query(&QueryMatch::ControlFlowGraph(builder.finish()));
        }

        ControlFlow::Continue(())
    }
}

/// Wrapper trait for [rome_analyze::NodeVisitor] adding control flow specific
/// utilities (error handling and automatic [FunctionBuilder] injection)
pub(super) trait NodeVisitor<B>: Sized {
    type Node: AstNode<Language = JsLanguage>;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self>;

    fn exit(self, _: Self::Node, _: &mut FunctionBuilder, _: StatementStack) -> SyntaxResult<()> {
        Ok(())
    }
}

/// Wrapper type implementing [rome_analyze::NodeVisitor] for types
/// implementing the control-flow specific [NodeVisitor] trait
pub(super) struct VisitorAdapter<V>(SyntaxResult<V>);

impl<B, V> rome_analyze::NodeVisitor<ControlFlowVisitor<B>, B> for VisitorAdapter<V>
where
    V: NodeVisitor<B>,
{
    type Node = V::Node;

    fn enter(
        node: Self::Node,
        _: &mut VisitorContext<JsLanguage, B>,
        stack: &mut ControlFlowVisitor<B>,
    ) -> ControlFlow<B, Self> {
        let (visitor, stack) = match StatementStack::new(stack) {
            Some((builder, stack)) => (builder, stack),
            None => return ControlFlow::Continue(Self(Err(SyntaxError::MissingRequiredChild))),
        };

        let result = if let Some(builder) = visitor.builder.as_mut() {
            let result = V::enter(node, builder, stack);

            if result.is_err() {
                visitor.builder.take();
            }

            result
        } else {
            Err(SyntaxError::MissingRequiredChild)
        };

        ControlFlow::Continue(Self(result))
    }

    fn exit(
        self,
        node: Self::Node,
        _: &mut VisitorContext<JsLanguage, B>,
        stack: &mut ControlFlowVisitor<B>,
    ) -> ControlFlow<B> {
        let state = match self {
            Self(Ok(state)) => state,
            _ => return ControlFlow::Continue(()),
        };

        let (visitor, stack) = match StatementStack::new(stack) {
            Some((builder, stack)) => (builder, stack),
            None => return ControlFlow::Continue(()),
        };

        if let Some(builder) = visitor.builder.as_mut() {
            if state.exit(node, builder, stack).is_err() {
                visitor.builder.take();
            }
        }

        ControlFlow::Continue(())
    }
}
