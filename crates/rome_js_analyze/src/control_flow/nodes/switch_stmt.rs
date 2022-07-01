use rome_control_flow::builder::BlockId;
use rome_js_syntax::{JsAnySwitchClause, JsLabeledStatement, JsSwitchStatement, JsSyntaxToken};
use rome_rowan::{AstNode, SyntaxResult};

use crate::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::control_flow) struct SwitchVisitor {
    entry_block: BlockId,
    pub(super) label: Option<JsSyntaxToken>,
    pub(super) break_block: BlockId,
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

pub(in crate::control_flow) struct CaseVisitor;

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
            JsAnySwitchClause::JsDefaultClause(node) => {
                builder
                    .append_jump(false, case_block)
                    .with_node(node.default_token()?);
            }
        }

        builder.set_cursor(case_block);

        Ok(Self)
    }
}
