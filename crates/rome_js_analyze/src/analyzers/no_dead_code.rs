use std::{cmp::Ordering, collections::VecDeque, vec::IntoIter};

use roaring::bitmap::RoaringBitmap;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_control_flow::InstructionKind;
use rome_js_syntax::{JsSyntaxElement, TextRange};
use rustc_hash::FxHashMap;

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
    type State = UnreachableRange;
    type Signals = UnreachableRanges;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut signals = UnreachableRanges::new();

        let cfg = ctx.query();

        // Perform a simple reachability analysis on the control flow graph by
        // traversing the function starting at the entry points
        // Each entry in the queue holds the state for a single linearly
        // independent path through the function as it gets built
        // incrementally: this state consist of the index of the next block to
        // visit, the set of all blocks already visited, and the current
        //terminating instruction for the path if one was encountered
        let mut queue = VecDeque::new();

        for index in &cfg.entry_blocks {
            queue.push_back((*index, RoaringBitmap::new(), None));
        }

        // This maps holds a list of "path state", the active terminator
        // intruction for each path that can reach the block
        let mut block_paths = FxHashMap::default();

        while let Some((index, mut visited, mut terminator)) = queue.pop_front() {
            // Add the block to the visited set for the path, and the current
            // state of the path to the global reachable blocks map
            visited.insert(index);

            block_paths
                .entry(index)
                .or_insert_with(Vec::new)
                .push(terminator);

            let index = index as usize;
            let block = &cfg.blocks[index];

            // Set to true if the `terminator` is found inside of this block
            let mut has_direct_terminator = false;

            for inst in &block.instructions {
                let node_range = inst.node.as_ref().map(|node| node.text_trimmed_range());

                // If this block has already ended, immediately mark this instruction as unreachable
                if let Some(terminator) = terminator.filter(|_| has_direct_terminator) {
                    if let Some(node) = &inst.node {
                        signals.push(node, terminator);
                    }
                }

                match inst.kind {
                    InstructionKind::Statement => {}
                    InstructionKind::Jump { conditional, block } => {
                        // Push the jump target block to the queue if it hasn't
                        // been visited yet in this path
                        if !visited.contains(block.index()) {
                            queue.push_back((block.index(), visited.clone(), terminator));
                        }

                        // Jump is a terminator instruction if it's unconditional
                        if terminator.is_none() && !conditional {
                            terminator = Some(node_range);
                            has_direct_terminator = true;
                        }
                    }
                    InstructionKind::Return => {
                        if terminator.is_none() {
                            terminator = Some(node_range);
                            has_direct_terminator = true;
                        }
                    }
                }
            }
        }

        // Detect unrechable blocks using the result of the above traversal
        'blocks: for (index, block) in cfg.blocks.iter().enumerate() {
            let index = index as u32;
            match block_paths.get(&index) {
                // Block has incoming paths, but may be unreachable if they all
                // have a dominating terminator intruction
                Some(paths) => {
                    let mut terminators = Vec::new();
                    for path in paths {
                        if let Some(terminator) = *path {
                            terminators.push(terminator);
                        } else {
                            // This path has no terminator, the block is reachable
                            continue 'blocks;
                        }
                    }

                    // Mark each instruction in the block as unreachable with
                    // the appropriate terminator labels
                    for inst in &block.instructions {
                        if let Some(node) = &inst.node {
                            for terminator in &terminators {
                                signals.push(node, *terminator);
                            }
                        }
                    }
                }
                // Block has no incoming paths, is completely cut off from the CFG
                // In theory this shouldn't happen as our CFG also stores
                // unreachable edges, if we get here there might be a bug in
                // the control flow analysis
                None => {
                    for inst in &block.instructions {
                        if let Some(node) = &inst.node {
                            // There is no incoming control flow so we can't
                            // determine a terminator instruction for this
                            // unreachable range
                            signals.push(node, None);
                        }
                    }
                }
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::warning(
            state.text_trimmed_range,
            markup! {
                "This code is unreachable"
            },
        )
        .unnecessary();

        // Pluralize and adapt the error message accordingly based on the
        // number and position of secondary labels
        match state.terminators.as_slice() {
            // The CFG didn't contain enough informations to determine a cause
            // for this range being unreachable
            [] => {}
            // A single node is responsible for this range being unreachable
            [node] => {
                if node.start() < state.text_trimmed_range.start() {
                    diagnostic = diagnostic
                        .secondary(*node, "This statement will abort control flow ...")
                        .primary("... before it can reach this code");
                } else {
                    diagnostic = diagnostic
                        .primary("This code will never be reached ...")
                        .secondary(
                            *node,
                            "... because this statement will abort control flow beforehand",
                        );
                }
            }
            // The range has two dominating terminator instructions
            [node_a, node_b] => {
                diagnostic = diagnostic
                    .secondary(*node_a, "Either this statement ...")
                    .secondary(*node_b, "... or this statement will abort control flow ...")
                    .primary("... before it can reach this code");
            }
            // The range has three or more dominating terminator instructions
            terminators => {
                // SAFETY: This substraction is safe since the match expression
                // ensures the slice has at least 3 elements
                let last = terminators.len() - 1;

                for (index, node) in terminators.iter().enumerate() {
                    if index == 0 {
                        diagnostic = diagnostic.secondary(*node, "Either this statement, ...");
                    } else if index == last {
                        diagnostic = diagnostic
                            .secondary(*node, "... or this statement will abort control flow ...");
                    } else {
                        diagnostic = diagnostic.secondary(*node, "... this statement, ...");
                    }
                }

                diagnostic = diagnostic.primary("... before it can reach this code");
            }
        }

        Some(diagnostic)
    }
}

/// Stores a list of unreachable code ranges, sorted in ascending source order
#[derive(Debug)]
pub(crate) struct UnreachableRanges {
    ranges: Vec<UnreachableRange>,
}

impl UnreachableRanges {
    fn new() -> Self {
        UnreachableRanges { ranges: Vec::new() }
    }

    fn push(&mut self, node: &JsSyntaxElement, terminator: Option<TextRange>) {
        let text_range = node.text_range();
        let text_trimmed_range = node.text_trimmed_range();

        // Perform a binary search on the ranges already in storage to find an
        // appropriate position for either merging or inserting the incoming range
        let insertion = self.ranges.binary_search_by(|entry| {
            if entry.text_range.end() < text_range.start() {
                Ordering::Less
            } else if text_range.end() < entry.text_range.start() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        match insertion {
            // The search returned an existing overlapping range, extend it to
            // cover the incoming range
            Ok(index) => {
                let entry = &mut self.ranges[index];
                entry.text_range = entry.text_range.cover(text_range);
                entry.text_trimmed_range = entry.text_trimmed_range.cover(text_trimmed_range);

                if let Some(terminator) = terminator {
                    // Terminator labels are also stored in ascending order to
                    // faciliate the generation of labels when the diagnostic
                    // gets emitted
                    let terminator_insertion = entry
                        .terminators
                        .binary_search_by_key(&terminator.start(), |node| node.start());

                    if let Err(index) = terminator_insertion {
                        entry.terminators.insert(index, terminator);
                    }
                }
            }
            // No overlapping range was found, insert at the appropriate
            // position to preserve the ordering instead
            Err(index) => {
                self.ranges.insert(
                    index,
                    UnreachableRange {
                        text_range,
                        text_trimmed_range,
                        terminators: terminator.into_iter().collect(),
                    },
                );
            }
        }
    }
}

impl IntoIterator for UnreachableRanges {
    type Item = UnreachableRange;
    type IntoIter = IntoIter<UnreachableRange>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}

/// Stores the trimmed and un-trimmed ranges for a block of unreachable source
/// code, along with a list of secondary labels pointing to the dominating
/// terminator instructions that cause it to be unreachable
#[derive(Debug)]
pub(crate) struct UnreachableRange {
    text_range: TextRange,
    text_trimmed_range: TextRange,
    terminators: Vec<TextRange>,
}
