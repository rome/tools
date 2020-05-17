/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Label, LabelKind} from "../tokenizer/state";
import {types as tt} from "../tokenizer/types";
import {Position, SourceLocation} from "@romejs/parser-core";
import {
	IndexTracker,
	isIdentifierChar,
	isIdentifierStart,
	keywordRelationalOperator,
	lineBreak,
	skipWhiteSpace,
} from "@romejs/js-parser-utils";
import {JSParser, OpeningContext} from "../parser";
import {
	AnyBindingPattern,
	AnyExpression,
	AnyForInOfStatement,
	AnyForStatement,
	AnyNode,
	AnyStatement,
	AnyTargetAssignmentPattern,
	AnyTargetBindingPattern,
	BindingIdentifier,
	BlockStatement,
	BreakStatement,
	CatchClause,
	ContinueStatement,
	DebuggerStatement,
	Directive,
	DoWhileStatement,
	EmptyStatement,
	ExpressionStatement,
	ForInStatement,
	ForOfStatement,
	ForStatement,
	FunctionDeclaration,
	FunctionExpression,
	FunctionHead,
	IfStatement,
	InterpreterDirective,
	LabeledStatement,
	Program,
	ReferenceIdentifier,
	ReturnStatement,
	SwitchCase,
	SwitchStatement,
	TSDeclareFunction,
	TSTypeParameterDeclaration,
	ThrowStatement,
	TryStatement,
	VariableDeclaration,
	VariableDeclarationKind,
	VariableDeclarationStatement,
	VariableDeclarator,
	WhileStatement,
	WithStatement,
} from "@romejs/js-ast";
import * as charCodes from "@romejs/string-charcodes";
import {nextToken, setStrict, skipLineComment} from "../tokenizer/index";
import {
	ParseExportResult,
	ParseImportResult,
	checkLVal,
	checkYieldAwaitInDefaultParams,
	maybeParseTSTypeParameters,
	parseBindingIdentifier,
	parseBindingListNonEmpty,
	parseClassDeclaration,
	parseExport,
	parseExpression,
	parseFunctionBodyAndFinish,
	parseIdentifier,
	parseImport,
	parseMaybeAssign,
	parseParenExpression,
	parseTSEnumDeclaration,
	parseTSTypeAnnotation,
	parseTSTypeExpressionStatement,
	parseTargetBindingPattern,
	toTargetAssignmentPattern,
} from "./index";
import {ob1Add, ob1Get0, ob1Inc, ob1Number0} from "@romejs/ob1";
import {descriptions} from "@romejs/diagnostics";

const loopLabel: Label = {kind: "loop"};
const switchLabel: Label = {kind: "switch"};

export function parseTopLevel(parser: JSParser): Program {
	const start = parser.getPosition();
	const openContext: OpeningContext = {
		name: "top-level",
		start,
		indent: ob1Number0,
		open: tt.eof,
		close: tt.eof,
	};

	// Parse the body, and catch fatal syntax errors

	// Get the first token
	nextToken(parser);

	const interpreter = parsePossibleInterpreterDirective(parser);
	const {body, directives} = parseBlockBody(parser, true, true, openContext);

	return parser.finishNode(
		start,
		{
			type: "Program",
			corrupt: parser.state.corrupt,
			body,
			directives,
			mtime: parser.mtime,
			diagnostics: parser.getDiagnostics(),
			filename: parser.filename,
			comments: parser.state.comments,
			sourceType: parser.sourceType,
			interpreter,
			syntax: Array.from(parser.syntax),
			hasHoistedVars: parser.state.hasHoistedVars,
		},
	);
}

export function parsePossibleInterpreterDirective(
	parser: JSParser,
): undefined | InterpreterDirective {
	// Check for #!
	if (
		parser.match(tt.hash) &&
		parser.input[ob1Get0(parser.state.endPos.index)] === "!"
	) {
		// Parse as a regular comment, we should abstract this logic
		// TODO this gets pushed to all the comments which is bad
		const comment = skipLineComment(parser, 2);

		// Advance to next token
		parser.next();

		return {
			type: "InterpreterDirective",
			value: comment.value,
			loc: comment.loc,
		};
	} else {
		return undefined;
	}
}

export function expressionStatementToDirective(
	parser: JSParser,
	stmt: ExpressionStatement,
): Directive {
	const expr = stmt.expression;

	const start = parser.getLoc(stmt).start;

	const raw = parser.getRawInput(
		parser.getLoc(expr).start.index,
		parser.getLoc(expr).end.index,
	);
	const val = raw.slice(1, -1); // remove quotes
	const end = parser.getLoc(stmt).end;

	return parser.finishNodeAt(
		start,
		end,
		{
			type: "Directive",
			value: val,
		},
	);
}

export function isLetStart(parser: JSParser, context?: string): boolean {
	if (!parser.isContextual("let")) {
		return false;
	}

	skipWhiteSpace.lastIndex = ob1Get0(parser.state.index);
	const skip = skipWhiteSpace.exec(parser.input);
	if (skip == null) {
		throw new Error("Should never be true");
	}

	const next = ob1Add(parser.state.index, skip[0].length);
	const nextCh = parser.input.charCodeAt(ob1Get0(next));

	// For ambiguous cases, determine if a LexicalDeclaration (or only a

	// Statement) is allowed here. If context is not empty then only a Statement

	// is allowed. However, `let [` is an explicit negative lookahead for

	// ExpressionStatement, so special-case it first.
	if (nextCh === charCodes.leftSquareBracket) {
		return true;
	}

	if (context !== undefined) {
		return false;
	}

	if (nextCh === charCodes.leftCurlyBrace) {
		return true;
	}

	if (isIdentifierStart(nextCh)) {
		let pos = ob1Add(next, 1);
		while (isIdentifierChar(parser.input.charCodeAt(ob1Get0(pos)))) {
			pos = ob1Inc(pos);
		}

		const ident = parser.getRawInput(next, pos);
		if (!keywordRelationalOperator.test(ident)) {
			return true;
		}
	}
	return false;
}

type StatementContext =
	 | undefined
	| "if"
	| "label"
	| "do"
	| "while"
	| "with"
	| "for";

// Parse a single statement.

//

// If expecting a statement and finding a slash operator, parse a
// regular expression literal. This is to handle cases like
// `if (foo) /blah/.exec(foo)`, where looking at the previous token
// does not help.
export function parseStatement(
	parser: JSParser,
	context: StatementContext = undefined,
	topLevel: boolean = false,
): AnyStatement {
	let startType = parser.state.tokenType;
	const start = parser.getPosition();

	if (startType === tt._const && parser.isSyntaxEnabled("ts")) {
		const ahead = parser.lookaheadState();
		if (ahead.tokenType === tt.name && ahead.tokenValue === "enum") {
			parser.expect(tt._const);
			parser.expectContextual("enum");
			return parseTSEnumDeclaration(parser, start, /* isConst */ true);
		}
	}

	let kind: undefined | VariableDeclarationKind;
	if (isLetStart(parser, context)) {
		startType = tt._var;
		kind = "let";
	}

	// Most types of statements are recognized by the keyword they

	// start with. Many are trivial to parse, some require a bit of

	// complexity.
	switch (startType) {
		case tt._break:
			return parseBreakContinueStatement(parser, start, true);

		case tt._continue:
			return parseBreakContinueStatement(parser, start, false);

		case tt._debugger:
			return parseDebuggerStatement(parser, start);

		case tt._do:
			return parseDoStatement(parser, start);

		case tt._for:
			return parseForStatement(parser, start);

		case tt._function: {
			if (parser.lookaheadState().tokenType === tt.dot) {
				// MetaProperty: eg. function.sent
				break;
			}

			if (context !== undefined) {
				if (parser.inScope("STRICT")) {
					parser.addDiagnostic({
						description: descriptions.JS_PARSER.ILLEGAL_FUNCTION_IN_STRICT,
					});
				} else if (context !== "if" && context !== "label") {
					parser.addDiagnostic({
						description: descriptions.JS_PARSER.ILLEGAL_FUNCTION_IN_NON_STRICT,
					});
				}
			}

			parser.expect(tt._function);

			const result = parseFunctionDeclaration(parser, start, false);

			if (context !== undefined && result.head.generator === true) {
				parser.addDiagnostic({
					description: descriptions.JS_PARSER.ILLEGAL_GENERATOR_DEFINITION,
					loc: result.loc,
				});
			}

			return result;
		}

		case tt._class: {
			if (context !== undefined) {
				parser.unexpectedToken();
			}
			return parseClassDeclaration(parser, start);
		}

		case tt._if:
			return parseIfStatement(parser, start);

		case tt._return:
			return parseReturnStatement(parser, start);

		case tt._switch:
			return parseSwitchStatement(parser, start);

		case tt._throw:
			return parseThrowStatement(parser, start);

		case tt._try:
			return parseTryStatement(parser, start);

		case tt._const:
		case tt._var: {
			kind =
				kind === undefined ? assertVarKind(String(parser.state.tokenValue)) : kind;
			if (context !== undefined && kind !== "var") {
				parser.addDiagnostic({
					description: descriptions.JS_PARSER.LEXICAL_DECLARATION_IN_SINGLE_STATEMENT_CONTEXT,
				});
			}
			return parseVarStatement(parser, start, kind);
		}

		case tt._while:
			return parseWhileStatement(parser, start);

		case tt._with:
			return parseWithStatement(parser, start);

		case tt.braceL:
			return parseBlock(parser);

		case tt.semi:
			return parseEmptyStatement(parser, start);

		case tt._export:
		case tt._import: {
			const nextToken = parser.lookaheadState();
			if (nextToken.tokenType === tt.parenL || nextToken.tokenType === tt.dot) {
				break;
			}

			parser.next();

			let result: ParseExportResult | ParseImportResult;
			if (startType === tt._import) {
				result = parseImport(parser, start);
			} else {
				result = parseExport(parser, start);
			}

			if (!topLevel) {
				parser.addDiagnostic({
					description: descriptions.JS_PARSER.IMPORT_EXPORT_MUST_TOP_LEVEL,
				});
			}

			assertModuleNodeAllowed(parser, result);

			return result;
		}

		case tt.name:
			if (isAsyncFunctionDeclarationStart(parser)) {
				if (context !== undefined) {
					parser.addDiagnostic({
						description: descriptions.JS_PARSER.ILLEGAL_ASYNC_DEFINITION,
					});
				}

				// async identifier
				parser.expect(tt.name);

				// function keyword
				parser.expect(tt._function);

				return parseFunctionDeclaration(parser, start, true);
			}
	}

	// If the statement does not start with a statement keyword or a

	// brace, it's an ExpressionStatement or LabeledStatement. We

	// simply start parsing an expression, and afterwards, if the

	// next token is a colon and the expression was a simple

	// Identifier node, we switch to interpreting it as a label.
	const maybeName = String(parser.state.tokenValue);
	const expr = parseExpression(parser, "statement expression");

	if (
		startType === tt.name &&
		expr.type === "ReferenceIdentifier" &&
		parser.eat(tt.colon)
	) {
		return parseLabeledStatement(parser, start, maybeName, expr, context);
	} else {
		return parseExpressionStatement(parser, start, expr);
	}
}

export function isAsyncFunctionDeclarationStart(parser: JSParser): boolean {
	if (!parser.isContextual("async")) {
		return false;
	}

	const {input} = parser;
	const {index} = parser.state;

	skipWhiteSpace.lastIndex = ob1Get0(index);
	const skip = skipWhiteSpace.exec(input);

	if (!skip || skip.length === 0) {
		return false;
	}

	const next = ob1Add(index, skip[0].length);

	return (
		!lineBreak.test(parser.getRawInput(index, next)) &&
		parser.getRawInput(next, ob1Add(next, 8)) === "function" &&
		(ob1Get0(next) + 8 === input.length ||
		!isIdentifierChar(input.charCodeAt(ob1Get0(next) + 8)))
	);
}

export function assertModuleNodeAllowed(parser: JSParser, node: AnyNode): void {
	if (
		(node.type === "ImportDeclaration" &&
		(node.importKind === "type" || node.importKind === "typeof")) ||
		(node.type === "ExportLocalDeclaration" && node.exportKind === "type") ||
		(node.type === "ExportAllDeclaration" && node.exportKind === "type")
	) {
		// Allow Flow type imports and exports in all conditions because
		// Flow itself does not care about 'sourceType'.
		return;
	}

	if (!parser.inModule) {
		parser.addDiagnostic({
			loc: node.loc,
			description: descriptions.JS_PARSER.IMPORT_EXPORT_IN_SCRIPT(
				parser.options.manifestPath,
			),
		});
	}
}

export function parseBreakContinueStatement(
	parser: JSParser,
	start: Position,
	isBreak: boolean,
): BreakStatement | ContinueStatement {
	parser.next();

	let label;
	if (parser.isLineTerminator()) {
		label = undefined;
	} else if (parser.match(tt.name)) {
		label = parseIdentifier(parser);
		parser.semicolon();
	} else {
		parser.unexpectedToken();
	}

	// Verify that there is an actual destination to break or

	// continue to.
	let i;
	for (i = 0; i < parser.state.labels.length; ++i) {
		const lab = parser.state.labels[i];
		if (label === undefined || lab.name === label.name) {
			if (lab.kind !== undefined && (isBreak || lab.kind === "loop")) {
				break;
			}

			if (label && isBreak) {
				break;
			}
		}
	}
	if (i === parser.state.labels.length) {
		parser.addDiagnostic({
			start,
			description: descriptions.JS_PARSER.UNKNOWN_LABEL(label && label.name),
		});
	}

	if (isBreak) {
		return parser.finishNode(
			start,
			{
				type: "BreakStatement",
				label,
			},
		);
	} else {
		return parser.finishNode(
			start,
			{
				type: "ContinueStatement",
				label,
			},
		);
	}
}

export function parseDebuggerStatement(
	parser: JSParser,
	start: Position,
): DebuggerStatement {
	parser.next();
	parser.semicolon();
	return parser.finishNode(start, {type: "DebuggerStatement"});
}

export function parseDoStatement(
	parser: JSParser,
	start: Position,
): DoWhileStatement {
	parser.next();
	parser.state.labels.push(loopLabel);
	const body = parseStatement(parser, "do");
	parser.state.labels.pop();
	parser.expect(tt._while);
	const test = parseParenExpression(parser, "do test");
	parser.eat(tt.semi);
	return parser.finishNode(
		start,
		{
			type: "DoWhileStatement",
			body,
			test,
		},
	);
}

export function parseForStatement(
	parser: JSParser,
	start: Position,
): AnyForStatement {
	parser.next();
	parser.state.labels.push(loopLabel);

	let awaitAt;
	if (parser.inScope("ASYNC") && parser.eatContextual("await")) {
		awaitAt = parser.getLastEndPosition();
	}

	const openContext = parser.expectOpening(tt.parenL, tt.parenR, "for head");

	if (parser.match(tt.semi)) {
		if (awaitAt) {
			parser.unexpectedToken();
		}
		return parseFor(parser, start, openContext, undefined);
	}

	const _isLet = isLetStart(parser);
	if (parser.match(tt._var) || parser.match(tt._const) || _isLet) {
		const initStart = parser.getPosition();

		const kind = assertVarKind(_isLet ? "let" : String(parser.state.tokenValue));
		parser.next();

		const declarations = parseVar(parser, initStart, kind, true);

		const init: VariableDeclaration = parser.finishNode(
			initStart,
			{
				type: "VariableDeclaration",
				kind,
				declarations,
			},
		);

		if (
			(parser.match(tt._in) || parser.isContextual("of")) &&
			init.declarations.length === 1
		) {
			return parseForIn(parser, start, openContext, init, awaitAt);
		}

		if (awaitAt !== undefined) {
			parser.addDiagnostic({
				start: awaitAt,
				description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
			});
		}

		return parseFor(parser, start, openContext, init);
	}

	const refShorthandDefaultPos: IndexTracker = {index: ob1Number0};
	let init = parseExpression(parser, "for init", true, refShorthandDefaultPos);

	if (parser.match(tt._in) || parser.isContextual("of")) {
		const description = parser.isContextual("of")
			? "for-of statement"
			: "for-in statement";
		const initPattern = toTargetAssignmentPattern(parser, init, description);
		checkLVal(parser, init, undefined, undefined, description);
		return parseForIn(parser, start, openContext, initPattern, awaitAt);
	}

	if (ob1Get0(refShorthandDefaultPos.index) > 0) {
		parser.unexpectedToken(
			parser.getPositionFromIndex(refShorthandDefaultPos.index),
		);
	}

	if (awaitAt !== undefined) {
		parser.addDiagnostic({
			start: awaitAt,
			description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
		});
	}

	return parseFor(parser, start, openContext, init);
}

export function assertVarKind(kind: string): VariableDeclarationKind {
	if (kind === "let" || kind === "var" || kind === "const") {
		return kind;
	} else {
		throw new Error(`Expected valid variable kind but got ${kind}`);
	}
}

export function parseIfStatement(parser: JSParser, start: Position): IfStatement {
	parser.next();
	const test = parseParenExpression(parser, "if test");
	const consequent = parseStatement(parser, "if");
	const alternate = parser.eat(tt._else)
		? parseStatement(parser, "if")
		: undefined;
	return parser.finishNode(
		start,
		{
			type: "IfStatement",
			test,
			consequent,
			alternate,
		},
	);
}

export function parseReturnStatement(
	parser: JSParser,
	start: Position,
): ReturnStatement {
	if (
		!parser.inScope("FUNCTION") &&
		parser.sourceType !== "template" &&
		!parser.options.allowReturnOutsideFunction
	) {
		parser.addDiagnostic({
			description: descriptions.JS_PARSER.RETURN_OUTSIDE_FUNCTION,
		});
	}

	parser.next();

	// In `return` (and `break`/`continue`), the keywords with

	// optional arguments, we eagerly look for a semicolon or the

	// possibility to insert one.
	let argument;
	if (!parser.isLineTerminator()) {
		argument = parseExpression(parser, "return argument");
		parser.semicolon();
	}

	return parser.finishNode(
		start,
		{
			type: "ReturnStatement",
			argument,
		},
	);
}

export function parseSwitchStatement(
	parser: JSParser,
	start: Position,
): SwitchStatement {
	parser.expect(tt._switch);
	const discriminant = parseParenExpression(parser, "switch discriminant");
	const cases: Array<SwitchCase> = [];
	const hasBrace = parser.match(tt.braceL);
	const openContext = parser.expectOpening(tt.braceL, tt.braceR, "switch body");
	parser.state.labels.push(switchLabel);

	if (hasBrace) {
		// Statements under must be grouped (by label) in SwitchCase
		// nodes. `cur` is used to keep the node that we are currently
		// adding statements to.
		let cur:
			 | undefined
			| {
					start: Position;
					test: undefined | AnyExpression;
					consequent: Array<AnyStatement>;
				};

		function pushCase() {
			if (cur === undefined) {
				return;
			}

			cases.push(
				parser.finishNode(
					cur.start,
					{
						type: "SwitchCase",
						test: cur.test,
						consequent: cur.consequent,
					},
				),
			);

			cur = undefined;
		}

		let sawDefault;

		while (true) {
			if (parser.match(tt.braceR) || parser.match(tt.eof)) {
				break;
			}

			if (parser.match(tt._case) || parser.match(tt._default)) {
				pushCase();

				const isCase = parser.match(tt._case);

				parser.next();

				let test;
				if (isCase) {
					test = parseExpression(parser, "case test");
				} else {
					if (sawDefault) {
						// TODO point to other default
						parser.addDiagnostic({
							start: parser.state.lastStartPos,
							description: descriptions.JS_PARSER.MULTIPLE_DEFAULT_CASE,
						});
					}
					sawDefault = true;
				}

				cur = {
					start: parser.getPosition(),
					consequent: [],
					test,
				};

				parser.expect(tt.colon);
			} else {
				const stmt = parseStatement(parser, undefined);
				if (cur === undefined) {
					parser.addDiagnostic({
						loc: stmt.loc,
						description: descriptions.JS_PARSER.SWITCH_STATEMENT_OUTSIDE_CASE,
					});
				} else {
					cur.consequent.push(stmt);
				}
			}
		}

		pushCase();
	}

	parser.expectClosing(openContext);
	parser.state.labels.pop();

	return parser.finishNode(
		start,
		{
			type: "SwitchStatement",
			discriminant,
			cases,
		},
	);
}

export function parseThrowStatement(
	parser: JSParser,
	start: Position,
): ThrowStatement {
	parser.next();
	if (
		lineBreak.test(
			parser.getRawInput(
				parser.state.lastEndPos.index,
				parser.state.startPos.index,
			),
		)
	) {
		parser.addDiagnostic({
			start: parser.state.lastEndPos,
			description: descriptions.JS_PARSER.NEWLINE_AFTER_THROW,
		});
	}

	const argument = parseExpression(parser, "throw argument");
	parser.semicolon();
	return parser.finishNode(
		start,
		{
			type: "ThrowStatement",
			argument,
		},
	);
}

export function parseTryStatement(
	parser: JSParser,
	start: Position,
): TryStatement {
	parser.next();

	const block = parseBlock(parser);
	let handler: undefined | CatchClause = undefined;

	if (parser.match(tt._catch)) {
		const clauseStart = parser.getPosition();
		parser.next();

		let param: undefined | AnyBindingPattern;
		if (parser.match(tt.parenL)) {
			const openContext = parser.expectOpening(
				tt.parenL,
				tt.parenR,
				"catch clause param",
			);
			param = parseTargetBindingPattern(parser);
			const clashes: Map<string, AnyNode> = new Map();
			checkLVal(parser, param, true, clashes, "catch clause");
			parser.expectClosing(openContext);
		}

		const body = parseBlock(parser);
		handler = parser.finishNode(
			clauseStart,
			{
				type: "CatchClause",
				body,
				param,
			},
		);
	}

	const finalizer = parser.eat(tt._finally) ? parseBlock(parser) : undefined;

	if (!handler && !finalizer) {
		parser.addDiagnostic({
			start,
			description: descriptions.JS_PARSER.TRY_MISSING_FINALLY_OR_CATCH,
		});
	}

	return parser.finishNode(
		start,
		{
			type: "TryStatement",
			block,
			finalizer,
			handler,
		},
	);
}

export function parseVarStatement(
	parser: JSParser,
	start: Position,
	kind: VariableDeclarationKind,
): VariableDeclarationStatement {
	parser.next();
	const declarations = parseVar(parser, start, kind, false);
	parser.semicolon();
	return parser.finishNode(
		start,
		{
			type: "VariableDeclarationStatement",
			declaration: parser.finishNode(
				start,
				{
					type: "VariableDeclaration",
					kind,
					declarations,
				},
			),
		},
	);
}

export function parseWhileStatement(
	parser: JSParser,
	start: Position,
): WhileStatement {
	parser.next();
	const test = parseParenExpression(parser, "while test");
	parser.state.labels.push(loopLabel);
	const body = parseStatement(parser, "while");
	parser.state.labels.pop();
	return parser.finishNode(start, {type: "WhileStatement", test, body});
}

export function parseWithStatement(
	parser: JSParser,
	start: Position,
): WithStatement {
	parser.next();
	const object = parseParenExpression(parser, "with object");
	const body = parseStatement(parser, "with");

	if (parser.inScope("STRICT")) {
		parser.addDiagnostic({
			loc: parser.finishLoc(start),
			description: descriptions.JS_PARSER.WITH_IN_STRICT,
		});
	}

	return parser.finishNode(
		start,
		{
			type: "WithStatement",
			object,
			body,
		},
	);
}

export function parseEmptyStatement(
	parser: JSParser,
	start: Position,
): EmptyStatement {
	parser.next();
	return parser.finishNode(start, {type: "EmptyStatement"});
}

export function parseLabeledStatement(
	parser: JSParser,
	start: Position,
	maybeName: string,
	expr: ReferenceIdentifier,
	context: StatementContext,
): LabeledStatement {
	for (const label of parser.state.labels) {
		if (label.name === maybeName) {
			parser.addDiagnostic({
				loc: expr.loc,
				description: descriptions.JS_PARSER.DUPLICATE_LABEL(maybeName, label.loc),
			});
		}
	}

	let kind: LabelKind = undefined;
	if (parser.state.tokenType.isLoop) {
		kind = "loop";
	} else if (parser.match(tt._switch)) {
		kind = "switch";
	}

	for (let i = parser.state.labels.length - 1; i >= 0; i--) {
		const label = parser.state.labels[i];
		if (label.statementStart === start.index) {
			label.statementStart = parser.state.startPos.index;
			label.kind = kind;
		} else {
			break;
		}
	}

	parser.state.labels.push({
		name: maybeName,
		kind,
		loc: parser.getLoc(expr),
		statementStart: parser.state.startPos.index,
	});

	let statementContext: StatementContext = "label";
	if (context !== undefined) {
		if (context.includes("label")) {
			statementContext = context;
		} else {
			// @ts-ignore
			statementContext = `${context}label`;
		}
	}
	const body = parseStatement(parser, statementContext);

	if (
		body.type === "ClassDeclaration" ||
		(body.type === "VariableDeclarationStatement" &&
		body.declaration.kind !== "var") ||
		(body.type === "FunctionDeclaration" &&
		(parser.inScope("STRICT") ||
		body.head.generator === true ||
		body.head.async === true))
	) {
		parser.addDiagnostic({
			loc: body.loc,
			description: descriptions.JS_PARSER.INVALID_LABEL_DECLARATION,
		});
	}

	parser.state.labels.pop();
	return parser.finishNode(
		start,
		{
			type: "LabeledStatement",
			label: {
				...expr,
				type: "Identifier",
			},
			body,
		},
	);
}

export function parseExpressionStatement(
	parser: JSParser,
	start: Position,
	expr: AnyExpression,
): AnyStatement {
	const node = parseTSTypeExpressionStatement(parser, start, expr);
	if (node !== undefined) {
		return node;
	}

	parser.semicolon();
	return parser.finishNode(
		start,
		{
			type: "ExpressionStatement",
			expression: expr,
		},
	);
}

export function parseBlock(
	parser: JSParser,
	allowDirectives?: boolean,
): BlockStatement {
	const start = parser.getPosition();
	const openContext = parser.expectOpening(tt.braceL, tt.braceR, "block");
	const {body, directives} = parseBlockBody(
		parser,
		allowDirectives,
		false,
		openContext,
	);
	return parser.finishNode(
		start,
		{
			type: "BlockStatement",
			directives,
			body,
		},
	);
}

export function isValidDirective(parser: JSParser, stmt: AnyStatement): boolean {
	return (
		stmt.type === "ExpressionStatement" &&
		stmt.expression.type === "StringLiteral" &&
		!parser.isParenthesized(stmt.expression)
	);
}

export function parseBlockBody(
	parser: JSParser,
	allowDirectives: boolean = false,
	topLevel: boolean,
	openContext: OpeningContext,
): {
	body: Array<AnyStatement>;
	directives: Array<Directive>;
} {
	return parseBlockOrModuleBlockBody(
		parser,
		allowDirectives,
		topLevel,
		openContext,
	);
}

export function parseBlockOrModuleBlockBody(
	parser: JSParser,
	allowDirectives: boolean,
	topLevel: boolean,
	openContext: OpeningContext,
): {
	body: Array<AnyStatement>;
	directives: Array<Directive>;
} {
	const body: Array<AnyStatement> = [];
	const directives: Array<Directive> = [];

	let parsedNonDirective = false;
	let didSetStrict = undefined;
	let octalPosition;

	while (true) {
		if (parser.match(openContext.close) || parser.match(tt.eof)) {
			parser.expectClosing(openContext);
			break;
		}

		if (!parsedNonDirective && parser.state.containsOctal && !octalPosition) {
			octalPosition = parser.state.octalPosition;
		}

		const stmt = parseStatement(parser, undefined, topLevel);

		if (
			allowDirectives &&
			!parsedNonDirective &&
			stmt.type === "ExpressionStatement" &&
			isValidDirective(parser, stmt)
		) {
			const directive = expressionStatementToDirective(parser, stmt);
			directives.push(directive);

			if (didSetStrict === undefined && directive.value === "use strict") {
				setStrict(parser, true);
				didSetStrict = true;

				if (octalPosition !== undefined) {
					parser.addDiagnostic({
						index: octalPosition,
						description: descriptions.JS_PARSER.OCTAL_IN_STRICT,
					});
				}
			}

			continue;
		}

		parsedNonDirective = true;
		body.push(stmt);
	}

	if (didSetStrict) {
		parser.popScope("STRICT");
	}

	return {body, directives};
}

export function parseFor(
	parser: JSParser,
	start: Position,
	openContext: OpeningContext,
	init: undefined | (VariableDeclaration | AnyExpression),
): ForStatement {
	parser.expect(tt.semi);

	const test = parser.match(tt.semi)
		? undefined
		: parseExpression(parser, "for test");
	parser.expect(tt.semi);

	const update = parser.match(tt.parenR)
		? undefined
		: parseExpression(parser, "for update");
	parser.expectClosing(openContext);

	const body = parseStatement(parser, "for");
	parser.state.labels.pop();

	return parser.finishNode(
		start,
		{
			type: "ForStatement",
			init,
			test,
			update,
			body,
		},
	);
}

export function parseForIn(
	parser: JSParser,
	start: Position,
	openContext: OpeningContext,
	init: VariableDeclaration | AnyTargetAssignmentPattern,
	awaitAt: undefined | Position,
): AnyForInOfStatement {
	const isForIn: boolean = parser.match(tt._in);
	parser.next();

	const isAwait = awaitAt !== undefined;
	if (isForIn && isAwait) {
		parser.addDiagnostic({
			start: awaitAt,
			description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
		});
	}

	if (
		init.type === "VariableDeclaration" &&
		init.declarations[0].init !== undefined &&
		(!isForIn ||
		parser.inScope("STRICT") ||
		init.kind !== "var" ||
		init.declarations[0].id.type !== "BindingIdentifier")
	) {
		parser.addDiagnostic({
			loc: init.loc,
			description: descriptions.JS_PARSER.FOR_IN_OF_WITH_INITIALIZER,
		});
	}

	const left = init;
	const right = isForIn
		? parseExpression(parser, "for right")
		: parseMaybeAssign(parser, "for right");
	parser.expectClosing(openContext);

	const body = parseStatement(parser, "for");
	parser.state.labels.pop();

	if (isForIn) {
		const node: ForInStatement = parser.finishNode(
			start,
			{
				type: "ForInStatement",
				left,
				right,
				body,
			},
		);
		return node;
	} else {
		const node: ForOfStatement = parser.finishNode(
			start,
			{
				type: "ForOfStatement",
				await: isAwait,
				left,
				right,
				body,
			},
		);
		return node;
	}
}

export function parseVar(
	parser: JSParser,
	start: Position,
	kind: string,
	isFor: boolean,
): Array<VariableDeclarator> {
	const declarations: Array<VariableDeclarator> = [];

	while (true) {
		const start = parser.getPosition();
		const id = parseVarHead(parser, start);

		if (kind === "var") {
			parser.state.hasHoistedVars = true;
		}

		let init;
		if (parser.eat(tt.eq)) {
			init = parseMaybeAssign(parser, "var init", isFor);
		} else {
			if (kind === "const" && !(parser.match(tt._in) || parser.isContextual("of"))) {
				// `const` with no initializer is allowed in TypeScript.
				// It could be a declaration like `const x: number;`.
				if (!parser.isSyntaxEnabled("ts")) {
					parser.addDiagnostic({
						description: descriptions.JS_PARSER.CONST_WITHOUT_INITIALIZER,
						loc: id.loc,
					});
				}
			}

			// We exclude `const` because we already validated it above
			if (
				kind !== "const" &&
				id.type !== "BindingIdentifier" &&
				!(isFor && (parser.match(tt._in) || parser.isContextual("of")))
			) {
				parser.addDiagnostic({
					start: parser.state.lastEndPos,
					description: descriptions.JS_PARSER.COMPLEX_BINDING_WITHOUT_INITIALIZER,
				});
			}
		}

		declarations.push(
			parser.finishNode(
				start,
				{
					type: "VariableDeclarator",
					id,
					init,
				},
			),
		);

		if (!parser.eat(tt.comma)) {
			break;
		}
	}

	return declarations;
}

export function parseVarHead(
	parser: JSParser,
	start: Position,
): AnyTargetBindingPattern {
	const id = parseTargetBindingPattern(parser);

	checkLVal(parser, id, true, undefined, "variable declaration");

	let definite: undefined | boolean;
	if (id.type === "BindingIdentifier" && parser.match(tt.bang)) {
		definite = true;
		parser.expectSyntaxEnabled("ts");
		parser.next();
	}

	if (parser.match(tt.colon)) {
		const typeAnnotation = parseTSTypeAnnotation(parser, true);

		return parser.finishNode(
			start,
			{
				...id,
				meta: parser.finishNode(
					start,
					{
						type: "PatternMeta",
						typeAnnotation,
						definite,
					},
				),
			},
		);
	} else if (definite) {
		return {
			...id,
			meta: parser.finishNode(start, {type: "PatternMeta", definite}),
		};
	} else {
		return id;
	}
}

function parseFunctionId(
	parser: JSParser,
	requiredStatementId: boolean,
): undefined | BindingIdentifier {
	if (requiredStatementId || parser.match(tt.name)) {
		return parseBindingIdentifier(parser);
	} else {
		return undefined;
	}
}

export function parseFunctionDeclaration(
	parser: JSParser,
	start: Position,
	isAsync: boolean,
): FunctionDeclaration | TSDeclareFunction {
	const {id, body, ...shape} = parseFunction(
		parser,
		{
			start,
			requiredStatementId: true,
			isStatement: true,
			isAsync,
		},
	);

	if (id === undefined) {
		throw new Error("Required function name");
	}

	if (body === undefined) {
		return parser.finalizeNode({
			type: "TSDeclareFunction",
			...shape,
			id,
		});
	}

	return parser.finalizeNode({
		type: "FunctionDeclaration",
		...shape,
		id,
		body,
	});
}

export function parseExportDefaultFunctionDeclaration(
	parser: JSParser,
	start: Position,
	isAsync: boolean,
): FunctionDeclaration | TSDeclareFunction {
	let {id, body, ...shape} = parseFunction(
		parser,
		{
			start,
			requiredStatementId: false,
			isStatement: true,
			isAsync,
		},
	);

	if (id === undefined) {
		id = {
			type: "BindingIdentifier",
			name: "*default*",
			// Does this `loc` make sense?
			loc: shape.loc,
		};
	}

	if (body === undefined) {
		return parser.finalizeNode({
			type: "TSDeclareFunction",
			...shape,
			id,
		});
	}

	return parser.finalizeNode({
		type: "FunctionDeclaration",
		...shape,
		id,
		body,
	});
}

export function parseFunctionExpression(
	parser: JSParser,
	start: Position,
	isAsync: boolean,
): FunctionExpression {
	const {body, ...shape} = parseFunction(
		parser,
		{
			start,
			requiredStatementId: false,
			isStatement: false,
			isAsync,
		},
	);

	if (body === undefined) {
		throw new Error("Expected body");
	}

	return {
		...shape,
		body,
		type: "FunctionExpression",
	};
}

export function parseFunction(
	parser: JSParser,
	opts: {
		start: Position;
		isStatement: boolean;
		requiredStatementId: boolean;
		isAsync: boolean;
	},
): {
	id: undefined | BindingIdentifier;
	head: FunctionHead;
	body: undefined | BlockStatement;
	loc: SourceLocation;
} {
	const {start, isStatement, requiredStatementId, isAsync} = opts;

	const isGenerator = parser.eat(tt.star);

	let id;
	if (isStatement) {
		id = parseFunctionId(parser, requiredStatementId);
	}

	const oldYieldPos = parser.state.yieldPos;
	const oldAwaitPos = parser.state.awaitPos;
	parser.pushScope("FUNCTION_LOC", start);
	parser.pushScope("FUNCTION", true);
	parser.pushScope("METHOD", false);
	parser.pushScope("GENERATOR", isGenerator);
	parser.pushScope("ASYNC", isAsync);
	parser.pushScope("CLASS_PROPERTY", false);
	parser.pushScope("NON_ARROW_FUNCTION");
	parser.state.yieldPos = ob1Number0;
	parser.state.awaitPos = ob1Number0;

	if (!isStatement) {
		id = parseFunctionId(parser, false);
	}

	const headStart = parser.getPosition();
	const {params, rest, typeParameters} = parseFunctionParams(parser);
	const {head, body} = parseFunctionBodyAndFinish(
		parser,
		{
			allowBodiless: isStatement,
			id,
			params,
			rest,
			isArrowFunction: false,
			isMethod: false,
			isAsync,
			isGenerator,
			headStart,
			start,
		},
	);

	parser.state.yieldPos = oldYieldPos;
	parser.state.awaitPos = oldAwaitPos;

	parser.popScope("NON_ARROW_FUNCTION");
	parser.popScope("FUNCTION");
	parser.popScope("FUNCTION_LOC");
	parser.popScope("CLASS_PROPERTY");
	parser.popScope("METHOD");
	parser.popScope("GENERATOR");
	parser.popScope("ASYNC");

	if (body !== undefined && body.type !== "BlockStatement") {
		throw new Error("Expected block statement for functions");
	}

	return {
		head: {
			...head,
			typeParameters,
		},
		body,
		id,
		loc: parser.finishLoc(start),
	};
}

export function splitFunctionParams(
	params: FunctionHead["params"],
): {
	params: FunctionHead["params"];
	thisType: undefined | BindingIdentifier;
} {
	const firstParam = params[0];
	if (
		firstParam !== undefined &&
		firstParam.type === "BindingIdentifier" &&
		firstParam.name === "this"
	) {
		return {
			thisType: firstParam,
			params: params.slice(1),
		};
	} else {
		return {
			thisType: undefined,
			params,
		};
	}
}

export function parseFunctionParams(
	parser: JSParser,
	kind?: string,
	allowTSModifiers?: boolean,
): {
	typeParameters: undefined | TSTypeParameterDeclaration;
	params: Array<AnyBindingPattern>;
	rest: undefined | AnyTargetBindingPattern;
} {
	let typeParameters = undefined;
	if (parser.isRelational("<")) {
		typeParameters = maybeParseTSTypeParameters(parser);

		if (typeParameters !== undefined && (kind === "get" || kind === "set")) {
			parser.addDiagnostic({
				loc: typeParameters.loc,
				description: descriptions.JS_PARSER.ACCESSOR_WITH_TYPE_PARAMS,
			});
		}
	}

	parser.pushScope("PARAMETERS", true);

	const openContext = parser.expectOpening(
		tt.parenL,
		tt.parenR,
		"function params",
	);
	const {list: params, rest} = parseBindingListNonEmpty(
		parser,
		openContext,
		allowTSModifiers,
	);

	parser.popScope("PARAMETERS");
	checkYieldAwaitInDefaultParams(parser);
	return {params, rest, typeParameters};
}
