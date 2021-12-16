use crate::{SyntaxKind, T};

// We need to keep context for regex literals
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LexerState {
	pub(crate) expr_allowed: bool,
	pub(crate) prev: Option<SyntaxKind>,
	pub(crate) had_linebreak: bool,
	pub(crate) ctx: Vec<Context>,
}

impl LexerState {
	pub(crate) fn new() -> Self {
		Self {
			expr_allowed: true,
			prev: None,
			had_linebreak: false,
			ctx: vec![Context::BraceStmt],
		}
	}

	pub(crate) fn is_in_template(&self) -> bool {
		self.ctx.last() == Some(&Context::Template)
	}

	pub(crate) fn update(&mut self, next: SyntaxKind) {
		self.expr_allowed = self.update_expr_allowed(next);
		self.prev = Some(next);
	}

	fn update_expr_allowed(&mut self, next: SyntaxKind) -> bool {
		if next.is_keyword() && self.prev == Some(T![.]) {
			return false;
		}

		match next {
			T![')'] | T!['}'] => {
				if self.ctx.len() == 1 {
					return true;
				}

				let closed = self
					.ctx
					.pop()
					.expect("Tried update state with ) or } but context is somehow empty");

				if closed == Context::BraceStmt && self.ctx.last() == Some(&Context::FnExpr) {
					self.ctx.pop();
					return false;
				}

				if closed == Context::TplInternal {
					return !self.is_in_template();
				}

				!ctx_is_expr(closed)
			}

			T![function] => {
				if self.expr_allowed
					&& ctx_is_brace_block(
						&self.ctx,
						self.prev,
						self.had_linebreak,
						self.expr_allowed,
					) {
					self.ctx.push(Context::FnExpr);
				}
				false
			}

			SyntaxKind::BACKTICK => {
				if let Some(Context::Template) = self.ctx.last() {
					self.ctx.pop();
				} else {
					self.ctx.push(Context::Template);
				}
				false
			}

			SyntaxKind::DOLLAR_CURLY => {
				self.ctx.push(Context::TplInternal);
				true
			}

			// TODO: es6 for of
			T![ident] => self.prev == Some(T![var]) && self.had_linebreak,

			T!['{'] => {
				let next = if ctx_is_brace_block(
					&self.ctx,
					self.prev,
					self.had_linebreak,
					self.expr_allowed,
				) {
					Context::BraceStmt
				} else {
					Context::BraceExpr
				};
				self.ctx.push(next);
				true
			}

			T!['('] => {
				let next = match self.prev {
					Some(t) if t.is_keyword() => match t {
						T![if] | T![with] | T![while] => Context::ParenStmt { for_loop: false },
						T![for] => Context::ParenStmt { for_loop: true },
						_ => Context::ParenExpr,
					},
					_ => Context::ParenExpr,
				};
				self.ctx.push(next);
				true
			}

			T![++] | T![--] => self.expr_allowed,

			_ => next.is_before_expr(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Context {
	BraceStmt,
	BraceExpr,
	TplInternal,
	ParenStmt { for_loop: bool },
	ParenExpr,
	Template,
	FnExpr,
}

fn ctx_is_expr(ctx: Context) -> bool {
	matches!(
		ctx,
		Context::BraceExpr
			| Context::TplInternal
			| Context::ParenExpr
			| Context::Template
			| Context::FnExpr
	)
}

fn ctx_is_brace_block(
	ctx: &[Context],
	prev: Option<SyntaxKind>,
	had_linebreak: bool,
	expr_allowed: bool,
) -> bool {
	if let Some(T![:]) = prev {
		match ctx.last() {
			Some(Context::BraceStmt) => return true,
			Some(Context::BraceExpr) => return false,
			_ => {}
		}
	}

	match prev {
		Some(T![return]) => had_linebreak,

		Some(T![else]) | Some(T![;]) | Some(T![')']) | None => true,

		Some(T!['{']) => ctx.last() == Some(&Context::BraceStmt),

		_ => !expr_allowed,
	}
}
