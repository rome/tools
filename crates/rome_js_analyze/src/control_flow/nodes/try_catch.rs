use rome_control_flow::builder::BlockId;
use rome_js_syntax::{JsCatchClause, JsFinallyClause, JsTryFinallyStatement};
use rome_rowan::SyntaxResult;

use crate::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::control_flow) struct TryFinallyVisitor {
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

pub(in crate::control_flow) struct CatchVisitor {
    /// If this catch clause is part of a simple try-catch statement, this
    /// contains the block to execute after this node
    next_block: Option<BlockId>,
}

impl<B> NodeVisitor<B> for CatchVisitor {
    type Node = JsCatchClause;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        // If this case clause is part of a try-finally statement, register the
        // catch block to the parent visitor
        if let Ok(try_finally_stmt) = stack.read_top::<TryFinallyVisitor>() {
            let finally_block = try_finally_stmt.finally_block.get_or_insert_with(|| {
                let finally_block = builder.append_block();
                builder.add_entry_block(finally_block);
                finally_block
            });

            // Implicit jump from the end of the try block to the finally block
            builder.append_jump(false, *finally_block);

            let catch_block = builder.append_block();
            builder.add_entry_block(catch_block);
            builder.set_cursor(catch_block);

            Ok(Self { next_block: None })
        } else {
            // Otherwise this is a simple try-catch statement, it doesn't need
            // to have its own visitor since the required logic can be
            //implemented entirely within the catch visitor

            // Cursor is at the end of the try block, jump to the next block
            let next_block = builder.append_block();
            builder.append_jump(false, next_block);

            // Create the catch block, mark it as entry point and start writing
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
            // Implicitly jump from the end of the catch block to the finally block
            // SAFETY: The `finally_block` has been created when this node was entered
            builder.append_jump(false, try_finally_stmt.finally_block.unwrap());

            Ok(())
        } else {
            // SAFETY: `next_block` is always set to `Some` in
            // `CatchVisitor::enter` if the parent is not a try-finally statement
            let next_block = next_block.unwrap();

            // Insert a jump to the next block at the end of the catch block
            builder.append_jump(false, next_block);

            // Continue writing on the next block
            builder.set_cursor(next_block);

            Ok(())
        }
    }
}

pub(in crate::control_flow) struct FinallyVisitor;

impl<B> NodeVisitor<B> for FinallyVisitor {
    type Node = JsFinallyClause;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        let try_finally_stmt = stack.read_top::<TryFinallyVisitor>()?;

        // `finally_block` is initialized to `None` in `TryFinallyVisitor::enter`,
        // if it's set to `Some` by the time the traversal reached the finally
        // clause this means an intermediate catch clause has allocated it
        // in-between (in `CatchVisitor::enter`)
        let has_catch = try_finally_stmt.finally_block.is_some();
        let finally_block = try_finally_stmt.finally_block.get_or_insert_with(|| {
            let finally_block = builder.append_block();
            builder.add_entry_block(finally_block);
            finally_block
        });

        // Append an implicit jump from the catch block to the finally block if it exists
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
