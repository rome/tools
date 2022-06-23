use std::collections::VecDeque;

use roaring::bitmap::RoaringBitmap;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_control_flow::InstructionKind;
use rome_js_syntax::TextRange;

use crate::control_flow::ControlFlowGraph;

declare_rule! {
    /// Disallow unreachable code
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function example() {
    ///     return;
    ///     neverCalled();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function example() {
    ///     for(let i = 0; i < 10; ++i) {
    ///         break;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function example() {
    ///     for(const key in value) {
    ///         continue;
    ///         neverCalled();
    ///     }
    /// }
    /// ```
    pub(crate) NoDeadCode = "noDeadCode"
}

impl Rule for NoDeadCode {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = ControlFlowGraph;
    type State = TextRange;
    type Signals = Vec<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals = Vec::new();

        let cfg = ctx.query();

        // Perform a simple reachability analysis on the control flow graph by
        // traversing the function starting at the entry points
        let mut reachable_blocks = RoaringBitmap::new();
        let mut queue = VecDeque::new();

        for index in &cfg.entry_blocks {
            reachable_blocks.insert(*index);
            queue.push_back(*index);
        }

        while let Some(index) = queue.pop_front() {
            let index = index as usize;
            let block = &cfg.blocks[index];

            // Tracks whether this block is "terminated", if an instruction
            // that unconditionally aborts the control flow of this block has
            // been encountered
            let mut has_terminator = false;

            for inst in &block.instructions {
                // If this block is terminated, mark this instruction as unreachable and continue
                if has_terminator {
                    if let Some(node) = &inst.node {
                        signals.push(node.text_trimmed_range());
                    }
                    continue;
                }

                match inst.kind {
                    InstructionKind::Statement => {}
                    InstructionKind::Jump { conditional, block } => {
                        // Insert an edge if this jump is reachable
                        if reachable_blocks.insert(block.index()) {
                            queue.push_back(block.index());
                        }

                        // Jump is a terminator instruction if it's unconditional
                        if !conditional {
                            has_terminator = true;
                        }
                    }
                    InstructionKind::Return => {
                        has_terminator = true;
                    }
                }
            }
        }

        // Detect blocks that were never reached by the above traversal
        for (index, block) in cfg.blocks.iter().enumerate() {
            let index = index as u32;
            if reachable_blocks.contains(index) {
                continue;
            }

            for inst in &block.instructions {
                if let Some(node) = &inst.node {
                    signals.push(node.text_trimmed_range());
                }
            }
        }

        // TODO: Merge adjacent ranges instead of emitting individual
        // diagnostics for each statement in an unreachable block
        signals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::warning(
                *state,
                markup! {
                    "This code is unreachable"
                },
            )
            .unnecessary(),
        )
    }
}
