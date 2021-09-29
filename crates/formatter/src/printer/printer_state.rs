use crate::printer::cst_builder::{CSTBuilder, CSTBuilderSnapshot};

/// Printer state that is global to all tokens.
/// Stores the result of the print operation (buffer and mappings) and at what
/// position the printer currently is.
#[derive(Debug, Default)]
pub(crate) struct PrinterState {
	pub pending_indent: u16,
	pub pending_spaces: u16,
	pub line_width: usize,
	pub cst: CSTBuilder,
}

impl PrinterState {
	/// Allows creating a snapshot of the state that can be restored using [restore]
	pub fn snapshot(&self) -> PrinterStateSnapshot {
		PrinterStateSnapshot {
			pending_spaces: self.pending_spaces,
			pending_indents: self.pending_indent,
			line_width: self.line_width,
			cst_builder_snapshot: self.cst.snapshot(),
		}
	}

	/// Restores the printer state to the state stored in the snapshot.
	pub fn restore(&mut self, snapshot: PrinterStateSnapshot) {
		self.pending_spaces = snapshot.pending_spaces;
		self.pending_indent = snapshot.pending_indents;
		self.line_width = snapshot.line_width;
		self.cst.restore(snapshot.cst_builder_snapshot);
	}
}

/// Snapshot of a printer state.
pub(crate) struct PrinterStateSnapshot {
	pending_indents: u16,
	pending_spaces: u16,
	line_width: usize,
	cst_builder_snapshot: CSTBuilderSnapshot,
}
