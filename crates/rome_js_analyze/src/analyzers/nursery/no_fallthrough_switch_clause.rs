use rome_control_flow::{ExceptionHandlerKind, InstructionKind};
use rome_rowan::{AstNode, AstNodeList, NodeOrToken};

use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsSwitchClause, JsLanguage, JsSwitchStatement};
use rustc_hash::FxHashSet;

use crate::ControlFlowGraph;

declare_rule! {
    /// Disallow fallthrough of `switch` clauses.
    ///
    /// Switch clauses in `switch` statements fall through by default.
    /// This can lead to unexpected behavior when forgotten.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-fallthrough
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch(bar) {
    /// 	case 0:
    /// 		a();
    /// 	case 1:
    /// 		b()
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// switch(foo) {
    /// 	case 1:
    /// 		doSomething();
    /// 		break;
    /// 	case 2:
    /// 		doSomething();
    /// }
    /// ```
    ///
    pub(crate) NoFallthroughSwitchClause {
        version: "13.0.0",
        name: "noFallthroughSwitchClause",
        recommended: false,
    }
}

impl Rule for NoFallthroughSwitchClause {
    type Query = ControlFlowGraph;
    type State = AnyJsSwitchClause;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let cfg = ctx.query();
        let mut fallthrough: Vec<AnyJsSwitchClause> = Vec::new();
        // block to process.
        let mut block_stack = Vec::new();
        let mut visited_blocks = FxHashSet::default();
        block_stack.push(0u32);
        visited_blocks.insert(0u32);
        // Traverse the control flow graph and search for switch statements.
        while let Some(block_index) = block_stack.pop() {
            // SAFETY: this is a safe conversion because it is already an index for `cfg.blocks`.
            let block_index = block_index as usize;
            let Some(block) = cfg.blocks.get(block_index) else {
                continue;
            };
            // Register exception handlers as blocks to process
            // Ignore finally handler: they are already in the Control Flow Graph.
            for exception_handler in block
                .exception_handlers
                .iter()
                .filter(|x| matches!(x.kind, ExceptionHandlerKind::Catch))
            {
                if visited_blocks.insert(exception_handler.target) {
                    block_stack.push(exception_handler.target);
                }
            }
            // Traverse the instructions of the block searching for a switch statement.
            let mut is_switch = false;
            let mut has_default_clause = false;
            let mut switch_clause_blocks = FxHashSet::default();
            for instruction in block.instructions.iter() {
                match instruction.kind {
                    InstructionKind::Statement => {
                        if let Some(node) = &instruction.node {
                            if let Some(switch_stmt) =
                                node.parent().and_then(JsSwitchStatement::cast)
                            {
                                if is_switch {
                                    unreachable!("A block cannot contain two switch statements.")
                                }
                                is_switch = true;
                                has_default_clause =
                                    switch_stmt.cases().iter().any(|switch_clause| {
                                        switch_clause.as_js_default_clause().is_some()
                                    });
                            }
                        }
                    }
                    InstructionKind::Jump {
                        conditional, block, ..
                    } => {
                        let jump_block_index = block.index();
                        // Avoid cycles and redundant checks.
                        if visited_blocks.insert(jump_block_index) {
                            block_stack.push(jump_block_index);
                        }
                        // If the last statement is the discriminant of a switch statements,
                        // then all succeeding jumps are jumps to switch clauses.
                        // The unconditional jump jumps to the default switch clause if it exists.
                        if is_switch && (conditional || has_default_clause) {
                            // Take the unconditional jump into account only if a default clause is present.
                            switch_clause_blocks.insert(jump_block_index);
                        }
                        if !conditional {
                            // The next instructions are unreachable.
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        // The next instructions are unreachable.
                        break;
                    }
                }
            }
            if !switch_clause_blocks.is_empty() {
                // Analyze the found switch statement to detect any fallthrough.
                let mut new_fallthrough =
                    get_switch_clause_fallthrough(&switch_clause_blocks, &visited_blocks, cfg);
                fallthrough.append(&mut new_fallthrough);
            }
        }
        fallthrough
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.syntax().text_trimmed_range(),
                markup! {
                    "This case is falling through to the next case."
                },
            )
            .note(markup! {
                "Add a `break` or `return` statement to the end of this case to prevent fallthrough."
            }),
        )
    }
}

fn get_switch_clause_fallthrough(
    switch_clause_blocks: &FxHashSet<u32>,
    visited_blocks: &FxHashSet<u32>,
    cfg: &rome_control_flow::ControlFlowGraph<JsLanguage>,
) -> Vec<AnyJsSwitchClause> {
    // Register all switch clauses as block to process.
    let mut block_stack: Vec<u32> = switch_clause_blocks.iter().copied().collect();
    let mut visited_blocks = visited_blocks.clone();
    let mut fallthrough = Vec::new();
    // Traverse the control flow graph
    while let Some(block_index) = block_stack.pop() {
        // SAFETY: this is a safe conversion because it is already an index for `cfg.blocks`.
        let block_index = block_index as usize;
        let Some(block) = cfg.blocks.get(block_index) else {
            continue;
        };
        // Register exception handlers as blocks to process
        // Ignore finally handler: they are already in the Control Flow Graph.
        for exception_handler in block
            .exception_handlers
            .iter()
            .filter(|x| matches!(x.kind, ExceptionHandlerKind::Catch))
        {
            if visited_blocks.insert(exception_handler.target) {
                block_stack.push(exception_handler.target);
            }
        }
        let mut first_statement = None;
        for instruction in block.instructions.iter() {
            match instruction.kind {
                InstructionKind::Statement => {
                    if first_statement.is_none() {
                        if let Some(NodeOrToken::Node(node)) = &instruction.node {
                            first_statement = Some(node);
                        }
                    }
                }
                InstructionKind::Jump {
                    conditional, block, ..
                } => {
                    let jump_block_index = block.index();
                    // Avoid cycles and redundant checks.
                    if visited_blocks.insert(jump_block_index) {
                        block_stack.push(jump_block_index);
                    }
                    if !conditional {
                        if switch_clause_blocks.contains(&jump_block_index) {
                            if let Some(last_statement) = first_statement {
                                // This is a fallthrough
                                if let Some(switch_clause) =
                                    last_statement.ancestors().find_map(AnyJsSwitchClause::cast)
                                {
                                    fallthrough.push(switch_clause);
                                }
                            }
                        }
                        // The next instructions are unreachable.
                        break;
                    }
                }
                InstructionKind::Return => {
                    // The next instructions are unreachable.
                    break;
                }
            }
        }
    }
    fallthrough
}
