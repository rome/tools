use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use rome_rowan::{Language, SyntaxNode};

pub mod builder;

use crate::builder::BlockId;

/// The [ControlFlowGraph] is an auxiliary data structure to the syntax tree,
/// representing the execution order of statements and expressions in a given
/// function as a graph of [BasicBlock]
#[derive(Debug, Clone)]
pub struct ControlFlowGraph<L: Language> {
    /// List of blocks that make up this function
    pub blocks: Vec<BasicBlock<L>>,
    /// Indices of the block that act as "entry point" for the function,
    /// defined as points that the control flow may arbitrarily jump into
    ///
    /// Currently this is used for the first block in the function, as well as
    /// `catch` and `finally` clauses to model the exception control flow
    /// possibly diverging into these
    pub entry_blocks: Vec<u32>,
}

impl<L: Language> ControlFlowGraph<L> {
    fn new() -> Self {
        ControlFlowGraph {
            blocks: vec![BasicBlock::new()],
            entry_blocks: vec![0],
        }
    }
}

/// A basic block represents an atomic unit of control flow, a flat list of
/// instructions that will be executed linearly when a function is run.
///
/// Note, however, that while the instructions that comprise a basic block are
/// guaranteed to be executed in order from the start towards the end, the
/// block may not be executed entirely if a jump or return instruction is
/// encountered.
#[derive(Debug, Clone)]
pub struct BasicBlock<L: Language> {
    pub instructions: Vec<Instruction<L>>,
}

impl<L: Language> BasicBlock<L> {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

/// Instructions are used to represent statements or expressions being executed
/// as side effects, as well as potential divergence points for the control flow.
///
/// Each node has an associated kind, as well as an optional syntax node: the
/// node is useful to emit diagnostics but does not have a semantic value, and
/// is optional. The code generating the control flow graph may emit
/// instructions that do not correspond to any node in the syntax tree, to model
/// the control flow of the program accurately.
#[derive(Debug, Clone)]
pub struct Instruction<L: Language> {
    pub kind: InstructionKind,
    pub node: Option<SyntaxNode<L>>,
}

/// The different types of supported [Instruction]
#[derive(Copy, Clone, Debug)]
pub enum InstructionKind {
    /// Indicates the [SyntaxNode] associated with this instruction is to be
    /// evaluated at this point in the program
    Statement,
    /// This instruction may cause the control flow to diverge towards `block`,
    /// either unconditionally if `conditional` is set to `false`, or after
    /// evaluating the associated syntax node otherwise
    Jump { conditional: bool, block: BlockId },
    /// This instruction causes the control flow to unconditionally abort the
    /// execution of the function, for example is JavaScript this can be triggered by a `return` or `throw` statement
    Return,
}

/// The Display implementation for [ControlFlowGraph] prints a flowchart in
/// mermaid.js syntax
///
/// By default the graph is printed in "simple" mode where each basic block is
/// represented as a node in the graph:
///
/// ```mermaid
/// flowchart TB
///     block_0["<b>block_0</b><br/>Statement(JS_VARIABLE_DECLARATION 38..47)<br/>Jump { block: 1 }"]
///     block_1["<b>block_1</b><br/>Jump { condition: JS_BINARY_EXPRESSION 49..58, block: 2 }<br/>Jump { block: 3 }"]
///     block_2["<b>block_2</b><br/>Statement(JS_POST_UPDATE_EXPRESSION 60..63)"]
///     block_3["<b>block_3</b><br/>Statement(JS_EXPRESSION_STATEMENT 260..277)"]
///     
///     block_0 --> block_1
///     block_1 -- "JS_BINARY_EXPRESSION 49..58" --> block_2
///     block_1 --> block_3
/// ```
///
/// However the graph can also be printed in "detailed" mode by formatting it
/// in alternate mode using `{:#}`, this will print each basic block as a
/// subgraph instead:
///
/// ```mermaid
/// flowchart TB
///     subgraph block_0
///         direction TB
///         block_0_inst_0["Statement(JS_VARIABLE_DECLARATION 38..47)"]
///         block_0_inst_0 --> block_0_inst_1["Jump { block: 1 }"]
///     end
///     subgraph block_1
///         direction TB
///         block_1_inst_0["Jump { condition: JS_BINARY_EXPRESSION 49..58, block: 2 }"]
///         block_1_inst_0 --> block_1_inst_1["Jump { block: 3 }"]
///     end
///     subgraph block_2
///         direction TB
///         block_2_inst_0["Statement(JS_POST_UPDATE_EXPRESSION 60..63)"]
///     end
///     subgraph block_3
///         direction TB
///         block_3_inst_0["Statement(JS_EXPRESSION_STATEMENT 260..277)"]
///     end
///
///     block_0_inst_1 --> block_1
///     block_1_inst_0 -- "JS_BINARY_EXPRESSION 49..58" --> block_2
///     block_1_inst_1 --> block_3
/// ```
impl<L: Language> Display for ControlFlowGraph<L> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "flowchart TB")?;

        let mut links = HashMap::new();
        for (id, block) in self.blocks.iter().enumerate() {
            if fmt.alternate() {
                writeln!(fmt, "    subgraph block_{id}")?;
                writeln!(fmt, "        direction TB")?;
            } else {
                write!(fmt, "    block_{id}[\"<b>block_{id}</b><br/>")?;
            }

            for (index, inst) in block.instructions.iter().enumerate() {
                if fmt.alternate() {
                    write!(fmt, "        ")?;

                    if let Some(index) = index.checked_sub(1) {
                        write!(fmt, "block_{id}_inst_{index} --> ")?;
                    }

                    writeln!(fmt, "block_{id}_inst_{index}[\"{inst}\"]")?;
                } else {
                    if index > 0 {
                        write!(fmt, "<br/>")?;
                    }

                    write!(fmt, "{inst}")?;
                }

                if let InstructionKind::Jump { conditional, block } = inst.kind {
                    links.insert(
                        (id, index, block.index()),
                        inst.node
                            .as_ref()
                            .filter(|_| conditional)
                            .map(|node| (node.kind(), node.text_trimmed_range())),
                    );
                }
            }

            if fmt.alternate() {
                writeln!(fmt, "    end")?;
            } else {
                writeln!(fmt, "\"]")?;
            }
        }

        writeln!(fmt)?;

        for ((id, index, to), condition) in links {
            write!(fmt, "    block_{id}")?;

            if fmt.alternate() {
                write!(fmt, "_inst_{index}")?;
            }

            if let Some((cond, range)) = condition {
                write!(fmt, " -- \"{cond:?} {range:?}\"")?;
            }

            writeln!(fmt, " --> block_{to}")?;
        }

        Ok(())
    }
}

impl<L: Language> Display for Instruction<L> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.kind {
            InstructionKind::Statement => {
                if let Some(node) = &self.node {
                    write!(
                        fmt,
                        "Statement({:?} {:?})",
                        node.kind(),
                        node.text_trimmed_range()
                    )
                } else {
                    write!(fmt, "Statement")
                }
            }

            InstructionKind::Jump {
                conditional: true,
                block,
            } if self.node.is_some() => {
                // SAFETY: Checked by the above call to `is_some`
                let node = self.node.as_ref().unwrap();
                write!(
                    fmt,
                    "Jump {{ condition: {:?} {:?}, block: {} }}",
                    node.kind(),
                    node.text_trimmed_range(),
                    block.index()
                )
            }

            InstructionKind::Jump { block, .. } => {
                write!(fmt, "Jump {{ block: {} }}", block.index())
            }

            InstructionKind::Return => {
                if let Some(node) = &self.node {
                    write!(
                        fmt,
                        "Return({:?} {:?})",
                        node.kind(),
                        node.text_trimmed_range()
                    )
                } else {
                    write!(fmt, "Return")
                }
            }
        }
    }
}
