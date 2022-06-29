use rome_rowan::{Language, SyntaxNode};

use crate::{BasicBlock, ControlFlowGraph, Instruction, InstructionKind};

/// Identifier for a block in a [ControlFlowGraph]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    index: u32,
}

impl BlockId {
    /// Returns the index of the block in the function
    pub fn index(self) -> u32 {
        self.index
    }
}

/// Helper struct for building an instance of [ControlFlowGraph], the builder
/// keeps track of an "insertion cursor" within the graph where [Instruction]
/// should be added
pub struct FunctionBuilder<L: Language> {
    result: ControlFlowGraph<L>,
    block_cursor: BlockId,
}

impl<L: Language> Default for FunctionBuilder<L> {
    fn default() -> Self {
        Self {
            result: ControlFlowGraph::new(),
            block_cursor: BlockId { index: 0 },
        }
    }
}

impl<L: Language> FunctionBuilder<L> {
    /// Finishes building the function
    pub fn finish(self) -> ControlFlowGraph<L> {
        self.result
    }

    /// Allocate a new empty block, returning its [BlockId]
    pub fn append_block(&mut self) -> BlockId {
        let index = self
            .result
            .blocks
            .len()
            .try_into()
            .expect("BlockId overflow");

        self.result.blocks.push(BasicBlock::new());
        BlockId { index }
    }

    /// Get the [BlockId] at the current position of the cursor
    pub fn cursor(&self) -> BlockId {
        self.block_cursor
    }

    /// Move the cursor to the end of `block`
    pub fn set_cursor(&mut self, block: BlockId) {
        debug_assert!(block.index < self.result.blocks.len() as u32);
        self.block_cursor = block;
    }

    /// Add a block to the list of entry points for the function
    pub fn add_entry_block(&mut self, block: BlockId) {
        self.result.entry_blocks.push(block.index());
    }

    /// Insert an instruction at the current position of the cursor
    fn append_instruction(&mut self, kind: InstructionKind) -> InstructionBuilder<L> {
        let index = self.block_cursor.index as usize;
        let block = &mut self.result.blocks[index];

        let index = block.instructions.len();
        block.instructions.push(Instruction { kind, node: None });

        InstructionBuilder(&mut block.instructions[index])
    }

    pub fn append_statement(&mut self) -> InstructionBuilder<L> {
        self.append_instruction(InstructionKind::Statement)
    }

    pub fn append_return(&mut self) -> InstructionBuilder<L> {
        self.append_instruction(InstructionKind::Return)
    }

    pub fn append_jump(&mut self, conditional: bool, block: BlockId) -> InstructionBuilder<L> {
        self.append_instruction(InstructionKind::Jump { conditional, block })
    }
}

pub struct InstructionBuilder<'a, L: Language>(&'a mut Instruction<L>);

impl<'a, L: Language> InstructionBuilder<'a, L> {
    pub fn with_node(mut self, node: SyntaxNode<L>) -> Self {
        self.0.node = Some(node);
        self
    }
}
