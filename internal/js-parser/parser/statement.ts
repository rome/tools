/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Label, LabelKind} from "../tokenizer/state";
import {types as tt} from "../tokenizer/types";
import {Position, SourceLocation} from "@internal/parser-core";
import {
	IndexTracker,
	isIdentifierChar,
	isIdentifierStart,
	keywordRelationalOperator,
	lineBreak,
	skipWhiteSpace,
} from "@internal/js-parser-utils";
import {
	JSParser,
	OpeningContext,
	eat,
	eatContextual,
	expect,
	expectClosing,
	expectContextual,
	expectOpening,
	expectSyntaxEnabled,
	inScope,
	isContextual,
	isLineTerminator,
	isParenthesized,
	isRelational,
	isSyntaxEnabled,
	lookaheadState,
	match,
	next,
	popScope,
	pushScope,
	semicolon,
	unexpectedDiagnostic,
	unexpectedToken,
} from "../parser";
import {
	AnyJSBindingPattern,
	AnyJSExpression,
	AnyJSForInOfStatement,
	AnyJSForStatement,
	AnyJSStatement,
	AnyJSTargetAssignmentPattern,
	AnyJSTargetBindingPattern,
	AnyNode,
	JSBindingIdentifier,
	JSBlockStatement,
	JSBreakStatement,
	JSCatchClause,
	JSContinueStatement,
	JSDebuggerStatement,
	JSDirective,
	JSDoWhileStatement,
	JSEmptyStatement,
	JSExpressionStatement,
	JSForInStatement,
	JSForOfStatement,
	JSForStatement,
	JSFunctionDeclaration,
	JSFunctionExpression,
	JSFunctionHead,
	JSIfStatement,
	JSInterpreterDirective,
	JSLabeledStatement,
	JSReferenceIdentifier,
	JSReturnStatement,
	JSRoot,
	JSSwitchCase,
	JSSwitchStatement,
	JSThrowStatement,
	JSTryStatement,
	JSVariableDeclaration,
	JSVariableDeclarationKind,
	JSVariableDeclarationStatement,
	JSVariableDeclarator,
	JSWhileStatement,
	JSWithStatement,
	TSDeclareFunction,
	TSTypeParameterDeclaration,
} from "@internal/ast";
import * as charCodes from "@internal/string-charcodes";
import {
	nextToken,
	setStrict,
	skipInterpreterDirective,
} from "../tokenizer/index";
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
import {ob1Add, ob1Get0, ob1Inc, ob1Number0} from "@internal/ob1";
import {descriptions} from "@internal/diagnostics";

const loopLabel: Label = {kind: "loop"};
const switchLabel: Label = {kind: "switch"};

export function parseTopLevel(parser: JSParser): JSRoot {
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
		parser.finishRoot({
			type: "JSRoot",
			corrupt: parser.state.corrupt,
			body,
			directives,
			sourceType: parser.options.sourceType,
			interpreter,
			syntax: Array.from(parser.meta.syntax),
			hasHoistedVars: parser.state.hasHoistedVars,
		}),
	);
}

export function parsePossibleInterpreterDirective(
	parser: JSParser,
): undefined | JSInterpreterDirective {
	// Check for #!
	if (
		match(parser, tt.hash) &&
		parser.input[ob1Get0(parser.state.endIndex)] === "!"
	) {
		const directive = skipInterpreterDirective(parser, 1);

		// Advance to next token
		next(parser);

		return directive;
	} else {
		return undefined;
	}
}

export function expressionStatementToDirective(
	parser: JSParser,
	stmt: JSExpressionStatement,
): JSDirective {
	const expr = stmt.expression;

	const start = parser.getLoc(stmt).start;

	const raw = parser.getRawInput(
		parser.getInputStartIndex(expr),
		parser.getInputEndIndex(expr),
	);
	const val = raw.slice(1, -1); // remove quotes
	const end = parser.getLoc(stmt).end;

	return parser.finishNodeAt(
		start,
		end,
		{
			type: "JSDirective",
			value: val,
		},
	);
}

export function isLetStart(parser: JSParser, context?: string): boolean {
	if (!isContextual(parser, "let")) {
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
	// JSExpressionStatement, so special-case it first.
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
): AnyJSStatement {
	let startType = parser.state.tokenType;
	const start = parser.getPosition();

	if (startType === tt._const && isSyntaxEnabled(parser, "ts")) {
		const ahead = lookaheadState(parser);
		if (ahead.tokenType === tt.name && ahead.tokenValue === "enum") {
			expect(parser, tt._const);
			expectContextual(parser, "enum");
			return parseTSEnumDeclaration(parser, start, /* isConst */ true);
		}
	}

	let kind: undefined | JSVariableDeclarationKind;
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
			if (lookaheadState(parser).tokenType === tt.dot) {
				// JSMetaProperty: eg. function.sent
				break;
			}

			if (context !== undefined) {
				if (inScope(parser, "STRICT")) {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.ILLEGAL_FUNCTION_IN_STRICT,
						},
					);
				} else if (context !== "if" && context !== "label") {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.ILLEGAL_FUNCTION_IN_NON_STRICT,
						},
					);
				}
			}

			expect(parser, tt._function);

			const result = parseFunctionDeclaration(parser, start, false);

			if (context !== undefined && result.head.generator === true) {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.ILLEGAL_GENERATOR_DEFINITION,
						loc: result.loc,
					},
				);
			}

			return result;
		}

		case tt._class: {
			if (context !== undefined) {
				unexpectedToken(parser);
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
				kind === undefined
					? assertVarKind(String(parser.state.tokenValue))
					: kind;
			if (context !== undefined && kind !== "var") {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.LEXICAL_DECLARATION_IN_SINGLE_STATEMENT_CONTEXT,
					},
				);
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
			const nextToken = lookaheadState(parser);
			if (nextToken.tokenType === tt.parenL || nextToken.tokenType === tt.dot) {
				break;
			}

			next(parser);

			let result: ParseExportResult | ParseImportResult;
			if (startType === tt._import) {
				result = parseImport(parser, start);
			} else {
				result = parseExport(parser, start);
			}

			if (!topLevel) {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.IMPORT_EXPORT_MUST_TOP_LEVEL,
					},
				);
			}

			assertModuleNodeAllowed(parser, result);

			return result;
		}

		case tt.name:
			if (isAsyncFunctionDeclarationStart(parser)) {
				if (context !== undefined) {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.ILLEGAL_ASYNC_DEFINITION,
						},
					);
				}

				// async identifier
				expect(parser, tt.name);

				// function keyword
				expect(parser, tt._function);

				return parseFunctionDeclaration(parser, start, true);
			}
	}

	// If the statement does not start with a statement keyword or a

	// brace, it's an JSExpressionStatement or JSLabeledStatement. We

	// simply start parsing an expression, and afterwards, if the

	// next token is a colon and the expression was a simple

	// JSIdentifier node, we switch to interpreting it as a label.
	const maybeName = String(parser.state.tokenValue);
	const expr = parseExpression(parser, "statement expression");

	if (
		startType === tt.name &&
		expr.type === "JSReferenceIdentifier" &&
		eat(parser, tt.colon)
	) {
		return parseLabeledStatement(parser, start, maybeName, expr, context);
	} else {
		return parseExpressionStatement(parser, start, expr);
	}
}

export function isAsyncFunctionDeclarationStart(parser: JSParser): boolean {
	if (!isContextual(parser, "async")) {
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
		(node.type === "JSImportDeclaration" &&
		(node.importKind === "type" || node.importKind === "typeof")) ||
		(node.type === "JSExportLocalDeclaration" && node.exportKind === "type") ||
		(node.type === "JSExportAllDeclaration" && node.exportKind === "type")
	) {
		// Allow Flow type imports and exports in all conditions because
		// Flow itself does not care about 'sourceType'.
		return;
	}

	if (!parser.meta.inModule) {
		unexpectedDiagnostic(
			parser,
			{
				loc: node.loc,
				description: descriptions.JS_PARSER.IMPORT_EXPORT_IN_SCRIPT(
					parser.options.manifestPath,
				),
			},
		);
	}
}

export function parseBreakContinueStatement(
	parser: JSParser,
	start: Position,
	isBreak: boolean,
): JSBreakStatement | JSContinueStatement {
	next(parser);

	let label;
	if (isLineTerminator(parser)) {
		label = undefined;
	} else if (match(parser, tt.name)) {
		label = parseIdentifier(parser);
		semicolon(parser);
	} else {
		unexpectedToken(parser);
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
		unexpectedDiagnostic(
			parser,
			{
				start,
				description: descriptions.JS_PARSER.UNKNOWN_LABEL(label && label.name),
			},
		);
	}

	if (isBreak) {
		return parser.finishNode(
			start,
			{
				type: "JSBreakStatement",
				label,
			},
		);
	} else {
		return parser.finishNode(
			start,
			{
				type: "JSContinueStatement",
				label,
			},
		);
	}
}

export function parseDebuggerStatement(
	parser: JSParser,
	start: Position,
): JSDebuggerStatement {
	next(parser);
	semicolon(parser);
	return parser.finishNode(start, {type: "JSDebuggerStatement"});
}

export function parseDoStatement(
	parser: JSParser,
	start: Position,
): JSDoWhileStatement {
	next(parser);
	parser.state.labels.push(loopLabel);
	const body = parseStatement(parser, "do");
	parser.state.labels.pop();
	expect(parser, tt._while);
	const test = parseParenExpression(parser, "do test");
	eat(parser, tt.semi);
	return parser.finishNode(
		start,
		{
			type: "JSDoWhileStatement",
			body,
			test,
		},
	);
}

export function parseForStatement(
	parser: JSParser,
	start: Position,
): AnyJSForStatement {
	next(parser);
	parser.state.labels.push(loopLabel);

	let awaitAt;
	if (inScope(parser, "ASYNC") && eatContextual(parser, "await")) {
		awaitAt = parser.getLastEndPosition();
	}

	const openContext = expectOpening(parser, tt.parenL, tt.parenR, "for head");

	if (match(parser, tt.semi)) {
		if (awaitAt) {
			unexpectedToken(parser);
		}
		return parseFor(parser, start, openContext, undefined);
	}

	const _isLet = isLetStart(parser);
	if (match(parser, tt._var) || match(parser, tt._const) || _isLet) {
		const initStart = parser.getPosition();

		const kind = assertVarKind(_isLet ? "let" : String(parser.state.tokenValue));
		next(parser);

		const declarations = parseVar(parser, initStart, kind, true);

		const init: JSVariableDeclaration = parser.finishNode(
			initStart,
			{
				type: "JSVariableDeclaration",
				kind,
				declarations,
			},
		);

		if (
			(match(parser, tt._in) || isContextual(parser, "of")) &&
			init.declarations.length === 1
		) {
			return parseForIn(parser, start, openContext, init, awaitAt);
		}

		if (awaitAt !== undefined) {
			unexpectedDiagnostic(
				parser,
				{
					start: awaitAt,
					description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
				},
			);
		}

		return parseFor(parser, start, openContext, init);
	}

	const refShorthandDefaultPos: IndexTracker = {index: ob1Number0};
	let init = parseExpression(parser, "for init", true, refShorthandDefaultPos);

	if (match(parser, tt._in) || isContextual(parser, "of")) {
		const description = isContextual(parser, "of")
			? "for-of statement"
			: "for-in statement";
		const initPattern = toTargetAssignmentPattern(parser, init, description);
		checkLVal(parser, init, undefined, undefined, description);
		return parseForIn(parser, start, openContext, initPattern, awaitAt);
	}

	if (ob1Get0(refShorthandDefaultPos.index) > 0) {
		unexpectedToken(
			parser,
			parser.getPositionFromIndex(refShorthandDefaultPos.index),
		);
	}

	if (awaitAt !== undefined) {
		unexpectedDiagnostic(
			parser,
			{
				start: awaitAt,
				description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
			},
		);
	}

	return parseFor(parser, start, openContext, init);
}

export function assertVarKind(kind: string): JSVariableDeclarationKind {
	if (kind === "let" || kind === "var" || kind === "const") {
		return kind;
	} else {
		throw new Error(`Expected valid variable kind but got ${kind}`);
	}
}

export function parseIfStatement(
	parser: JSParser,
	start: Position,
): JSIfStatement {
	next(parser);
	const test = parseParenExpression(parser, "if test");
	const consequent = parseStatement(parser, "if");
	const alternate = eat(parser, tt._else)
		? parseStatement(parser, "if")
		: undefined;
	return parser.finishNode(
		start,
		{
			type: "JSIfStatement",
			test,
			consequent,
			alternate,
		},
	);
}

export function parseReturnStatement(
	parser: JSParser,
	start: Position,
): JSReturnStatement {
	if (
		!inScope(parser, "FUNCTION") &&
		parser.options.sourceType !== "template" &&
		!parser.options.allowReturnOutsideFunction
	) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.RETURN_OUTSIDE_FUNCTION,
			},
		);
	}

	next(parser);

	// In `return` (and `break`/`continue`), the keywords with
	// optional arguments, we eagerly look for a semicolon or the
	// possibility to insert one.
	let argument;
	if (!isLineTerminator(parser)) {
		argument = parseExpression(parser, "return argument");
		semicolon(parser);
	}

	return parser.finishNode(
		start,
		{
			type: "JSReturnStatement",
			argument,
		},
	);
}

export function parseSwitchStatement(
	parser: JSParser,
	start: Position,
): JSSwitchStatement {
	expect(parser, tt._switch);
	const discriminant = parseParenExpression(parser, "switch discriminant");
	const cases: Array<JSSwitchCase> = [];
	const hasBrace = match(parser, tt.braceL);
	const openContext = expectOpening(parser, tt.braceL, tt.braceR, "switch body");
	parser.state.labels.push(switchLabel);

	if (hasBrace) {
		// Statements under must be grouped (by label) in JSSwitchCase
		// nodes. `cur` is used to keep the node that we are currently
		// adding statements to.
		let cur:
			| undefined
			| {
					start: Position;
					test: undefined | AnyJSExpression;
					consequent: Array<AnyJSStatement>;
				};

		function pushCase() {
			if (cur === undefined) {
				return;
			}

			cases.push(
				parser.finishNode(
					cur.start,
					{
						type: "JSSwitchCase",
						test: cur.test,
						consequent: cur.consequent,
					},
				),
			);

			cur = undefined;
		}

		let sawDefault;

		while (true) {
			if (match(parser, tt.braceR) || match(parser, tt.eof)) {
				break;
			}

			if (match(parser, tt._case) || match(parser, tt._default)) {
				pushCase();

				const start = parser.getPosition();
				const isCase = match(parser, tt._case);

				next(parser);

				let test;
				if (isCase) {
					test = parseExpression(parser, "case test");
				} else {
					if (sawDefault) {
						// TODO point to other default
						unexpectedDiagnostic(
							parser,
							{
								start: parser.state.lastStartPos,
								description: descriptions.JS_PARSER.MULTIPLE_DEFAULT_CASE,
							},
						);
					}
					sawDefault = true;
				}

				cur = {
					start,
					consequent: [],
					test,
				};

				expect(parser, tt.colon);
			} else {
				const stmt = parseStatement(parser, undefined);
				if (cur === undefined) {
					unexpectedDiagnostic(
						parser,
						{
							loc: stmt.loc,
							description: descriptions.JS_PARSER.SWITCH_STATEMENT_OUTSIDE_CASE,
						},
					);
				} else {
					cur.consequent.push(stmt);
				}
			}
		}

		pushCase();
	}

	expectClosing(parser, openContext);
	parser.state.labels.pop();

	return parser.finishNode(
		start,
		{
			type: "JSSwitchStatement",
			discriminant,
			cases,
		},
	);
}

export function parseThrowStatement(
	parser: JSParser,
	start: Position,
): JSThrowStatement {
	next(parser);
	if (
		lineBreak.test(
			parser.getRawInput(parser.state.lastEndIndex, parser.state.startIndex),
		)
	) {
		unexpectedDiagnostic(
			parser,
			{
				start: parser.state.lastEndPos,
				description: descriptions.JS_PARSER.NEWLINE_AFTER_THROW,
			},
		);
	}

	const argument = parseExpression(parser, "throw argument");
	semicolon(parser);
	return parser.finishNode(
		start,
		{
			type: "JSThrowStatement",
			argument,
		},
	);
}

export function parseTryStatement(
	parser: JSParser,
	start: Position,
): JSTryStatement {
	next(parser);

	const block = parseBlock(parser);
	let handler: undefined | JSCatchClause = undefined;

	if (match(parser, tt._catch)) {
		const clauseStart = parser.getPosition();
		next(parser);

		let param: undefined | AnyJSBindingPattern;
		if (match(parser, tt.parenL)) {
			const openContext = expectOpening(
				parser,
				tt.parenL,
				tt.parenR,
				"catch clause param",
			);
			param = parseTargetBindingPattern(parser);
			const clashes: Map<string, AnyNode> = new Map();
			checkLVal(parser, param, true, clashes, "catch clause");
			expectClosing(parser, openContext);
		}

		const body = parseBlock(parser);
		handler = parser.finishNode(
			clauseStart,
			{
				type: "JSCatchClause",
				body,
				param,
			},
		);
	}

	const finalizer = eat(parser, tt._finally) ? parseBlock(parser) : undefined;

	if (!handler && !finalizer) {
		unexpectedDiagnostic(
			parser,
			{
				start,
				description: descriptions.JS_PARSER.TRY_MISSING_FINALLY_OR_CATCH,
			},
		);
	}

	return parser.finishNode(
		start,
		{
			type: "JSTryStatement",
			block,
			finalizer,
			handler,
		},
	);
}

export function parseVarStatement(
	parser: JSParser,
	start: Position,
	kind: JSVariableDeclarationKind,
): JSVariableDeclarationStatement {
	next(parser);
	const declarations = parseVar(parser, start, kind, false);
	semicolon(parser);
	return parser.finishNode(
		start,
		{
			type: "JSVariableDeclarationStatement",
			declaration: parser.finishNode(
				start,
				{
					type: "JSVariableDeclaration",
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
): JSWhileStatement {
	next(parser);
	const test = parseParenExpression(parser, "while test");
	parser.state.labels.push(loopLabel);
	const body = parseStatement(parser, "while");
	parser.state.labels.pop();
	return parser.finishNode(start, {type: "JSWhileStatement", test, body});
}

export function parseWithStatement(
	parser: JSParser,
	start: Position,
): JSWithStatement {
	next(parser);
	const object = parseParenExpression(parser, "with object");
	const body = parseStatement(parser, "with");

	if (inScope(parser, "STRICT")) {
		unexpectedDiagnostic(
			parser,
			{
				loc: parser.finishLoc(start),
				description: descriptions.JS_PARSER.WITH_IN_STRICT,
			},
		);
	}

	return parser.finishNode(
		start,
		{
			type: "JSWithStatement",
			object,
			body,
		},
	);
}

export function parseEmptyStatement(
	parser: JSParser,
	start: Position,
): JSEmptyStatement {
	next(parser);
	return parser.finishNode(start, {type: "JSEmptyStatement"});
}

export function parseLabeledStatement(
	parser: JSParser,
	start: Position,
	maybeName: string,
	expr: JSReferenceIdentifier,
	context: StatementContext,
): JSLabeledStatement {
	for (const label of parser.state.labels) {
		if (label.name === maybeName) {
			unexpectedDiagnostic(
				parser,
				{
					loc: expr.loc,
					description: descriptions.JS_PARSER.DUPLICATE_LABEL(
						maybeName,
						label.loc,
					),
				},
			);
		}
	}

	let kind: LabelKind = undefined;
	if (parser.state.tokenType.isLoop) {
		kind = "loop";
	} else if (match(parser, tt._switch)) {
		kind = "switch";
	}

	const startIndex = parser.getIndexFromPosition(start, parser.filename);
	for (let i = parser.state.labels.length - 1; i >= 0; i--) {
		const label = parser.state.labels[i];
		if (label.statementStart === startIndex) {
			label.statementStart = parser.getIndex();
			label.kind = kind;
		} else {
			break;
		}
	}

	parser.state.labels.push({
		name: maybeName,
		kind,
		loc: parser.getLoc(expr),
		statementStart: parser.getIndex(),
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
		body.type === "JSClassDeclaration" ||
		(body.type === "JSVariableDeclarationStatement" &&
		body.declaration.kind !== "var") ||
		(body.type === "JSFunctionDeclaration" &&
		(inScope(parser, "STRICT") ||
		body.head.generator === true ||
		body.head.async === true))
	) {
		unexpectedDiagnostic(
			parser,
			{
				loc: body.loc,
				description: descriptions.JS_PARSER.INVALID_LABEL_DECLARATION,
			},
		);
	}

	parser.state.labels.pop();
	return parser.finishNode(
		start,
		{
			type: "JSLabeledStatement",
			label: {
				...expr,
				type: "JSIdentifier",
			},
			body,
		},
	);
}

export function parseExpressionStatement(
	parser: JSParser,
	start: Position,
	expr: AnyJSExpression,
): AnyJSStatement {
	const node = parseTSTypeExpressionStatement(parser, start, expr);
	if (node !== undefined) {
		return node;
	}

	semicolon(parser);
	return parser.finishNode(
		start,
		{
			type: "JSExpressionStatement",
			expression: expr,
		},
	);
}

export function parseBlock(
	parser: JSParser,
	allowDirectives?: boolean,
): JSBlockStatement {
	const start = parser.getPosition();
	const openContext = expectOpening(parser, tt.braceL, tt.braceR, "block");
	const {body, directives} = parseBlockBody(
		parser,
		allowDirectives,
		false,
		openContext,
	);
	return parser.finishNode(
		start,
		{
			type: "JSBlockStatement",
			directives,
			body,
		},
	);
}

export function isValidDirective(
	parser: JSParser,
	stmt: AnyJSStatement,
): boolean {
	return (
		stmt.type === "JSExpressionStatement" &&
		stmt.expression.type === "JSStringLiteral" &&
		!isParenthesized(parser, stmt.expression)
	);
}

export function parseBlockBody(
	parser: JSParser,
	allowDirectives: boolean = false,
	topLevel: boolean,
	openContext: OpeningContext,
): {
	body: Array<AnyJSStatement>;
	directives: Array<JSDirective>;
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
	body: Array<AnyJSStatement>;
	directives: Array<JSDirective>;
} {
	const body: Array<AnyJSStatement> = [];
	const directives: Array<JSDirective> = [];

	let parsedNonDirective = false;
	let didSetStrict = undefined;
	let octalPosition;

	while (true) {
		if (match(parser, openContext.close) || match(parser, tt.eof)) {
			expectClosing(parser, openContext);
			break;
		}

		if (!parsedNonDirective && parser.state.containsOctal && !octalPosition) {
			octalPosition = parser.state.octalPosition;
		}

		const stmt = parseStatement(parser, undefined, topLevel);

		if (
			allowDirectives &&
			!parsedNonDirective &&
			stmt.type === "JSExpressionStatement" &&
			isValidDirective(parser, stmt)
		) {
			const directive = expressionStatementToDirective(parser, stmt);
			directives.push(directive);

			if (didSetStrict === undefined && directive.value === "use strict") {
				setStrict(parser, true);
				didSetStrict = true;

				if (octalPosition !== undefined) {
					unexpectedDiagnostic(
						parser,
						{
							index: octalPosition,
							description: descriptions.JS_PARSER.OCTAL_IN_STRICT,
						},
					);
				}
			}

			continue;
		}

		parsedNonDirective = true;
		body.push(stmt);
	}

	if (didSetStrict) {
		popScope(parser, "STRICT");
	}

	return {body, directives};
}

export function parseFor(
	parser: JSParser,
	start: Position,
	openContext: OpeningContext,
	init: undefined | (JSVariableDeclaration | AnyJSExpression),
): JSForStatement {
	expect(parser, tt.semi);

	const test = match(parser, tt.semi)
		? undefined
		: parseExpression(parser, "for test");
	expect(parser, tt.semi);

	const update = match(parser, tt.parenR)
		? undefined
		: parseExpression(parser, "for update");
	expectClosing(parser, openContext);

	const body = parseStatement(parser, "for");
	parser.state.labels.pop();

	return parser.finishNode(
		start,
		{
			type: "JSForStatement",
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
	init: JSVariableDeclaration | AnyJSTargetAssignmentPattern,
	awaitAt: undefined | Position,
): AnyJSForInOfStatement {
	const isForIn: boolean = match(parser, tt._in);
	next(parser);

	const isAwait = awaitAt !== undefined;
	if (isForIn && isAwait) {
		unexpectedDiagnostic(
			parser,
			{
				start: awaitAt,
				description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
			},
		);
	}

	if (
		init.type === "JSVariableDeclaration" &&
		init.declarations[0].init !== undefined &&
		(!isForIn ||
		inScope(parser, "STRICT") ||
		init.kind !== "var" ||
		init.declarations[0].id.type !== "JSBindingIdentifier")
	) {
		unexpectedDiagnostic(
			parser,
			{
				loc: init.loc,
				description: descriptions.JS_PARSER.FOR_IN_OF_WITH_INITIALIZER,
			},
		);
	}

	const left = init;
	const right = isForIn
		? parseExpression(parser, "for right")
		: parseMaybeAssign(parser, "for right");
	expectClosing(parser, openContext);

	const body = parseStatement(parser, "for");
	parser.state.labels.pop();

	if (isForIn) {
		const node: JSForInStatement = parser.finishNode(
			start,
			{
				type: "JSForInStatement",
				left,
				right,
				body,
			},
		);
		return node;
	} else {
		const node: JSForOfStatement = parser.finishNode(
			start,
			{
				type: "JSForOfStatement",
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
): Array<JSVariableDeclarator> {
	const declarations: Array<JSVariableDeclarator> = [];

	while (true) {
		const start = parser.getPosition();
		const id = parseVarHead(parser, start);

		if (kind === "var") {
			parser.state.hasHoistedVars = true;
		}

		let init;
		if (eat(parser, tt.eq)) {
			init = parseMaybeAssign(parser, "var init", isFor);
		} else {
			if (
				kind === "const" &&
				!(match(parser, tt._in) || isContextual(parser, "of"))
			) {
				// `const` with no initializer is allowed in TypeScript.
				// It could be a declaration like `const x: number;`.
				if (!isSyntaxEnabled(parser, "ts")) {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.CONST_WITHOUT_INITIALIZER,
							loc: id.loc,
						},
					);
				}
			}

			// We exclude `const` because we already validated it above
			if (
				kind !== "const" &&
				id.type !== "JSBindingIdentifier" &&
				!(isFor && (match(parser, tt._in) || isContextual(parser, "of")))
			) {
				unexpectedDiagnostic(
					parser,
					{
						start: parser.state.lastEndPos,
						description: descriptions.JS_PARSER.COMPLEX_BINDING_WITHOUT_INITIALIZER,
					},
				);
			}
		}

		declarations.push(
			parser.finishNode(
				start,
				{
					type: "JSVariableDeclarator",
					id,
					init,
				},
			),
		);

		if (!eat(parser, tt.comma)) {
			break;
		}
	}

	return declarations;
}

export function parseVarHead(
	parser: JSParser,
	start: Position,
): AnyJSTargetBindingPattern {
	const id = parseTargetBindingPattern(parser);

	checkLVal(parser, id, true, undefined, "variable declaration");

	let definite: undefined | boolean;
	if (id.type === "JSBindingIdentifier" && match(parser, tt.bang)) {
		definite = true;
		expectSyntaxEnabled(parser, "ts");
		next(parser);
	}

	if (match(parser, tt.colon)) {
		const typeAnnotation = parseTSTypeAnnotation(parser, true);

		return parser.finishNode(
			start,
			{
				...id,
				meta: parser.finishNode(
					start,
					{
						type: "JSPatternMeta",
						typeAnnotation,
						definite,
					},
				),
			},
		);
	} else if (definite) {
		return {
			...id,
			meta: parser.finishNode(start, {type: "JSPatternMeta", definite}),
		};
	} else {
		return id;
	}
}

function parseFunctionId(
	parser: JSParser,
	requiredStatementId: boolean,
): undefined | JSBindingIdentifier {
	if (requiredStatementId || match(parser, tt.name)) {
		return parseBindingIdentifier(parser);
	} else {
		return undefined;
	}
}

export function parseFunctionDeclaration(
	parser: JSParser,
	start: Position,
	isAsync: boolean,
): JSFunctionDeclaration | TSDeclareFunction {
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
		type: "JSFunctionDeclaration",
		...shape,
		id,
		body,
	});
}

export function parseExportDefaultFunctionDeclaration(
	parser: JSParser,
	start: Position,
	isAsync: boolean,
): JSFunctionDeclaration | TSDeclareFunction {
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
			type: "JSBindingIdentifier",
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
		type: "JSFunctionDeclaration",
		...shape,
		id,
		body,
	});
}

export function parseFunctionExpression(
	parser: JSParser,
	start: Position,
	isAsync: boolean,
): JSFunctionExpression {
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
		type: "JSFunctionExpression",
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
	id: undefined | JSBindingIdentifier;
	head: JSFunctionHead;
	body: undefined | JSBlockStatement;
	loc: SourceLocation;
} {
	const {start, isStatement, requiredStatementId, isAsync} = opts;

	const isGenerator = eat(parser, tt.star);

	let id;
	if (isStatement) {
		id = parseFunctionId(parser, requiredStatementId);
	}

	const oldYieldPos = parser.state.yieldPos;
	const oldAwaitPos = parser.state.awaitPos;
	pushScope(parser, "FUNCTION_LOC", start);
	pushScope(parser, "FUNCTION", true);
	pushScope(parser, "METHOD", false);
	pushScope(parser, "GENERATOR", isGenerator);
	pushScope(parser, "ASYNC", isAsync);
	pushScope(parser, "CLASS_PROPERTY", false);
	pushScope(parser, "NON_ARROW_FUNCTION");
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

	popScope(parser, "NON_ARROW_FUNCTION");
	popScope(parser, "FUNCTION");
	popScope(parser, "FUNCTION_LOC");
	popScope(parser, "CLASS_PROPERTY");
	popScope(parser, "METHOD");
	popScope(parser, "GENERATOR");
	popScope(parser, "ASYNC");

	if (body !== undefined && body.type !== "JSBlockStatement") {
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
	params: JSFunctionHead["params"],
): {
	params: JSFunctionHead["params"];
	thisType: undefined | JSBindingIdentifier;
} {
	const firstParam = params[0];
	if (
		firstParam !== undefined &&
		firstParam.type === "JSBindingIdentifier" &&
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
	params: Array<AnyJSBindingPattern>;
	rest: undefined | AnyJSTargetBindingPattern;
} {
	let typeParameters = undefined;
	if (isRelational(parser, "<")) {
		typeParameters = maybeParseTSTypeParameters(parser);

		if (typeParameters !== undefined && (kind === "get" || kind === "set")) {
			unexpectedDiagnostic(
				parser,
				{
					loc: typeParameters.loc,
					description: descriptions.JS_PARSER.ACCESSOR_WITH_TYPE_PARAMS,
				},
			);
		}
	}

	pushScope(parser, "PARAMETERS", true);

	const openContext = expectOpening(
		parser,
		tt.parenL,
		tt.parenR,
		"function params",
	);
	const {list: params, rest} = parseBindingListNonEmpty(
		parser,
		openContext,
		allowTSModifiers,
	);

	popScope(parser, "PARAMETERS");
	checkYieldAwaitInDefaultParams(parser);
	return {params, rest, typeParameters};
}
