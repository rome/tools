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

pub(in crate::control_flow) struct FinallyVisitor;

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
