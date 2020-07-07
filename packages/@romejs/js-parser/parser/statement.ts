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
} from "@romejs/ast";
import * as charCodes from "@romejs/string-charcodes";
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
import {ob1Add, ob1Get0, ob1Inc, ob1Number0} from "@romejs/ob1";
import {descriptions} from "@romejs/diagnostics";

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
			sourceType: parser.sourceType,
			interpreter,
			syntax: Array.from(parser.syntax),
			hasHoistedVars: parser.state.hasHoistedVars,
		}),
	);
}

export function parsePossibleInterpreterDirective(
	parser: JSParser,
): undefined | JSInterpreterDirective {
	// Check for #!
	if (
		parser.match(tt.hash) &&
		parser.input[ob1Get0(parser.state.endPos.index)] === "!"
	) {
		const directive = skipInterpreterDirective(parser, 1);

		// Advance to next token
		parser.next();

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
		parser.getLoc(expr).start.index,
		parser.getLoc(expr).end.index,
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

	if (startType === tt._const && parser.isSyntaxEnabled("ts")) {
		const ahead = parser.lookaheadState();
		if (ahead.tokenType === tt.name && ahead.tokenValue === "enum") {
			parser.expect(tt._const);
			parser.expectContextual("enum");
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
			if (parser.lookaheadState().tokenType === tt.dot) {
				// JSMetaProperty: eg. function.sent
				break;
			}

			if (context !== undefined) {
				if (parser.inScope("STRICT")) {
					parser.unexpectedDiagnostic({
						description: descriptions.JS_PARSER.ILLEGAL_FUNCTION_IN_STRICT,
					});
				} else if (context !== "if" && context !== "label") {
					parser.unexpectedDiagnostic({
						description: descriptions.JS_PARSER.ILLEGAL_FUNCTION_IN_NON_STRICT,
					});
				}
			}

			parser.expect(tt._function);

			const result = parseFunctionDeclaration(parser, start, false);

			if (context !== undefined && result.head.generator === true) {
				parser.unexpectedDiagnostic({
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
				kind === undefined
					? assertVarKind(String(parser.state.tokenValue))
					: kind;
			if (context !== undefined && kind !== "var") {
				parser.unexpectedDiagnostic({
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
				parser.unexpectedDiagnostic({
					description: descriptions.JS_PARSER.IMPORT_EXPORT_MUST_TOP_LEVEL,
				});
			}

			assertModuleNodeAllowed(parser, result);

			return result;
		}

		case tt.name:
			if (isAsyncFunctionDeclarationStart(parser)) {
				if (context !== undefined) {
					parser.unexpectedDiagnostic({
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

	// brace, it's an JSExpressionStatement or JSLabeledStatement. We

	// simply start parsing an expression, and afterwards, if the

	// next token is a colon and the expression was a simple

	// JSIdentifier node, we switch to interpreting it as a label.
	const maybeName = String(parser.state.tokenValue);
	const expr = parseExpression(parser, "statement expression");

	if (
		startType === tt.name &&
		expr.type === "JSReferenceIdentifier" &&
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
		(node.type === "JSImportDeclaration" &&
		(node.importKind === "type" || node.importKind === "typeof")) ||
		(node.type === "JSExportLocalDeclaration" && node.exportKind === "type") ||
		(node.type === "JSExportAllDeclaration" && node.exportKind === "type")
	) {
		// Allow Flow type imports and exports in all conditions because
		// Flow itself does not care about 'sourceType'.
		return;
	}

	if (!parser.inModule) {
		parser.unexpectedDiagnostic({
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
): JSBreakStatement | JSContinueStatement {
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
		parser.unexpectedDiagnostic({
			start,
			description: descriptions.JS_PARSER.UNKNOWN_LABEL(label && label.name),
		});
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
	parser.next();
	parser.semicolon();
	return parser.finishNode(start, {type: "JSDebuggerStatement"});
}

export function parseDoStatement(
	parser: JSParser,
	start: Position,
): JSDoWhileStatement {
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

		const init: JSVariableDeclaration = parser.finishNode(
			initStart,
			{
				type: "JSVariableDeclaration",
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
			parser.unexpectedDiagnostic({
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
		parser.unexpectedDiagnostic({
			start: awaitAt,
			description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
		});
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
	parser.next();
	const test = parseParenExpression(parser, "if test");
	const consequent = parseStatement(parser, "if");
	const alternate = parser.eat(tt._else)
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
		!parser.inScope("FUNCTION") &&
		parser.sourceType !== "template" &&
		!parser.options.allowReturnOutsideFunction
	) {
		parser.unexpectedDiagnostic({
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
			type: "JSReturnStatement",
			argument,
		},
	);
}

export function parseSwitchStatement(
	parser: JSParser,
	start: Position,
): JSSwitchStatement {
	parser.expect(tt._switch);
	const discriminant = parseParenExpression(parser, "switch discriminant");
	const cases: Array<JSSwitchCase> = [];
	const hasBrace = parser.match(tt.braceL);
	const openContext = parser.expectOpening(tt.braceL, tt.braceR, "switch body");
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
			if (parser.match(tt.braceR) || parser.match(tt.eof)) {
				break;
			}

			if (parser.match(tt._case) || parser.match(tt._default)) {
				pushCase();

				const start = parser.getPosition();
				const isCase = parser.match(tt._case);

				parser.next();

				let test;
				if (isCase) {
					test = parseExpression(parser, "case test");
				} else {
					if (sawDefault) {
						// TODO point to other default
						parser.unexpectedDiagnostic({
							start: parser.state.lastStartPos,
							description: descriptions.JS_PARSER.MULTIPLE_DEFAULT_CASE,
						});
					}
					sawDefault = true;
				}

				cur = {
					start,
					consequent: [],
					test,
				};

				parser.expect(tt.colon);
			} else {
				const stmt = parseStatement(parser, undefined);
				if (cur === undefined) {
					parser.unexpectedDiagnostic({
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
	parser.next();
	if (
		lineBreak.test(
			parser.getRawInput(
				parser.state.lastEndPos.index,
				parser.state.startPos.index,
			),
		)
	) {
		parser.unexpectedDiagnostic({
			start: parser.state.lastEndPos,
			description: descriptions.JS_PARSER.NEWLINE_AFTER_THROW,
		});
	}

	const argument = parseExpression(parser, "throw argument");
	parser.semicolon();
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
	parser.next();

	const block = parseBlock(parser);
	let handler: undefined | JSCatchClause = undefined;

	if (parser.match(tt._catch)) {
		const clauseStart = parser.getPosition();
		parser.next();

		let param: undefined | AnyJSBindingPattern;
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
				type: "JSCatchClause",
				body,
				param,
			},
		);
	}

	const finalizer = parser.eat(tt._finally) ? parseBlock(parser) : undefined;

	if (!handler && !finalizer) {
		parser.unexpectedDiagnostic({
			start,
			description: descriptions.JS_PARSER.TRY_MISSING_FINALLY_OR_CATCH,
		});
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
	parser.next();
	const declarations = parseVar(parser, start, kind, false);
	parser.semicolon();
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
	parser.next();
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
	parser.next();
	const object = parseParenExpression(parser, "with object");
	const body = parseStatement(parser, "with");

	if (parser.inScope("STRICT")) {
		parser.unexpectedDiagnostic({
			loc: parser.finishLoc(start),
			description: descriptions.JS_PARSER.WITH_IN_STRICT,
		});
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
	parser.next();
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
			parser.unexpectedDiagnostic({
				loc: expr.loc,
				description: descriptions.JS_PARSER.DUPLICATE_LABEL(
					maybeName,
					label.loc,
				),
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
		body.type === "JSClassDeclaration" ||
		(body.type === "JSVariableDeclarationStatement" &&
		body.declaration.kind !== "var") ||
		(body.type === "JSFunctionDeclaration" &&
		(parser.inScope("STRICT") ||
		body.head.generator === true ||
		body.head.async === true))
	) {
		parser.unexpectedDiagnostic({
			loc: body.loc,
			description: descriptions.JS_PARSER.INVALID_LABEL_DECLARATION,
		});
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

	parser.semicolon();
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
		!parser.isParenthesized(stmt.expression)
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
			stmt.type === "JSExpressionStatement" &&
			isValidDirective(parser, stmt)
		) {
			const directive = expressionStatementToDirective(parser, stmt);
			directives.push(directive);

			if (didSetStrict === undefined && directive.value === "use strict") {
				setStrict(parser, true);
				didSetStrict = true;

				if (octalPosition !== undefined) {
					parser.unexpectedDiagnostic({
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
	init: undefined | (JSVariableDeclaration | AnyJSExpression),
): JSForStatement {
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
	const isForIn: boolean = parser.match(tt._in);
	parser.next();

	const isAwait = awaitAt !== undefined;
	if (isForIn && isAwait) {
		parser.unexpectedDiagnostic({
			start: awaitAt,
			description: descriptions.JS_PARSER.REGULAR_FOR_AWAIT,
		});
	}

	if (
		init.type === "JSVariableDeclaration" &&
		init.declarations[0].init !== undefined &&
		(!isForIn ||
		parser.inScope("STRICT") ||
		init.kind !== "var" ||
		init.declarations[0].id.type !== "JSBindingIdentifier")
	) {
		parser.unexpectedDiagnostic({
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
		if (parser.eat(tt.eq)) {
			init = parseMaybeAssign(parser, "var init", isFor);
		} else {
			if (
				kind === "const" &&
				!(parser.match(tt._in) || parser.isContextual("of"))
			) {
				// `const` with no initializer is allowed in TypeScript.
				// It could be a declaration like `const x: number;`.
				if (!parser.isSyntaxEnabled("ts")) {
					parser.unexpectedDiagnostic({
						description: descriptions.JS_PARSER.CONST_WITHOUT_INITIALIZER,
						loc: id.loc,
					});
				}
			}

			// We exclude `const` because we already validated it above
			if (
				kind !== "const" &&
				id.type !== "JSBindingIdentifier" &&
				!(isFor && (parser.match(tt._in) || parser.isContextual("of")))
			) {
				parser.unexpectedDiagnostic({
					start: parser.state.lastEndPos,
					description: descriptions.JS_PARSER.COMPLEX_BINDING_WITHOUT_INITIALIZER,
				});
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

		if (!parser.eat(tt.comma)) {
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
	if (id.type === "JSBindingIdentifier" && parser.match(tt.bang)) {
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
	if (parser.isRelational("<")) {
		typeParameters = maybeParseTSTypeParameters(parser);

		if (typeParameters !== undefined && (kind === "get" || kind === "set")) {
			parser.unexpectedDiagnostic({
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
