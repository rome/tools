use rome_control_flow::builder::BlockId;
use rome_js_syntax::{JsElseClause, JsIfStatement};
use rome_rowan::{AstNode, SyntaxResult};

use crate::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::control_flow) struct IfVisitor {
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

pub(in crate::control_flow) struct ElseVisitor {
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
