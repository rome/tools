use std::any::TypeId;

use rome_control_flow::builder::BlockId;
use rome_js_syntax::{
    JsAnySwitchClause, JsBlockStatement, JsBreakStatement, JsCatchClause, JsContinueStatement,
    JsDebuggerStatement, JsDoWhileStatement, JsElseClause, JsEmptyStatement, JsExpressionStatement,
    JsFinallyClause, JsForInStatement, JsForOfStatement, JsForStatement, JsIfStatement,
    JsLabeledStatement, JsReturnStatement, JsSwitchStatement, JsSyntaxToken, JsThrowStatement,
    JsTryFinallyStatement, JsVariableStatement, JsWhileStatement,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxError, SyntaxResult};

use super::{
    visitor::{FunctionVisitor, NodeVisitor, StatementStack, VisitorAdapter},
    FunctionBuilder,
};

declare_node_union! {
    pub(super) JsSimpleStatement = JsDebuggerStatement | JsEmptyStatement | JsExpressionStatement | JsVariableStatement
}

pub(super) struct StatementVisitor;

impl<B> NodeVisitor<B> for StatementVisitor {
    type Node = JsSimpleStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        builder.append_statement().with_node(node.into_syntax());

        Ok(Self)
    }
}

pub(super) struct BlockVisitor {
    break_block: Option<(JsSyntaxToken, BlockId)>,
}

impl<B> NodeVisitor<B> for BlockVisitor {
    type Node = JsBlockStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let break_block = match node.parent::<JsLabeledStatement>() {
            Some(label) => {
                let label = label.label_token()?;
                let block = builder.append_block();
                Some((label, block))
            }
            None => None,
        };

        Ok(Self { break_block })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        if let Some((_, block)) = self.break_block {
            builder.append_jump(false, block);
        }

        Ok(())
    }
}

pub(super) struct TryFinallyVisitor {
    finally_block: Option<BlockId>,
}

impl<B> NodeVisitor<B> for TryFinallyVisitor {
    type Node = JsTryFinallyStatement;

    fn enter(_: Self::Node, _: &mut FunctionBuilder, _: StatementStack) -> SyntaxResult<Self> {
        Ok(Self {
            finally_block: None,
        })
    }
}

pub(super) struct CatchVisitor {
    next_block: Option<BlockId>,
}

impl<B> NodeVisitor<B> for CatchVisitor {
    type Node = JsCatchClause;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        if let Ok(try_finally_stmt) = stack.read_top::<TryFinallyVisitor>() {
            let finally_block = try_finally_stmt.finally_block.get_or_insert_with(|| {
                let finally_block = builder.append_block();
                builder.add_entry_block(finally_block);
                finally_block
            });

            builder.append_jump(false, *finally_block);

            let catch_block = builder.append_block();
            builder.add_entry_block(catch_block);
            builder.set_cursor(catch_block);

            Ok(Self { next_block: None })
        } else {
            // Cursor is at the end of the try block, jump to the next block
            let next_block = builder.append_block();
            builder.append_jump(false, next_block);

            // Create the catch block, mark is as entry point and start writing
            let catch_block = builder.append_block();
            builder.add_entry_block(catch_block);
            builder.set_cursor(catch_block);

            Ok(Self {
                next_block: Some(next_block),
            })
        }
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<()> {
        let Self { next_block } = self;

        let try_finally_stmt = stack.read_top::<TryFinallyVisitor>();

        if let Ok(try_finally_stmt) = try_finally_stmt {
            builder.append_jump(false, try_finally_stmt.finally_block.unwrap());

            Ok(())
        } else {
            let next_block = next_block.unwrap();

            // Insert a jump to the next block at the end of the catch block
            builder.append_jump(false, next_block);

            // Continue writing on the next block
            builder.set_cursor(next_block);

            Ok(())
        }
    }
}

pub(super) struct FinallyVisitor;

impl<B> NodeVisitor<B> for FinallyVisitor {
    type Node = JsFinallyClause;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        let try_finally_stmt = stack.read_top::<TryFinallyVisitor>()?;

        let has_catch = try_finally_stmt.finally_block.is_some();
        let finally_block = try_finally_stmt.finally_block.get_or_insert_with(|| {
            let finally_block = builder.append_block();
            builder.add_entry_block(finally_block);
            finally_block
        });

        if !has_catch {
            builder.append_jump(false, *finally_block);
        }

        builder.set_cursor(*finally_block);

        Ok(Self)
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let next_block = builder.append_block();
        builder.append_jump(false, next_block);

        builder.set_cursor(next_block);
        Ok(())
    }
}

pub(super) struct IfVisitor {
    entry_block: BlockId,
    consequent_start: BlockId,
    consequent_end: Option<BlockId>,
    alt_block: Option<(BlockId, BlockId)>,
}

impl<B> NodeVisitor<B> for IfVisitor {
    type Node = JsIfStatement;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let entry_block = builder.cursor();

        let consequent_start = builder.append_block();
        builder.set_cursor(consequent_start);

        Ok(Self {
            entry_block,
            consequent_start,
            consequent_end: None,
            alt_block: None,
        })
    }

    fn exit(
        self,
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let consequent_block = (
            self.consequent_start,
            self.consequent_end.unwrap_or_else(|| builder.cursor()),
        );

        let alt_block = self.alt_block;

        let next_block = builder.append_block();

        builder.set_cursor(self.entry_block);

        if let Some((alt_start, alt_end)) = alt_block {
            builder
                .append_jump(true, consequent_block.0)
                .with_node(node.test()?.into_syntax());
            builder.append_jump(false, alt_start);

            builder.set_cursor(alt_end);
            builder.append_jump(false, next_block);
        } else {
            builder
                .append_jump(true, consequent_block.0)
                .with_node(node.test()?.into_syntax());
            builder.append_jump(false, next_block);
        }

        builder.set_cursor(consequent_block.1);
        builder.append_jump(false, next_block);

        builder.set_cursor(next_block);

        Ok(())
    }
}

pub(super) struct ElseVisitor {
    consequent_block: BlockId,
    alt_block: BlockId,
}

impl<B> NodeVisitor<B> for ElseVisitor {
    type Node = JsElseClause;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let consequent_block = builder.cursor();

        let alt_block = builder.append_block();
        builder.set_cursor(alt_block);

        Ok(Self {
            consequent_block,
            alt_block,
        })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<()> {
        let if_state = stack.read_top::<IfVisitor>()?;

        if_state.consequent_end = Some(self.consequent_block);
        if_state.alt_block = Some((self.alt_block, builder.cursor()));

        Ok(())
    }
}

pub(super) struct SwitchVisitor {
    entry_block: BlockId,
    label: Option<JsSyntaxToken>,
    break_block: BlockId,
    is_first: bool,
}

impl<B> NodeVisitor<B> for SwitchVisitor {
    type Node = JsSwitchStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let entry_block = builder.cursor();
        let break_block = builder.append_block();

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        Ok(Self {
            entry_block,
            label,
            break_block,
            is_first: true,
        })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            entry_block,
            break_block,
            ..
        } = self;

        builder.append_jump(false, break_block);

        builder.set_cursor(entry_block);
        builder.append_jump(false, break_block);

        builder.set_cursor(break_block);

        Ok(())
    }
}

pub(super) struct CaseVisitor;

impl<B> NodeVisitor<B> for CaseVisitor {
    type Node = JsAnySwitchClause;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        let case_block = builder.append_block();

        let switch_stmt = stack.read_top::<SwitchVisitor>()?;

        if !switch_stmt.is_first {
            builder.append_jump(false, case_block);
        } else {
            switch_stmt.is_first = false;
        }

        builder.set_cursor(switch_stmt.entry_block);

        match node {
            JsAnySwitchClause::JsCaseClause(node) => {
                builder
                    .append_jump(true, case_block)
                    .with_node(node.test()?.into_syntax());
            }
            JsAnySwitchClause::JsDefaultClause(_) => {
                builder.append_jump(false, case_block);
            }
        }

        builder.set_cursor(case_block);

        Ok(Self)
    }
}

pub(super) struct ForVisitor {
    label: Option<JsSyntaxToken>,
    continue_block: BlockId,
    break_block: BlockId,
    cond_block: BlockId,
    loop_block: BlockId,
}

impl<B> NodeVisitor<B> for ForVisitor {
    type Node = JsForStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        // Immediately evaluate the initializer statement
        if let Some(initializer) = node.initializer() {
            builder
                .append_statement()
                .with_node(initializer.into_syntax());
        }

        // Create the condition block and unconditionally jump to it
        let cond_block = builder.append_block();
        builder.append_jump(false, cond_block);

        // Create the continue block and break block immediately
        let continue_block = builder.append_block();
        let break_block = builder.append_block();

        // Fill the continue block
        builder.set_cursor(continue_block);

        if let Some(update) = node.update() {
            builder.append_statement().with_node(update.into_syntax());
        }

        builder.append_jump(false, cond_block);

        // Create the loop block and fill it with the loop body statement
        let loop_block = builder.append_block();
        builder.set_cursor(loop_block);

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        Ok(Self {
            label,
            continue_block,
            break_block,
            cond_block,
            loop_block,
        })
    }

    fn exit(
        self,
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            continue_block,
            break_block,
            cond_block,
            loop_block,
            ..
        } = self;

        // Insert an unconditional jump to the continue block at the end of the loop body
        builder.append_jump(false, continue_block);

        // Write the condition block
        builder.set_cursor(cond_block);

        if let Some(test) = node.test() {
            builder
                .append_jump(true, loop_block)
                .with_node(test.syntax().clone());
        }

        builder.append_jump(false, break_block);

        // Set the cursor to the break block and move of to the next statement
        builder.set_cursor(break_block);

        Ok(())
    }
}

pub(super) struct ForInVisitor {
    label: Option<JsSyntaxToken>,
    continue_block: BlockId,
    break_block: BlockId,
}

impl<B> NodeVisitor<B> for ForInVisitor {
    type Node = JsForInStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let continue_block = builder.append_block();
        let loop_block = builder.append_block();
        let break_block = builder.append_block();

        builder.append_jump(false, continue_block);

        builder.set_cursor(continue_block);
        builder
            .append_jump(true, loop_block)
            .with_node(node.initializer()?.into_syntax());

        builder.append_jump(false, break_block);

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        builder.set_cursor(loop_block);

        Ok(Self {
            label,
            continue_block,
            break_block,
        })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            continue_block,
            break_block,
            ..
        } = self;

        builder.append_jump(false, continue_block);

        builder.set_cursor(break_block);

        Ok(())
    }
}

pub(super) struct ForOfVisitor {
    label: Option<JsSyntaxToken>,
    continue_block: BlockId,
    break_block: BlockId,
}

impl<B> NodeVisitor<B> for ForOfVisitor {
    type Node = JsForOfStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let continue_block = builder.append_block();
        let loop_block = builder.append_block();
        let break_block = builder.append_block();

        builder.append_jump(false, continue_block);

        builder.set_cursor(continue_block);
        builder
            .append_jump(true, loop_block)
            .with_node(node.initializer()?.into_syntax());

        builder.append_jump(false, break_block);

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        builder.set_cursor(loop_block);

        Ok(Self {
            label,
            continue_block,
            break_block,
        })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            continue_block,
            break_block,
            ..
        } = self;

        builder.append_jump(false, continue_block);

        builder.set_cursor(break_block);

        Ok(())
    }
}

pub(super) struct WhileVisitor {
    label: Option<JsSyntaxToken>,
    continue_block: BlockId,
    break_block: BlockId,
    loop_block: BlockId,
}

impl<B> NodeVisitor<B> for WhileVisitor {
    type Node = JsWhileStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        // Create the continue and break blocks
        let continue_block = builder.append_block();
        let break_block = builder.append_block();

        // Unconditionally jump to the continue block
        builder.append_jump(false, continue_block);

        // Create the loop block and fill it with the loop body statement
        let loop_block = builder.append_block();
        builder.set_cursor(loop_block);

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        Ok(Self {
            label,
            continue_block,
            break_block,
            loop_block,
        })
    }

    fn exit(
        self,
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            continue_block,
            break_block,
            loop_block,
            ..
        } = self;

        // Insert an unconditional jump to the continue block at the end of the loop body
        builder.append_jump(false, continue_block);

        // Write the continue block
        builder.set_cursor(continue_block);
        builder
            .append_jump(true, loop_block)
            .with_node(node.test()?.into_syntax());

        builder.append_jump(false, break_block);

        // Set the cursor to the break block and move of to the next statement
        builder.set_cursor(break_block);

        Ok(())
    }
}

pub(super) struct DoWhileVisitor {
    label: Option<JsSyntaxToken>,
    continue_block: BlockId,
    break_block: BlockId,
    body_block: BlockId,
}

impl<B> NodeVisitor<B> for DoWhileVisitor {
    type Node = JsDoWhileStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let body_block = builder.append_block();

        // Unconditionally jump into the loop
        builder.append_jump(false, body_block);

        let continue_block = builder.append_block();
        let break_block = builder.append_block();

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        // Fill the body block
        builder.set_cursor(body_block);

        Ok(Self {
            label,
            continue_block,
            break_block,
            body_block,
        })
    }

    fn exit(
        self,
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            continue_block,
            break_block,
            body_block,
            ..
        } = self;

        // Insert an implicit jump to the continue block at the end of the loop
        builder.append_jump(false, continue_block);

        // Fill the continue block
        builder.set_cursor(continue_block);
        builder
            .append_jump(true, body_block)
            .with_node(node.test()?.into_syntax());

        builder.append_jump(false, break_block);

        // Set the cursor to the break block and move to the next statement
        builder.set_cursor(break_block);

        Ok(())
    }
}

pub(super) struct BreakVisitor;

impl<B> NodeVisitor<B> for BreakVisitor {
    type Node = JsBreakStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        state: StatementStack,
    ) -> SyntaxResult<Self> {
        let label = node.label_token();

        let break_block = state
            .stack
            .iter()
            .rev()
            .take_while(|(type_id, _)| *type_id != TypeId::of::<VisitorAdapter<FunctionVisitor>>())
            .find_map(|(type_id, index)| {
                let (block_label, block) = if let Some(visitor) =
                    state.try_downcast::<ForVisitor>(*type_id, *index)
                {
                    (visitor.label.as_ref(), visitor.break_block)
                } else if let Some(visitor) = state.try_downcast::<ForInVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.break_block)
                } else if let Some(visitor) = state.try_downcast::<ForOfVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.break_block)
                } else if let Some(visitor) = state.try_downcast::<WhileVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.break_block)
                } else if let Some(visitor) = state.try_downcast::<DoWhileVisitor>(*type_id, *index)
                {
                    (visitor.label.as_ref(), visitor.break_block)
                } else if let Some(visitor) = state.try_downcast::<SwitchVisitor>(*type_id, *index)
                {
                    (visitor.label.as_ref(), visitor.break_block)
                } else if let Some(visitor) = state.try_downcast::<BlockVisitor>(*type_id, *index) {
                    let (label, block) = visitor.break_block.as_ref()?;
                    (Some(label), *block)
                } else {
                    return None;
                };

                match (block_label, &label) {
                    (Some(a), Some(b)) => {
                        if a.text_trimmed() == b.text_trimmed() {
                            Some(block)
                        } else {
                            None
                        }
                    }

                    (None, None) => Some(block),
                    _ => None,
                }
            })
            .ok_or(SyntaxError::MissingRequiredChild)?;

        builder
            .append_jump(false, break_block)
            .with_node(node.into_syntax());

        Ok(Self)
    }
}

pub(super) struct ContinueVisitor;

impl<B> NodeVisitor<B> for ContinueVisitor {
    type Node = JsContinueStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        state: StatementStack,
    ) -> SyntaxResult<Self> {
        let label = node.label_token();

        let continue_block = state
            .stack
            .iter()
            .rev()
            .take_while(|(type_id, _)| *type_id != TypeId::of::<VisitorAdapter<FunctionVisitor>>())
            .find_map(|(type_id, index)| {
                let (block_label, block) = if let Some(visitor) =
                    state.try_downcast::<ForVisitor>(*type_id, *index)
                {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<ForInVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<ForOfVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<WhileVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<DoWhileVisitor>(*type_id, *index)
                {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else {
                    return None;
                };

                match (block_label, &label) {
                    (Some(a), Some(b)) => {
                        if a.text_trimmed() == b.text_trimmed() {
                            Some(block)
                        } else {
                            None
                        }
                    }

                    (None, None) => Some(block),
                    _ => None,
                }
            })
            .ok_or(SyntaxError::MissingRequiredChild)?;

        builder
            .append_jump(false, continue_block)
            .with_node(node.into_syntax());

        Ok(Self)
    }
}

pub(super) struct ReturnVisitor;

impl<B> NodeVisitor<B> for ReturnVisitor {
    type Node = JsReturnStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        builder.append_return().with_node(node.into_syntax());

        Ok(Self)
    }
}

pub(super) struct ThrowVisitor;

impl<B> NodeVisitor<B> for ThrowVisitor {
    type Node = JsThrowStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        builder.append_return().with_node(node.into_syntax());

        Ok(Self)
    }
}
