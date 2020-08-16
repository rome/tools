/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from "@internal/parser-core";
import {
	JSParser,
	OpeningContext,
	addParenthesized,
	assertNoSpace,
	banUnicodeEscape,
	canInsertSemicolon,
	cloneNode,
	createBranch,
	createUnknownIdentifier,
	eat,
	expect,
	expectClosing,
	expectOpening,
	getLastScope,
	hasPrecedingLineBreak,
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
	resetStartLocationFromNode,
	shouldTokenizeJSX,
	tryBranch,
	unexpectedDiagnostic,
	unexpectedToken,
} from "../parser";
import {
	NumberTokenValue,
	RegExpTokenValue,
	finishToken,
	readRegexp,
} from "../tokenizer/index";
import * as charCodes from "@internal/string-charcodes";
import {types as tt} from "../tokenizer/types";
import {
	IndexTracker,
	createIndexTracker,
	isKeyword,
	isReservedWord,
	isStrictBindReservedWord,
	isStrictReservedWord,
} from "@internal/js-parser-utils";
import {
	AnyJSBindingPattern,
	AnyJSExpression,
	AnyJSObjectMember,
	AnyJSObjectPropertyKey,
	AnyJSTargetBindingPattern,
	AnyNode,
	AnyTSPrimary,
	AssignmentOperator,
	BinaryOperator,
	JSAmbiguousFlowTypeCastExpression,
	JSArrayExpression,
	JSArrayHole,
	JSArrowFunctionExpression,
	JSAssignmentIdentifier,
	JSAwaitExpression,
	JSBigIntLiteral,
	JSBinaryExpression,
	JSBindingIdentifier,
	JSBindingObjectPattern,
	JSBindingObjectPatternProperty,
	JSBlockStatement,
	JSBooleanLiteral,
	JSCallExpression,
	JSClassMethod,
	JSClassMethodKind,
	JSClassPrivateMethod,
	JSComputedPropertyKey,
	JSDoExpression,
	JSFunctionExpression,
	JSFunctionHead,
	JSIdentifier,
	JSImportCall,
	JSLogicalExpression,
	JSMemberExpression,
	JSMetaProperty,
	JSNewExpression,
	JSNullLiteral,
	JSNumericLiteral,
	JSObjectExpression,
	JSObjectMethod,
	JSObjectMethodKind,
	JSObjectProperty,
	JSOptionalCallExpression,
	JSPrivateName,
	JSReferenceIdentifier,
	JSRegExpLiteral,
	JSSpreadElement,
	JSSpreadProperty,
	JSStaticPropertyKey,
	JSStringLiteral,
	JSSuper,
	JSTaggedTemplateExpression,
	JSTemplateElement,
	JSTemplateLiteral,
	JSUnaryExpression,
	JSUpdateExpression,
	JSYieldExpression,
	LogicalOperator,
	TSAsExpression,
	TSConstKeyword,
	TSDeclareFunction,
	TSDeclareMethod,
	TSTypeParameterInstantiation,
	UnaryOperator,
	UpdateOperator,
} from "@internal/ast";
import {types as tc} from "../tokenizer/context";
import {
	ToReferencedItem,
	checkCommaAfterRestFromSpread,
	checkLVal,
	filterSpread,
	maybeParseTSTypeParameters,
	parseBlock,
	parseClassExpression,
	parseFunctionExpression,
	parseFunctionParams,
	parseJSXElement,
	parseMaybeDefault,
	parseSpread,
	parseTSTypeAnnotation,
	parseTSTypeArguments,
	parseTSTypeAssertion,
	parseTSTypeOrTypePredicateAnnotation,
	parseTSTypeParameters,
	raiseRestNotLast,
	toAssignmentPattern,
	toFunctionParamsBindingList,
	toReferencedList,
	toReferencedListDeep,
	toReferencedListOptional,
	tryTSNextParseConstantContext,
	tsCheckLiteralForConstantContext,
	tsNextThenParseType,
} from "./index";
import {
	Number0,
	ob1Get0,
	ob1Inc,
	ob1Number0,
	ob1Number0Neg1,
} from "@internal/ob1";
import {splitFunctionParams} from "./statement";
import {parseRegExp} from "@internal/codec-js-regexp";
import {descriptions} from "@internal/diagnostics";

export function checkPropClash(
	parser: JSParser,
	prop: AnyJSObjectMember | JSBindingObjectPatternProperty,
	props: Set<string>,
): void {
	if (
		prop.key.type === "JSComputedPropertyKey" ||
		prop.type === "JSObjectMethod"
	) {
		return undefined;
	}

	const key = prop.key.value;

	// We can only check these for collisions since they're statically known
	if (
		key.type !== "JSIdentifier" &&
		key.type !== "JSStringLiteral" &&
		key.type !== "JSNumericLiteral"
	) {
		return;
	}

	// It is either an JSIdentifier or a String/NumericLiteral
	const name = key.type === "JSIdentifier" ? key.name : String(key.value);

	if (name === "__proto__") {
		if (props.has("proto")) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.PROTO_PROP_REDEFINITION,
					loc: key.loc,
				},
			);
		} else {
			props.add("proto");
		}
	}
}

export function parseExpression(
	parser: JSParser,
	context: ExpressionContext,
	noIn?: boolean,
	refShorthandDefaultPos?: IndexTracker,
): AnyJSExpression {
	const startPos = parser.state.startPos;
	const expr = parseMaybeAssign(parser, context, noIn, refShorthandDefaultPos);
	if (match(parser, tt.comma)) {
		let expressions: Array<AnyJSExpression> = [expr];
		while (eat(parser, tt.comma)) {
			expressions.push(
				parseMaybeAssign(parser, context, noIn, refShorthandDefaultPos),
			);
		}

		expressions = filterSpread(parser, toReferencedList(parser, expressions));

		return parser.finishNode(
			startPos,
			{
				type: "JSSequenceExpression",
				expressions,
			},
		);
	}
	return expr;
}

export function parseMaybeAssign<T extends AnyNode = AnyJSExpression>(
	parser: JSParser,
	context: ExpressionContext,
	noIn?: boolean,
	refShorthandDefaultPos?: IndexTracker,
	afterLeftParse?: MaybeAssignAfterParse<T>,
	refNeedsArrowPos?: IndexTracker,
): AnyJSExpression | T {
	const branches = createBranch<AnyJSExpression | T>(parser);

	// Try parsing as JSX
	if (
		(isRelational(parser, "<") || match(parser, tt.jsxTagStart)) &&
		shouldTokenizeJSX(parser)
	) {
		branches.add(
			() => {
				return _parseMaybeAssign(
					parser,
					context,
					noIn,
					refShorthandDefaultPos,
					afterLeftParse,
					refNeedsArrowPos,
				);
			},
			{diagnosticsPriority: 1},
		);

		// Remove `tc.j_expr` and `tc.j_oTag` from 'context added
		// by parsing `jsxTagStart` to stop the JSX plugin from
		// messing with the tokens
		const cLength = parser.state.context.length;
		if (parser.state.context[cLength - 1] === tc.jsxOpenTag) {
			parser.state.context.length -= 2;
		}
		finishToken(parser, tt.relational, "<");
	}

	// Try parsing as an arrow function with type parameters
	if (isRelational(parser, "<")) {
		branches.add(() => {
			const start = parser.getPosition();
			const typeParameters = parseTSTypeParameters(parser);
			const possibleArrow = forwardNoArrowParamsConversionAt(
				parser,
				start,
				() =>
					_parseMaybeAssign<T>(
						parser,
						context,
						noIn,
						refShorthandDefaultPos,
						afterLeftParse,
						refNeedsArrowPos,
					)
				,
			);
			resetStartLocationFromNode(parser, possibleArrow, typeParameters);

			if (possibleArrow.type === "JSArrowFunctionExpression") {
				// `as` cast for reasons... `possibleArrow` is `T | JSArrowFunctionExpression`
				const arrow = (possibleArrow as JSArrowFunctionExpression);
				return {
					...arrow,
					head: {
						...arrow.head,
						typeParameters,
					},
				};
			} else {
				unexpectedDiagnostic(
					parser,
					{
						loc: typeParameters.loc,
						description: descriptions.JS_PARSER.EXPECTED_ARROW_AFTER_TYPE_PARAMS,
					},
				);
				return toReferenceIdentifier(
					parser,
					createUnknownIdentifier(parser, "type params without arrow function"),
				);
			}
		});
	}

	if (branches.hasOptimalBranch()) {
		return branches.pick();
	}

	return _parseMaybeAssign<T>(
		parser,
		context,
		noIn,
		refShorthandDefaultPos,
		afterLeftParse,
		refNeedsArrowPos,
	);
}

type MaybeAssignAfterParse<T> = (
	parser: JSParser,
	left: AnyJSExpression,
	startPos: Position,
) => T;

function _parseMaybeAssign<T extends AnyNode>(
	parser: JSParser,
	context: ExpressionContext,
	noIn?: boolean,
	refShorthandDefaultPos?: IndexTracker,
	afterLeftParse?: MaybeAssignAfterParse<T>,
	refNeedsArrowPos?: IndexTracker,
): AnyJSExpression | T {
	const startPos = parser.state.startPos;

	if (isContextual(parser, "yield")) {
		if (inScope(parser, "GENERATOR")) {
			let left: T | AnyJSExpression = parseYield(parser, noIn);
			if (afterLeftParse) {
				left = afterLeftParse(parser, left, startPos);
			}
			return left;
		} else {
			// The tokenizer will assume an expression is allowed after
			// `yield`, but this isn't that kind of yield
			parser.state.exprAllowed = false;
		}
	}

	const oldCommaAfterSpreadAt = parser.state.commaAfterSpreadAt;
	parser.state.commaAfterSpreadAt = ob1Number0Neg1;

	let failOnShorthandAssign;
	if (refShorthandDefaultPos) {
		failOnShorthandAssign = false;
	} else {
		refShorthandDefaultPos = createIndexTracker();
		failOnShorthandAssign = true;
	}

	if (match(parser, tt.parenL) || match(parser, tt.name)) {
		parser.state.potentialArrowAt = parser.getIndex();
	}

	let left: AnyJSExpression | T = parseMaybeConditional(
		parser,
		context,
		noIn,
		refShorthandDefaultPos,
		refNeedsArrowPos,
	);
	if (afterLeftParse) {
		left = afterLeftParse(parser, left, startPos);
	}

	if (parser.state.tokenType.isAssign) {
		const operator = (String(parser.state.tokenValue) as AssignmentOperator);
		const leftPatt = toAssignmentPattern(parser, left, "assignment expression");

		// reset because shorthand default was used correctly
		refShorthandDefaultPos.index = ob1Number0;

		checkLVal(parser, leftPatt, undefined, undefined, "assignment expression");

		// We should never get patterns here...?

		//if (left.type === 'BindingArrayPattern' || left.type === 'BindingObjectPattern') {
		//  checkCommaAfterRestFromSpread(parser);

		//}
		parser.state.commaAfterSpreadAt = oldCommaAfterSpreadAt;

		next(parser);
		const right = parseMaybeAssign(parser, "assignment right", noIn);
		return parser.finishNode(
			startPos,
			{
				type: "JSAssignmentExpression",
				operator,
				left: leftPatt,
				right,
			},
		);
	} else if (failOnShorthandAssign && ob1Get0(refShorthandDefaultPos.index) > 0) {
		unexpectedToken(
			parser,
			parser.getPositionFromIndex(refShorthandDefaultPos.index),
		);
	}

	parser.state.commaAfterSpreadAt = oldCommaAfterSpreadAt;

	return left;
}

export function parseMaybeConditional(
	parser: JSParser,
	context: ExpressionContext,
	noIn: undefined | boolean,
	refShorthandDefaultPos: IndexTracker,
	refNeedsArrowPos?: IndexTracker,
): AnyJSExpression {
	const startPos = parser.state.startPos;
	const potentialArrowAt = parser.state.potentialArrowAt;
	const expr = parseExpressionOps(parser, context, noIn, refShorthandDefaultPos);

	if (
		expr.type === "JSArrowFunctionExpression" &&
		parser.getInputStartIndex(expr) === potentialArrowAt
	) {
		return expr;
	}

	if (refShorthandDefaultPos && ob1Get0(refShorthandDefaultPos.index) > 0) {
		return expr;
	}

	return parseConditional(parser, expr, noIn, startPos, refNeedsArrowPos);
}

export function tryParseConditionalConsequent(
	parser: JSParser,
): {
	consequent: AnyJSExpression;
	failed: boolean;
} {
	const brancher = createBranch<{
		consequent: AnyJSExpression;
		failed: boolean;
	}>(parser);

	brancher.add(() => {
		parser.state.noArrowParamsConversionAt.push(parser.getIndex());
		const consequent = parseMaybeAssign(parser, "conditional consequent");
		parser.state.noArrowParamsConversionAt.pop();
		return {
			consequent,
			failed: !match(parser, tt.colon),
		};
	});

	return brancher.pick();
}

export function parseConditional(
	parser: JSParser,
	expr: AnyJSExpression,
	noIn: undefined | boolean,
	startPos: Position,
	refNeedsArrowPos?: IndexTracker,
): AnyJSExpression {
	if (!match(parser, tt.question)) {
		return expr;
	}

	// This is to handle a case like this: const foo = (foo?: bar) => {};
	// We'll be called due to the `?`, and we should mark ourselves as an
	// expected arrow function if parsing as a regular conditional fails
	if (refNeedsArrowPos) {
		const branch = createBranch<AnyJSExpression>(parser);

		branch.add(
			() => _parseConditional(parser, expr, noIn, startPos),
			{
				maxNewDiagnostics: 0,
			},
		);

		if (branch.hasBranch()) {
			return branch.pick();
		} else {
			refNeedsArrowPos.index = parser.getIndex();
			return expr;
		}
	}

	expect(parser, tt.question);
	const originalNoArrowAt = parser.state.noArrowAt;
	let {consequent} = tryParseConditionalConsequent(parser);
	parser.state.noArrowAt = originalNoArrowAt;

	if (!eat(parser, tt.colon)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.MISSING_CONDITIONAL_SEPARATOR,
			},
		);
	}

	const alternate = forwardNoArrowParamsConversionAt(
		parser,
		startPos,
		() =>
			parseMaybeAssign(
				parser,
				"conditional alternate",
				noIn,
				undefined,
				undefined,
				undefined,
			)
		,
	);

	return parser.finishNode(
		startPos,
		{
			type: "JSConditionalExpression",
			test: expr,
			consequent,
			alternate,
		},
	);
}

export function forwardNoArrowParamsConversionAt<T>(
	parser: JSParser,
	start: Position,
	parse: () => T,
): T {
	if (
		parser.state.noArrowParamsConversionAt.includes(
			parser.getIndexFromPosition(start, parser.filename),
		)
	) {
		let result: T;
		parser.state.noArrowParamsConversionAt.push(parser.getIndex());
		result = parse();
		parser.state.noArrowParamsConversionAt.pop();
		return result;
	} else {
		return parse();
	}
}

function _parseConditional(
	parser: JSParser,
	expr: AnyJSExpression,
	noIn: undefined | boolean,
	startPos: Position,
): AnyJSExpression {
	if (eat(parser, tt.question)) {
		const test = expr;
		const consequent = parseMaybeAssign(parser, "conditional consequent");
		expect(parser, tt.colon);
		const alternate = parseMaybeAssign(parser, "conditional alternate", noIn);
		return parser.finishNode(
			startPos,
			{
				type: "JSConditionalExpression",
				test,
				consequent,
				alternate,
			},
		);
	}
	return expr;
}

export function parseExpressionOps(
	parser: JSParser,
	context: ExpressionContext,
	noIn: undefined | boolean,
	refShorthandDefaultPos: IndexTracker,
): AnyJSExpression {
	const startPos = parser.state.startPos;
	const potentialArrowAt = parser.state.potentialArrowAt;
	const expr = parseMaybeUnary(parser, context, refShorthandDefaultPos);

	if (
		expr.type === "JSArrowFunctionExpression" &&
		parser.getInputStartIndex(expr) === potentialArrowAt
	) {
		return expr;
	}
	if (refShorthandDefaultPos && ob1Get0(refShorthandDefaultPos.index) > 0) {
		return expr;
	}

	return parseExpressionOp(parser, context, expr, startPos, -1, noIn);
}

export function parseExpressionOp(
	parser: JSParser,
	context: ExpressionContext,
	left: AnyJSExpression,
	leftStartPos: Position,
	minPrec: number,
	noIn: boolean = false,
): AnyJSExpression {
	if (
		tt._in.getBinop() > minPrec &&
		!hasPrecedingLineBreak(parser) &&
		isContextual(parser, "as")
	) {
		const _const = tryTSNextParseConstantContext(parser);

		let typeAnnotation;
		if (_const) {
			tsCheckLiteralForConstantContext(parser, left);
			typeAnnotation = _const;
		} else {
			typeAnnotation = tsNextThenParseType(parser);
		}

		const node: TSAsExpression = parser.finishNode(
			leftStartPos,
			{
				type: "TSAsExpression",
				typeAnnotation,
				expression: left,
			},
		);

		return parseExpressionOp(parser, context, node, leftStartPos, minPrec, noIn);
	}

	const prec = parser.state.tokenType.binop;
	if (prec !== undefined && (!noIn || !match(parser, tt._in))) {
		if (prec > minPrec) {
			const operator = (String(parser.state.tokenValue) as
				| BinaryOperator
				| LogicalOperator);

			if (
				operator === "**" &&
				left.type === "JSUnaryExpression" &&
				!isParenthesized(parser, left)
			) {
				unexpectedDiagnostic(
					parser,
					{
						loc: left.argument.loc,
						description: descriptions.JS_PARSER.WRAP_EXPONENTIATION,
					},
				);
			}

			const op = parser.state.tokenType;
			next(parser);

			const startPos = parser.state.startPos;

			const right = parseExpressionOp(
				parser,
				context,
				parseMaybeUnary(parser, context),
				startPos,
				op.rightAssociative ? prec - 1 : prec,
				noIn,
			);

			let node: JSLogicalExpression | JSBinaryExpression;
			if (operator === "||" || operator === "&&" || operator === "??") {
				node = parser.finishNode(
					leftStartPos,
					{
						type: "JSLogicalExpression",
						left,
						right,
						operator,
					},
				);
			} else {
				node = parser.finishNode(
					leftStartPos,
					{
						type: "JSBinaryExpression",
						left,
						right,
						operator,
					},
				);
			}

			return parseExpressionOp(
				parser,
				context,
				node,
				leftStartPos,
				minPrec,
				noIn,
			);
		}
	}

	return left;
}

// Parse unary operators, both prefix and postfix.
export function parseMaybeUnary(
	parser: JSParser,
	context: ExpressionContext,
	refShorthandDefaultPos?: IndexTracker,
): AnyJSExpression {
	if (
		isSyntaxEnabled(parser, "ts") &&
		!isSyntaxEnabled(parser, "jsx") &&
		isRelational(parser, "<")
	) {
		return parseTSTypeAssertion(parser);
	}

	if (isContextual(parser, "await") && inScope(parser, "ASYNC")) {
		return parseAwait(parser);
	}

	if (parser.state.tokenType.prefix) {
		const start = parser.getPosition();
		const update = match(parser, tt.incDec);
		const operator = (String(parser.state.tokenValue) as
			| UnaryOperator
			| UpdateOperator);
		const prefix = true;

		next(parser);

		const argument = parseMaybeUnary(parser, context);

		if (refShorthandDefaultPos && ob1Get0(refShorthandDefaultPos.index) > 0) {
			unexpectedToken(
				parser,
				parser.getPositionFromIndex(refShorthandDefaultPos.index),
			);
		}

		if (update) {
			checkLVal(parser, argument, undefined, undefined, "prefix operation");
		} else if (inScope(parser, "STRICT") && operator === "delete") {
			if (argument.type === "JSReferenceIdentifier") {
				unexpectedDiagnostic(
					parser,
					{
						loc: argument.loc,
						description: descriptions.JS_PARSER.DELETE_LOCAL_VARIABLE_IN_STRICT,
					},
				);
			} else if (
				argument.type === "JSMemberExpression" &&
				argument.property.value.type === "JSPrivateName"
			) {
				unexpectedDiagnostic(
					parser,
					{
						loc: argument.property.loc,
						description: descriptions.JS_PARSER.DELETE_PRIVATE_FIELD,
					},
				);
			}
		}

		let node: JSUpdateExpression | JSUnaryExpression;
		if (update) {
			if (operator !== "++" && operator !== "--") {
				throw new Error("Expected ++/-- operator only for JSUpdateExpression");
			}

			node = parser.finishNode(
				start,
				{
					type: "JSUpdateExpression",
					argument,
					operator,
					prefix,
				},
			);
		} else {
			if (operator === "++" || operator === "--") {
				throw new Error("BinaryExpression cannot have ++/-- operator");
			}

			node = parser.finishNode(
				start,
				{
					type: "JSUnaryExpression",
					argument,
					operator,
					prefix,
				},
			);
		}

		return node;
	}

	const startPos = parser.state.startPos;

	let expr = parseExpressionWithPossibleSubscripts(
		parser,
		context,
		refShorthandDefaultPos,
	);
	if (refShorthandDefaultPos && ob1Get0(refShorthandDefaultPos.index) > 0) {
		return expr;
	}

	while (parser.state.tokenType.postfix && !canInsertSemicolon(parser)) {
		const operator = (String(parser.state.tokenValue) as UpdateOperator);
		checkLVal(parser, expr, undefined, undefined, "postfix operation");
		next(parser);

		const updateNode: JSUpdateExpression = parser.finishNode(
			startPos,
			{
				type: "JSUpdateExpression",
				operator,
				prefix: false,
				argument: expr,
			},
		);
		expr = updateNode;
	}

	return expr;
}

// Parse call, dot, and `[]`-subscript expressions.
export function parseExpressionWithPossibleSubscripts(
	parser: JSParser,
	context: ExpressionContext,
	refShorthandDefaultPos?: IndexTracker,
): AnyJSExpression {
	const startPos = parser.state.startPos;
	const potentialArrowAt = parser.state.potentialArrowAt;
	const expr = parseExpressionAtom(parser, context, refShorthandDefaultPos);

	if (
		expr.type === "JSArrowFunctionExpression" &&
		parser.getInputStartIndex(expr) === potentialArrowAt
	) {
		return expr;
	}

	if (refShorthandDefaultPos && ob1Get0(refShorthandDefaultPos.index) > 0) {
		return expr;
	}

	return parseSubscripts(parser, expr, startPos);
}

export function parseSubscripts(
	parser: JSParser,
	base: AnyJSExpression,
	startPos: Position,
	noCalls?: boolean,
): AnyJSExpression {
	const maybeAsyncArrow = atPossibleAsync(parser, base);

	if (
		base.type === "JSReferenceIdentifier" &&
		base.name === "async" &&
		parser.state.noArrowAt.includes(
			parser.getIndexFromPosition(startPos, parser.filename),
		)
	) {
		const argsStart = parser.getPosition();
		const openContext = expectOpening(
			parser,
			tt.parenL,
			tt.parenR,
			"call arguments",
		);
		const callee = base;
		const {args} = parseCallExpressionArguments(parser, openContext, false);
		base = parser.finishNodeWithStarts(
			[argsStart, startPos],
			{
				type: "JSCallExpression",
				callee,
				arguments: args,
			},
		);
	} else if (
		base.type === "JSReferenceIdentifier" &&
		base.name === "async" &&
		isRelational(parser, "<")
	) {
		const branch = createBranch<AnyJSExpression>(parser);
		branch.add(() => parseAsyncArrowWithTypeParameters(parser, startPos));
		branch.add(() =>
			parseExpressionSubscriptsRecursively(
				parser,
				base,
				startPos,
				noCalls,
				maybeAsyncArrow,
			)
		);
		return branch.pick();
	}

	return parseExpressionSubscriptsRecursively(
		parser,
		base,
		startPos,
		noCalls,
		maybeAsyncArrow,
	);
}

export function parseAsyncArrowWithTypeParameters(
	parser: JSParser,
	startPos: Position,
): undefined | JSArrowFunctionExpression {
	const {params, rest, typeParameters} = parseFunctionParams(parser);

	const {returnType, valid} = parseArrowHead(parser);
	if (!valid) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.INVALID_ASYNC_ARROW_WITH_TYPE_PARAMS,
			},
		);
		return undefined;
	}

	const func = parseArrowExpression(
		parser,
		startPos,
		{
			bindingList: params,
			rest,
		},
		/* isAsync */ true,
	);

	return {
		...func,
		head: {
			...func.head,
			returnType,
			typeParameters,
		},
	};
}

function parseExpressionSubscriptsRecursively(
	parser: JSParser,
	base: AnyJSExpression,
	startPos: Position,
	noCalls: undefined | boolean,
	maybeAsyncArrow: boolean,
): AnyJSExpression {
	const state: ParseSubscriptState = {
		optionalChainMember: false,
		stop: false,
	};
	do {
		base = parseExpressionSubscript(
			parser,
			base,
			startPos,
			noCalls,
			state,
			maybeAsyncArrow,
		);
	} while (!state.stop);
	return base;
}

type ParseSubscriptState = {
	stop: boolean;
	optionalChainMember: boolean;
};

export function parseExpressionSubscript(
	parser: JSParser,
	base: AnyJSExpression,
	startPos: Position,
	noCalls: boolean = false,
	state: ParseSubscriptState,
	maybeAsyncArrow: boolean,
): AnyJSExpression {
	if (!hasPrecedingLineBreak(parser) && match(parser, tt.bang)) {
		parser.state.exprAllowed = false;
		next(parser);

		return parser.finishNode(
			startPos,
			{
				type: "TSNonNullExpression",
				expression: base,
			},
		);
	}

	if (match(parser, tt.questionDot)) {
		state.optionalChainMember = true;

		if (noCalls && lookaheadState(parser).tokenType === tt.parenL) {
			state.stop = true;
			return base;
		}

		next(parser);

		// eg: o.m?.<T>(e);
		if (isRelational(parser, "<")) {
			if (noCalls) {
				state.stop = true;
				return base;
			}

			const callee = base;
			const typeArguments = parseTSTypeArguments(parser);
			const openContext = expectOpening(
				parser,
				tt.parenL,
				tt.parenR,
				"call arguments",
			);
			const {args} = parseCallExpressionArguments(parser, openContext, false);
			return parser.finishNode(
				startPos,
				{
					type: "JSOptionalCallExpression",
					arguments: args,
					callee,
					typeArguments,
				},
			);
		}

		if (match(parser, tt.bracketL)) {
			const propStart = parser.getPosition();
			const openContext = expectOpening(
				parser,
				tt.bracketL,
				tt.bracketR,
				"computed property",
			);
			const object = base;
			const property = parseExpression(
				parser,
				"optional member expression property",
			);
			expectClosing(parser, openContext);
			return parser.finishNode(
				startPos,
				{
					type: "JSMemberExpression",
					object,
					property: parser.finishNode(
						propStart,
						{
							type: "JSComputedMemberProperty",
							optional: true,
							value: property,
						},
					),
				},
			);
		}

		if (match(parser, tt.parenL)) {
			const openContext = expectOpening(
				parser,
				tt.parenL,
				tt.parenR,
				"call arguments",
			);
			const callee = base;
			const {args} = parseCallExpressionArguments(parser, openContext, false);

			return parser.finishNode(
				startPos,
				{
					type: "JSOptionalCallExpression",
					callee,
					arguments: args,
				},
			);
		}

		const object = base;
		const property = parseIdentifier(parser, true);

		return parser.finishNode(
			startPos,
			{
				type: "JSMemberExpression",
				object,
				property: {
					type: "JSStaticMemberProperty",
					loc: property.loc,
					optional: true,
					value: property,
				},
			},
		);
	}

	if (eat(parser, tt.dot)) {
		const object = base;
		const property = parseMaybePrivateName(parser);

		return parser.finishNode(
			startPos,
			{
				type: "JSMemberExpression",
				object,
				property: {
					type: "JSStaticMemberProperty",
					loc: property.loc,
					value: property,
				},
			},
		);
	}

	if (match(parser, tt.bracketL)) {
		const propStart = parser.getPosition();
		const openContext = expectOpening(
			parser,
			tt.bracketL,
			tt.bracketR,
			"computed property",
		);
		const object = base;
		const property = parseExpression(
			parser,
			"member expression computed property",
		);
		expectClosing(parser, openContext);

		return parser.finishNode(
			startPos,
			{
				type: "JSMemberExpression",
				object,
				property: parser.finishNode(
					propStart,
					{
						type: "JSComputedMemberProperty",
						value: property,
					},
				),
			},
		);
	}

	// Supports: foo<Foo>(); and foo<Foo>``;
	if (isRelational(parser, "<") && isSyntaxEnabled(parser, "ts")) {
		const possibleCallExpression = tryBranch(
			parser,
			() => {
				const typeArguments = parseTSTypeArguments(parser);

				if (!noCalls && match(parser, tt.parenL)) {
					const argsStart = parser.getPosition();
					const openContext = expectOpening(
						parser,
						tt.parenL,
						tt.parenR,
						"call arguments",
					);
					const {args} = parseCallExpressionArguments(
						parser,
						openContext,
						false,
					);
					const node: JSCallExpression = parser.finishNodeWithStarts(
						[argsStart, startPos],
						{
							type: "JSCallExpression",
							arguments: args,
							callee: base,
							typeArguments,
						},
					);
					return node;
				}

				if (match(parser, tt.backQuote)) {
					return parseTaggedTemplateExpression(
						parser,
						startPos,
						base,
						state,
						typeArguments,
					);
				}

				return undefined;
			},
		);

		if (possibleCallExpression !== undefined) {
			return possibleCallExpression;
		}
	}

	if (!noCalls && match(parser, tt.parenL)) {
		const oldMaybeInArrowParameters = parser.state.maybeInArrowParameters;
		const oldYieldPos = parser.state.yieldPos;
		const oldAwaitPos = parser.state.awaitPos;
		parser.state.maybeInArrowParameters = true;
		parser.state.yieldPos = ob1Number0;
		parser.state.awaitPos = ob1Number0;

		const argsStart = parser.getPosition();
		const openContext = expectOpening(
			parser,
			tt.parenL,
			tt.parenR,
			"call arguments",
		);
		const callee = base;

		const oldCommaAfterSpreadAt = parser.state.commaAfterSpreadAt;
		parser.state.commaAfterSpreadAt = ob1Number0Neg1;

		let {args, params} = parseCallExpressionArguments(
			parser,
			openContext,
			maybeAsyncArrow,
		);

		if (maybeAsyncArrow && shouldParseAsyncArrow(parser)) {
			state.stop = true;

			checkCommaAfterRestFromSpread(parser);

			const node = parseAsyncArrowFromCallExpression(
				parser,
				startPos,
				params === undefined ? args : params,
			);
			checkYieldAwaitInDefaultParams(parser);
			parser.state.yieldPos = oldYieldPos;
			parser.state.awaitPos = oldAwaitPos;
			return node;
		} else {
			args = toReferencedListDeep(parser, args);

			// We keep the old value if it isn't null, for cases like

			//   (x = async(yield)) => {}
			parser.state.yieldPos = oldYieldPos || parser.state.yieldPos;
			parser.state.awaitPos = oldAwaitPos || parser.state.awaitPos;
		}

		parser.state.maybeInArrowParameters = oldMaybeInArrowParameters;
		parser.state.commaAfterSpreadAt = oldCommaAfterSpreadAt;

		return parser.finishNodeWithStarts(
			[argsStart, startPos],
			{
				type: state.optionalChainMember
					? "JSOptionalCallExpression"
					: "JSCallExpression",
				callee,
				arguments: args,
			},
		);
	}

	if (match(parser, tt.backQuote)) {
		return parseTaggedTemplateExpression(parser, startPos, base, state);
	}

	state.stop = true;
	return base;
}

export function parseTaggedTemplateExpression(
	parser: JSParser,
	startPos: Position,
	tag: AnyJSExpression,
	state: ParseSubscriptState,
	typeArguments?: TSTypeParameterInstantiation,
): JSTaggedTemplateExpression {
	if (state.optionalChainMember) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.TAGGED_TEMPLATE_IN_OPTIONAL_CHAIN,
			},
		);
	}

	const quasi = parseTemplate(parser, true);
	return parser.finishNode(
		startPos,
		{
			type: "JSTaggedTemplateExpression",
			tag,
			quasi,
			typeArguments,
		},
	);
}

export function checkYieldAwaitInDefaultParams(parser: JSParser) {
	if (
		ob1Get0(parser.state.yieldPos) > 0 &&
		(parser.state.awaitPos === ob1Number0 ||
		parser.state.yieldPos < parser.state.awaitPos)
	) {
		unexpectedDiagnostic(
			parser,
			{
				index: parser.state.yieldPos,
				description: descriptions.JS_PARSER.YIELD_IN_GENERATOR_PARAMS,
			},
		);
	}

	if (ob1Get0(parser.state.awaitPos) > 0) {
		unexpectedDiagnostic(
			parser,
			{
				index: parser.state.awaitPos,
				description: descriptions.JS_PARSER.AWAIT_IN_ASYNC_PARAMS,
			},
		);
	}
}

export function atPossibleAsync(
	parser: JSParser,
	base: AnyJSExpression,
): boolean {
	const start = parser.getInputStartIndex(base);
	const end = parser.getInputEndIndex(base);
	return (
		base.type === "JSReferenceIdentifier" &&
		base.name === "async" &&
		parser.state.lastEndIndex === end &&
		!canInsertSemicolon(parser) &&
		parser.getRawInput(start, end) === "async"
	);
}

export function parseCallExpressionArguments(
	parser: JSParser,
	openContext: OpeningContext,
	possibleAsyncArrow: boolean,
	refTrailingCommaPos?: IndexTracker,
): {
	args: JSCallExpression["arguments"];
	params:
		| undefined
		| Array<
				AnyJSExpression | JSSpreadElement | JSAmbiguousFlowTypeCastExpression
			>;
} {
	let callArgs: JSCallExpression["arguments"] = [];
	let funcParams: Array<
		AnyJSExpression | JSSpreadElement | JSAmbiguousFlowTypeCastExpression
	> = [];

	let innerParenStart;
	let first = true;

	let forceAsyncArrow = false;

	while (true) {
		if (match(parser, openContext.close) || match(parser, tt.eof)) {
			expectClosing(parser, openContext);
			break;
		}

		if (first) {
			first = false;
		} else {
			if (!expect(parser, tt.comma)) {
				break;
			}

			if (eat(parser, openContext.close)) {
				break;
			}
		}

		// we need to make sure that if this is an async arrow functions, that we don't allow inner parens inside the params
		if (match(parser, tt.parenL) && !innerParenStart) {
			innerParenStart = parser.state.startPos;
		}

		const elt = parseCallArgument(
			parser,
			"call expression argument",
			false,
			possibleAsyncArrow ? createIndexTracker() : undefined,
			possibleAsyncArrow ? createIndexTracker() : undefined,
			possibleAsyncArrow ? refTrailingCommaPos : undefined,
		);
		if (elt.type === "JSArrayHole") {
			throw new Error("Expected element");
		}

		if (elt.type === "JSAmbiguousFlowTypeCastExpression") {
			if (possibleAsyncArrow) {
				// Definitely needs to be an arrow
				forceAsyncArrow = true;

				if (callArgs.length > 0) {
					funcParams = callArgs.slice();
					callArgs = [];
				}

				funcParams.push(elt);
			} else {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.CONFUSING_CALL_ARGUMENT,
						loc: elt.loc,
					},
				);
			}
			continue;
		}

		if (funcParams.length > 0) {
			funcParams.push(elt);
		} else {
			callArgs.push(elt);
		}
	}

	if (forceAsyncArrow && !shouldParseAsyncArrow(parser)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.EXPECTED_ARROW_AFTER_ASYNC_TYPE_PARAMS,
			},
		);
	}

	// we found an async arrow function so let's not allow any inner parens
	if (
		possibleAsyncArrow &&
		innerParenStart !== undefined &&
		shouldParseAsyncArrow(parser)
	) {
		unexpectedDiagnostic(
			parser,
			{
				start: innerParenStart,
				description: descriptions.JS_PARSER.PARENTHESIZED_FUNCTION_PARAMS,
			},
		);
	}

	return {
		args: callArgs,
		params: funcParams.length === 0 ? undefined : funcParams,
	};
}

export function shouldParseAsyncArrow(parser: JSParser): boolean {
	return (
		match(parser, tt.colon) ||
		(match(parser, tt.arrow) && !canInsertSemicolon(parser))
	);
}

export function parseAsyncArrowFromCallExpression(
	parser: JSParser,
	start: Position,
	args: Array<
		AnyJSExpression | JSSpreadElement | JSAmbiguousFlowTypeCastExpression
	>,
): JSArrowFunctionExpression {
	let returnType;

	if (match(parser, tt.colon)) {
		const oldNoAnonFunctionType = parser.state.noAnonFunctionType;
		parser.state.noAnonFunctionType = true;
		returnType = parseTSTypeAnnotation(parser, true);
		parser.state.noAnonFunctionType = oldNoAnonFunctionType;
	}

	const oldYield = parser.state.yieldInPossibleArrowParameters;
	parser.state.yieldInPossibleArrowParameters = undefined;
	expect(parser, tt.arrow);
	const node = parseArrowExpression(
		parser,
		start,
		{
			assignmentList: args,
		},
		true,
	);
	parser.state.yieldInPossibleArrowParameters = oldYield;
	return {
		...node,
		head: {
			...node.head,
			returnType,
		},
	};
}

// Parse a no-call expression (like argument of `new` or `::` operators).
export function parseNoCallExpr(
	parser: JSParser,
	context: ExpressionContext,
): AnyJSExpression {
	const startPos = parser.state.startPos;
	return parseSubscripts(
		parser,
		parseExpressionAtom(parser, context),
		startPos,
		true,
	);
}

type ExpressionContext =
	| "await argument"
	| "export default declaration"
	| "export from"
	| "import source"
	| "return argument"
	| "switch discriminant"
	| "case test"
	| "throw argument"
	| "flow object property key"
	| "flow declare module id"
	| "flow declared predicate"
	| "class private property"
	| "class property value"
	| "assignment right"
	| "class heritage"
	| "new callee"
	| "var init"
	| "for right"
	| "for update"
	| "for test"
	| "for init"
	| "with object"
	| "while test"
	| "do test"
	| "if test"
	| "conditional consequent"
	| "conditional alternate"
	| "statement expression"
	| "class private property value"
	| "optional member expression property"
	| "member expression computed property"
	| "call expression argument"
	| "new expression argument"
	| "template expression value"
	| "object property value"
	| "property name"
	| "function body"
	| "yield argument"
	| "array element"
	| "spread argument"
	| "assignment pattern right"
	| "ts export assignment"
	| "ts external module reference expression"
	| "ts enum member initializer"
	| "ts enum member id"
	| "ts type assertion"
	| "ts literal type"
	| "ts import argument"
	| "jsx inner expression container"
	| "jsx attribute value"
	| "jsx spread child expression"
	| "jsx attribute spread"
	| "jsx text";

export function parseExpressionAtom(
	parser: JSParser,
	context: ExpressionContext,
	refShorthandDefaultPos?: IndexTracker,
): AnyJSExpression {
	// If a division operator appears in an expression position, the
	// tokenizer got confused, and we force it to read a regexp instead.
	if (parser.state.tokenType === tt.slash) {
		readRegexp(parser);
	}

	const canBeArrow = parser.state.potentialArrowAt === parser.getIndex();

	// We don't want to match <! as it's the start of a HTML comment
	if (
		isRelational(parser, "<") &&
		parser.input.charCodeAt(ob1Get0(parser.state.index)) !==
		charCodes.exclamationMark
	) {
		// In case we encounter an lt token here it will always be the start of
		// jsx as the lt sign is not allowed in places that expect an expression
		finishToken(parser, tt.jsxTagStart);
		return parseJSXElement(parser);
	}

	switch (parser.state.tokenType) {
		case tt.jsxTagStart:
			return parseJSXElement(parser);

		case tt._super:
			return parseSuper(parser);

		case tt._import:
			return parseImportOrMetaProperty(parser);

		case tt._this: {
			const start = parser.getPosition();
			next(parser);
			return parser.finishNode(start, {type: "JSThisExpression"});
		}

		case tt.name: {
			const start = parser.getPosition();
			const containsEsc = parser.state.escapePosition !== undefined;
			const id = parseIdentifier(parser);

			if (
				!containsEsc &&
				id.name === "async" &&
				match(parser, tt._function) &&
				!canInsertSemicolon(parser)
			) {
				next(parser);
				return parseFunctionExpression(parser, start, true);
			}

			if (
				canBeArrow &&
				!containsEsc &&
				id.name === "async" &&
				match(parser, tt.name)
			) {
				const oldYield = parser.state.yieldInPossibleArrowParameters;
				parser.state.yieldInPossibleArrowParameters = undefined;
				const params = [parseReferenceIdentifier(parser)];
				expect(parser, tt.arrow);
				// let foo = bar => {};
				const node = parseArrowExpression(
					parser,
					start,
					{
						assignmentList: params,
					},
					true,
				);
				parser.state.yieldInPossibleArrowParameters = oldYield;
				return node;
			}

			if (canBeArrow && !canInsertSemicolon(parser) && eat(parser, tt.arrow)) {
				const oldYield = parser.state.yieldInPossibleArrowParameters;
				parser.state.yieldInPossibleArrowParameters = undefined;
				const node = parseArrowExpression(
					parser,
					start,
					{
						assignmentList: [toReferenceIdentifier(parser, id)],
					},
				);
				parser.state.yieldInPossibleArrowParameters = oldYield;
				return node;
			}

			return toReferenceIdentifier(parser, id);
		}

		case tt._do:
			return parseDoExpression(parser);

		case tt.regexp:
			return parseRegExpLiteral(parser);

		case tt.num:
			return parseNumericLiteral(parser);

		case tt.bigint:
			return parseBigIntLiteral(parser);

		case tt.string:
			return parseStringLiteral(parser);

		case tt._null:
			return parseNullLiteral(parser);

		case tt._true:
		case tt._false:
			return parseBooleanLiteral(parser);

		case tt.parenL:
			return parseParenAndDistinguishExpression(parser, context, canBeArrow);

		case tt.bracketL:
			return parseArrayExpression(parser, refShorthandDefaultPos);

		case tt.braceL:
			return parseObjectExpression(parser, refShorthandDefaultPos);

		case tt._function:
			return parseFunctionExpressionOrMetaProperty(parser);

		case tt._class: {
			const start = parser.getPosition();
			return parseClassExpression(parser, start);
		}

		case tt._new:
			return parseNew(parser);

		case tt.backQuote:
			return parseTemplate(parser, false);

		default: {
			const start = parser.getPosition();
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.UNKNOWN_EXPRESSION_ATOM_START(
						context,
					),
				},
			);
			next(parser);
			return toReferenceIdentifier(
				parser,
				createUnknownIdentifier(parser, context, start),
			);
		}
	}
}

export function parseBooleanLiteral(parser: JSParser): JSBooleanLiteral {
	const start = parser.getPosition();
	const value = match(parser, tt._true);
	next(parser);
	return parser.finishNode(
		start,
		{
			type: "JSBooleanLiteral",
			value,
		},
	);
}

export function parseMaybePrivateName(
	parser: JSParser,
): JSPrivateName | JSIdentifier {
	const isPrivate = match(parser, tt.hash);

	if (isPrivate) {
		const start = parser.getPosition();
		next(parser);
		assertNoSpace(parser, descriptions.JS_PARSER.SPACE_BETWEEN_PRIVATE_HASH);
		const id = parseIdentifier(parser, true);
		return parser.finishNode(
			start,
			{
				type: "JSPrivateName",
				id,
			},
		);
	} else {
		return parseIdentifier(parser, true);
	}
}

export function parseFunctionExpressionOrMetaProperty(
	parser: JSParser,
): JSFunctionExpression | JSMetaProperty {
	const start = parser.getPosition();
	next(parser);

	// We do not do parseIdentifier here because when parseFunctionExpressionOrMetaProperty

	// is called we already know that the current token is a "name" with the value "function"

	// This will improve perf a tiny little bit as we do not do validation but more importantly

	// here is that parseIdentifier will remove an item from the expression stack

	// if "function" or "class" is parsed as identifier (in objects e.g.), which should not happen here.
	const meta = createIdentifier(parser, start, "function");

	if (inScope(parser, "GENERATOR") && eat(parser, tt.dot)) {
		return parseMetaProperty(parser, start, meta, "sent");
	}

	const node = parseFunctionExpression(parser, start, false);

	if (node.type !== "JSFunctionExpression") {
		throw new Error("Expected parseFunction to return a JSFunctionExpression");
	}

	return node;
}

export function parseMetaProperty(
	parser: JSParser,
	start: Position,
	meta: JSIdentifier,
	propertyName: string,
): JSMetaProperty {
	if (
		meta.name === "function" &&
		propertyName === "sent" &&
		!isContextual(parser, propertyName)
	) {
		// They didn't actually say `function.sent`, just `function.`, so a simple error would be less confusing.
		unexpectedToken(parser);
	}

	const escapePosition = parser.state.escapePosition;
	const property = parseIdentifier(parser, true);

	if (property.name === propertyName) {
		banUnicodeEscape(parser, escapePosition, propertyName);
	} else {
		unexpectedDiagnostic(
			parser,
			{
				loc: property.loc,
				description: descriptions.JS_PARSER.INVALID_META_PROPERTY(
					meta.name,
					propertyName,
				),
			},
		);
	}

	return parser.finishNode(
		start,
		{
			type: "JSMetaProperty",
			meta,
			property,
		},
	);
}

export function parseImportMetaProperty(parser: JSParser): JSMetaProperty {
	const start = parser.getPosition();
	const id = parseIdentifier(parser, true);
	expect(parser, tt.dot);
	const node = parseMetaProperty(parser, start, id, "meta");

	if (!parser.meta.inModule) {
		unexpectedDiagnostic(
			parser,
			{
				loc: node.loc,
				description: descriptions.JS_PARSER.IMPORT_META_OUTSIDE_MODULE,
			},
		);
	}

	return node;
}

export function parseParenExpression(
	parser: JSParser,
	context: ExpressionContext,
): AnyJSExpression {
	const openContext = expectOpening(parser, tt.parenL, tt.parenR, context);
	const val = parseExpression(parser, context);
	expectClosing(parser, openContext);
	return val;
}

export function parseParenAndDistinguishExpression(
	parser: JSParser,
	context: ExpressionContext,
	canBeArrow: boolean,
): AnyJSExpression {
	const startPos = parser.getPosition();
	const startIndex = parser.getIndex();

	if (parser.state.noArrowAt.includes(startIndex)) {
		canBeArrow = false;
	}

	const openContext = expectOpening(
		parser,
		tt.parenL,
		tt.parenR,
		"paren expression",
	);

	const oldMaybeInArrowParameters = parser.state.maybeInArrowParameters;
	const oldYieldPos = parser.state.yieldPos;
	const oldAwaitPos = parser.state.awaitPos;
	const oldYield = parser.state.yieldInPossibleArrowParameters;
	parser.state.maybeInArrowParameters = true;
	parser.state.yieldInPossibleArrowParameters = undefined;
	parser.state.yieldPos = ob1Number0;
	parser.state.awaitPos = ob1Number0;

	const innerStart = parser.getPosition();
	const exprList: Array<ToReferencedItem> = [];
	const refShorthandDefaultPos: IndexTracker = createIndexTracker();
	const refNeedsArrowPos: IndexTracker = createIndexTracker();
	let first = true;
	let spreadStart;
	let optionalCommaStart;

	while (!match(parser, tt.parenR)) {
		if (first) {
			first = false;
		} else {
			if (
				!expect(
					parser,
					tt.comma,
					refNeedsArrowPos.index === ob1Number0
						? undefined
						: parser.getPositionFromIndex(refNeedsArrowPos.index),
				)
			) {
				break;
			}

			if (match(parser, tt.parenR)) {
				optionalCommaStart = parser.state.startPos;
				break;
			}
		}

		if (match(parser, tt.ellipsis)) {
			const spreadNodeStartPos = parser.state.startPos;
			spreadStart = parser.state.startPos;
			exprList.push(
				parseParenItem(parser, parseSpread(parser), spreadNodeStartPos),
			);

			if (
				match(parser, tt.comma) &&
				lookaheadState(parser).tokenType === tt.parenR
			) {
				raiseRestNotLast(parser);
				eat(parser, tt.comma);
			}
		} else {
			exprList.push(
				parseMaybeAssign<ReturnType<typeof parseParenItem>>(
					parser,
					context,
					false,
					refShorthandDefaultPos,
					parseParenItem,
					refNeedsArrowPos,
				),
			);
		}
	}

	const innerEnd = parser.getPosition();
	expectClosing(parser, openContext);

	parser.state.maybeInArrowParameters = oldMaybeInArrowParameters;

	if (canBeArrow && shouldParseArrow(parser)) {
		const {valid, returnType} = parseArrowHead(parser);

		if (valid) {
			checkYieldAwaitInDefaultParams(parser);
			parser.state.yieldPos = oldYieldPos;
			parser.state.awaitPos = oldAwaitPos;

			for (const param of exprList) {
				if (isParenthesized(parser, param)) {
					unexpectedDiagnostic(
						parser,
						{
							loc: param.loc,
							description: descriptions.JS_PARSER.PARENTHESIZED_FUNCTION_PARAMS,
						},
					);
				}
			}

			const arrow = parseArrowExpression(
				parser,
				startPos,
				{
					assignmentList: exprList,
				},
			);
			parser.state.yieldInPossibleArrowParameters = oldYield;
			return {
				...arrow,
				head: {
					...arrow.head,
					returnType,
				},
			};
		}
	}

	parser.state.yieldInPossibleArrowParameters = oldYield;

	// We keep the old value if it isn't null, for cases like

	//   (x = (yield)) => {}
	parser.state.yieldPos = oldYieldPos || parser.state.yieldPos;
	parser.state.awaitPos = oldAwaitPos || parser.state.awaitPos;

	if (exprList.length === 0) {
		unexpectedDiagnostic(
			parser,
			{
				start: innerStart,
				end: innerEnd,
				description: descriptions.JS_PARSER.EMPTY_PARENTHESIZED_EXPRESSION,
			},
		);

		exprList.push(
			toReferenceIdentifier(
				parser,
				createUnknownIdentifier(
					parser,
					"empty parenthesized expression",
					innerStart,
					innerEnd,
				),
			),
		);
	}

	if (optionalCommaStart !== undefined) {
		unexpectedToken(parser, optionalCommaStart);
	}

	if (spreadStart !== undefined) {
		unexpectedToken(parser, spreadStart);
	}

	if (ob1Get0(refShorthandDefaultPos.index) > 0) {
		unexpectedToken(
			parser,
			parser.getPositionFromIndex(refShorthandDefaultPos.index),
		);
	}

	if (ob1Get0(refNeedsArrowPos.index) > 0) {
		unexpectedToken(parser, parser.getPositionFromIndex(refNeedsArrowPos.index));
	}

	const filterList = filterSpread(
		parser,
		toReferencedListDeep(parser, exprList, /* isParenthesizedExpr */ true),
	);

	let val: AnyJSExpression = filterList[0];
	if (filterList.length > 1) {
		val = parser.finishNodeAt(
			innerStart,
			innerEnd,
			{
				type: "JSSequenceExpression",
				expressions: filterList,
			},
		);
	}

	addParenthesized(parser, val);

	return val;
}

export function shouldParseArrow(parser: JSParser): boolean {
	return match(parser, tt.colon) || !canInsertSemicolon(parser);
}

export function parseArrowHead(
	parser: JSParser,
): {
	valid: boolean;
	returnType: undefined | AnyTSPrimary;
} {
	if (match(parser, tt.colon)) {
		const oldNoAnonFunctionType = parser.state.noAnonFunctionType;
		parser.state.noAnonFunctionType = true;

		const branch = createBranch<undefined | AnyTSPrimary>(parser);

		branch.add(() => {
			const res = parseTSTypeOrTypePredicateAnnotation(parser, tt.colon);

			if (canInsertSemicolon(parser)) {
				// No semicolon insertion expected
				return undefined;
			}

			if (eat(parser, tt.arrow)) {
				return res;
			}

			return undefined;
		});

		if (branch.hasBranch()) {
			const returnType = branch.pick();
			parser.state.noAnonFunctionType = oldNoAnonFunctionType;

			if (returnType === undefined) {
				throw new Error(
					"hasBranchResult call above should have refined this condition",
				);
			}

			return {
				valid: true,
				returnType,
			};
		} else {
			parser.state.noAnonFunctionType = oldNoAnonFunctionType;
			return {
				valid: false,
				returnType: undefined,
			};
		}
	} else {
		return {
			valid: eat(parser, tt.arrow),
			returnType: undefined,
		};
	}
}

// Parse a possible function param or call argument
export function parseParenItem(
	parser: JSParser,
	node: AnyJSExpression | JSSpreadElement,
	startPos: Position,
): ToReferencedItem {
	let optional: undefined | boolean = undefined;
	if (eat(parser, tt.question)) {
		optional = true;
	}

	if (match(parser, tt.colon)) {
		const typeAnnotation = parseTSTypeAnnotation(parser, true);
		return parser.finishNode(
			startPos,
			{
				type: "JSAmbiguousFlowTypeCastExpression",
				expression: node,
				typeAnnotation,
				optional,
			},
		);
	}

	if (optional) {
		return parser.finishNode(
			startPos,
			{
				type: "JSAmbiguousFlowTypeCastExpression",
				expression: node,
				typeAnnotation: undefined,
				optional,
			},
		);
	}

	return node;
}

export function parseNew(parser: JSParser): JSNewExpression | JSMetaProperty {
	const start = parser.getPosition();
	const meta = parseIdentifier(parser, true);

	if (eat(parser, tt.dot)) {
		const metaProp = parseMetaProperty(parser, start, meta, "target");

		if (
			!inScope(parser, "NON_ARROW_FUNCTION") &&
			!inScope(parser, "CLASS_PROPERTY")
		) {
			unexpectedDiagnostic(
				parser,
				{
					loc: metaProp.loc,
					description: descriptions.JS_PARSER.NEW_TARGET_OUTSIDE_CLASS,
				},
			);
		}

		return metaProp;
	}

	const callee = parseNoCallExpr(parser, "new callee");

	if (callee.type === "JSImportCall") {
		unexpectedDiagnostic(
			parser,
			{
				loc: callee.loc,
				description: descriptions.JS_PARSER.SUPER_OUTSIDE_METHOD,
			},
		);
	}

	const optionalMember = getFirstOptionalChainMember(callee);
	if (optionalMember !== undefined) {
		const memberLoc = parser.getLoc(optionalMember);

		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.NEW_IN_OPTIONAL_CHAIN(memberLoc),
			},
		);
	}

	if (eat(parser, tt.questionDot)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.NEW_IN_OPTIONAL_CHAIN(),
			},
		);
	}

	let optional = undefined;
	if (eat(parser, tt.questionDot)) {
		optional = true;
	}

	let typeArguments = undefined;
	if (isSyntaxEnabled(parser, "ts") && isRelational(parser, "<")) {
		typeArguments = tryBranch(parser, parseTSTypeArguments);
	}

	let args: Array<AnyJSExpression | JSSpreadElement> = [];
	if (match(parser, tt.parenL)) {
		const openContext = expectOpening(
			parser,
			tt.parenL,
			tt.parenR,
			"new argument",
		);
		args = parseExpressionListNonEmpty(
			parser,
			"new expression argument",
			openContext,
		);
		args = toReferencedList(parser, args);
	} else if (isSyntaxEnabled(parser, "ts") && typeArguments !== undefined) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.NEW_WITH_TYPESCRIPT_TYPE_ARGUMENTS_NO_PARENS,
			},
		);
	}

	return parser.finishNode(
		start,
		{
			type: "JSNewExpression",
			callee,
			typeArguments,
			arguments: args,
			optional,
		},
	);
}

function getFirstOptionalChainMember(
	node: AnyNode,
): undefined | JSOptionalCallExpression | JSMemberExpression {
	if (node.type === "JSOptionalCallExpression") {
		return node;
	}

	if (node.type === "JSMemberExpression") {
		if (node.property.optional) {
			return node;
		}

		if (node.property.type === "JSStaticMemberProperty") {
			return getFirstOptionalChainMember(node.object);
		}
	}

	return undefined;
}

// Parse template expression.
export function parseTemplateElement(
	parser: JSParser,
	isTagged: boolean,
): JSTemplateElement {
	const start = parser.getPosition();
	const tokenValue = parser.state.tokenValue;

	if (tokenValue === undefined) {
		if (isTagged) {
			parser.state.invalidTemplateEscapePosition = undefined;
		} else {
			unexpectedDiagnostic(
				parser,
				{
					index: parser.state.invalidTemplateEscapePosition,
					description: descriptions.JS_PARSER.INVALID_TEMPLATE_ESCAPE,
				},
			);
		}
	}

	const raw = parser.getRawInput(parser.state.startIndex, parser.state.endIndex).replace(
		/\r\n?/g,
		"\n",
	);
	const cooked = tokenValue === undefined ? raw : String(tokenValue);

	next(parser);
	const tail = match(parser, tt.backQuote);
	return parser.finishNode(
		start,
		{
			type: "JSTemplateElement",
			raw,
			cooked,
			tail,
		},
	);
}

export function parseTemplate(
	parser: JSParser,
	isTagged: boolean,
): JSTemplateLiteral {
	const start = parser.getPosition();
	const openContext = expectOpening(
		parser,
		tt.backQuote,
		tt.backQuote,
		"template literal",
	);
	const expressions = [];
	let curElt = parseTemplateElement(parser, isTagged);
	const quasis = [curElt];

	while (true) {
		if (match(parser, tt.eof) || curElt.tail === true) {
			break;
		}

		const exprPpenContext = expectOpening(
			parser,
			tt.dollarBraceL,
			tt.braceR,
			"template expression value",
		);
		expressions.push(parseExpression(parser, "template expression value"));
		expectClosing(parser, exprPpenContext);

		curElt = parseTemplateElement(parser, isTagged);
		quasis.push(curElt);
	}

	expectClosing(parser, openContext);

	return parser.finishNode(
		start,
		{
			type: "JSTemplateLiteral",
			expressions,
			quasis,
		},
	);
}

export function parseObjectExpression(
	parser: JSParser,
	refShorthandDefaultPos?: IndexTracker,
): JSObjectExpression {
	const propHash: Set<string> = new Set();
	let first = true;

	const start = parser.getPosition();
	const properties = [];

	const openContext = expectOpening(parser, tt.braceL, tt.braceR, "object");

	while (true) {
		if (match(parser, tt.braceR) || match(parser, tt.eof)) {
			expectClosing(parser, openContext);
			break;
		}

		if (first) {
			first = false;
		} else {
			if (!expect(parser, tt.comma)) {
				break;
			}

			if (eat(parser, tt.braceR)) {
				break;
			}
		}

		if (match(parser, tt.ellipsis)) {
			const prop: JSSpreadProperty = {
				...parseSpread(parser),
				type: "JSSpreadProperty",
			};
			properties.push(prop);
			continue;
		}

		const start = parser.getPosition();
		let isGenerator = eat(parser, tt.star);
		let isAsync = false;

		let key: JSStaticPropertyKey | JSComputedPropertyKey;
		let escapePosition;

		if (isContextual(parser, "async")) {
			if (isGenerator) {
				unexpectedToken(parser);
			}

			const asyncId = parseIdentifier(parser);
			if (
				match(parser, tt.colon) ||
				match(parser, tt.parenL) ||
				match(parser, tt.braceR) ||
				match(parser, tt.eq) ||
				match(parser, tt.comma)
			) {
				key = {
					type: "JSStaticPropertyKey",
					loc: asyncId.loc,
					value: asyncId,
				};
			} else {
				if (hasPrecedingLineBreak(parser)) {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.ASYNC_OBJECT_METHOD_LINE_BREAK,
						},
					);
				}

				isAsync = true;
				if (match(parser, tt.star)) {
					next(parser);
					isGenerator = true;
				}
				escapePosition = parser.state.escapePosition;
				key = parseObjectPropertyKey(parser);
			}
		} else {
			escapePosition = parser.state.escapePosition;
			key = parseObjectPropertyKey(parser);
		}

		const prop = parseObjectPropertyValue(
			parser,
			{
				key,
				start,
				isGenerator,
				isAsync,
				isPattern: false,
				refShorthandDefaultPos,
				escapePosition,
			},
		);
		if (prop === undefined) {
			continue;
		}
		if (prop.type === "JSBindingObjectPatternProperty") {
			throw new Error("Impossible");
		}

		checkPropClash(parser, prop, propHash);
		properties.push(prop);
	}

	return parser.finishNode(
		start,
		{
			type: "JSObjectExpression",
			properties,
		},
	);
}

export function parseObjectPattern(
	parser: JSParser,
	refShorthandDefaultPos?: IndexTracker,
): JSBindingObjectPattern {
	const propHash: Set<string> = new Set();
	let first = true;

	const start = parser.getPosition();
	const properties: Array<JSBindingObjectPatternProperty> = [];
	let rest: undefined | JSBindingIdentifier;

	const openContext = expectOpening(
		parser,
		tt.braceL,
		tt.braceR,
		"object pattern",
	);

	let firstRestLocation = undefined;

	while (true) {
		if (match(parser, tt.eof) || match(parser, tt.braceR)) {
			break;
		}

		if (first) {
			first = false;
		} else {
			expect(parser, tt.comma);

			if (match(parser, tt.eof) || match(parser, tt.braceR)) {
				break;
			}
		}

		let isGenerator = false;
		let isAsync = false;
		let start = parser.getPosition();

		if (eat(parser, tt.ellipsis)) {
			const argument = parseBindingIdentifier(parser);
			rest = argument;

			if (firstRestLocation !== undefined) {
				unexpectedDiagnostic(
					parser,
					{
						loc: argument.loc,
						description: descriptions.JS_PARSER.MULTIPLE_DESTRUCTURING_RESTS,
					},
				);
			}

			if (match(parser, tt.braceR) || match(parser, tt.eof)) {
				break;
			}

			if (
				match(parser, tt.comma) &&
				lookaheadState(parser).tokenType === tt.braceR
			) {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.TRAILING_COMMA_AFTER_REST,
					},
				);
				eat(parser, tt.comma);
				break;
			} else {
				firstRestLocation = argument.loc;
				continue;
			}
		}

		start = parser.getPosition();

		const key = parseObjectPropertyKey(parser);
		const prop = parseObjectPropertyValue(
			parser,
			{
				key,
				start,
				isGenerator,
				isAsync,
				isPattern: true,
				refShorthandDefaultPos,
				escapePosition: undefined,
			},
		);

		if (prop === undefined) {
			continue;
		}

		checkPropClash(parser, prop, propHash);

		if (prop.type !== "JSBindingObjectPatternProperty") {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.INVALID_OBJECT_PATTERN_PROP,
					loc: prop.loc,
				},
			);
			continue;
		}

		properties.push(prop);
	}

	expectClosing(parser, openContext);

	if (firstRestLocation !== undefined) {
		raiseRestNotLast(parser, firstRestLocation);
	}

	return parser.finishNode(
		start,
		{
			type: "JSBindingObjectPattern",
			properties,
			rest,
		},
	);
}

export function isGetterOrSetterMethod(
	parser: JSParser,
	key: JSStaticPropertyKey | JSComputedPropertyKey,
	// `key` is always from `name.key`, we just need it here to refine
	keyVal: JSIdentifier | AnyJSExpression | JSPrivateName,
	isPattern: boolean,
): keyVal is JSIdentifier {
	return (
		!isPattern &&
		key.type === "JSStaticPropertyKey" &&
		keyVal.type === "JSIdentifier" &&
		(keyVal.name === "get" || keyVal.name === "set") &&
		(match(parser, tt.string) ||
		// get "string"() {}
		match(parser, tt.num) ||
		// get 1() {}
		match(parser, tt.bracketL) ||
		// get ["string"]() {}
		match(parser, tt.name) ||
		// get foo() {}
		!!parser.state.tokenType.keyword) // get debugger() {}
	);
}

// get methods aren't allowed to have any parameters
// set methods must have exactly 1 parameter
export function checkGetterSetterParamCount(
	parser: JSParser,
	method:
		| JSObjectMethod
		| JSClassMethod
		| JSClassPrivateMethod
		| TSDeclareFunction
		| TSDeclareMethod,
	kind: string,
): void {
	const head = method.head;

	if (kind === "get") {
		if (head.rest !== undefined || head.params.length !== 0) {
			unexpectedDiagnostic(
				parser,
				{
					loc: method.loc,
					description: descriptions.JS_PARSER.GETTER_WITH_PARAMS,
				},
			);
		}
	} else if (kind === "set") {
		if (head.rest !== undefined) {
			unexpectedDiagnostic(
				parser,
				{
					loc: head.rest.loc,
					description: descriptions.JS_PARSER.SETTER_WITH_REST,
				},
			);
		} else if (head.params.length !== 1) {
			unexpectedDiagnostic(
				parser,
				{
					loc: method.loc,
					description: descriptions.JS_PARSER.SETTER_NOT_ONE_PARAM,
				},
			);
		}
	}
}

type ParseObjectMethodOpts = {
	key: AnyJSObjectPropertyKey;
	start: Position;
	isGenerator: boolean;
	isAsync: boolean;
	isPattern: boolean;
	escapePosition: undefined | Number0;
};

export function parseObjectMethod(
	parser: JSParser,
	{
		key,
		start,
		isGenerator,
		isAsync,
		isPattern,
		escapePosition,
	}: ParseObjectMethodOpts,
): undefined | JSObjectMethod {
	if (isAsync || isGenerator || match(parser, tt.parenL)) {
		if (isPattern) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.OBJECT_METHOD_IN_PATTERN,
				},
			);
		}

		const partial = parseMethod(
			parser,
			{
				kind: "method",
				isClass: false,
				isGenerator,
				isAsync,
				isConstructor: false,
			},
		);

		const {body} = partial;
		if (body === undefined || body.type !== "JSBlockStatement") {
			throw new Error("Expected body");
		}

		return parser.finishNode(
			start,
			{
				...partial,
				body,
				key,
				type: "JSObjectMethod",
				kind: "method",
			},
		);
	}

	if (isGetterOrSetterMethod(parser, key, key.value, isPattern)) {
		if (isAsync) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.ASYNC_GETTER_SETTER,
				},
			);
		}

		if (isGenerator) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.GENERATOR_GETTER_SETTER,
				},
			);
		}

		const kind = key.value.name;
		if (kind !== "get" && kind !== "set") {
			throw new Error(
				"Name should be get or set as we already validated it as such",
			);
		}
		banUnicodeEscape(parser, escapePosition, kind);

		const newKey = parseObjectPropertyKey(parser);

		const partial = parseMethod(
			parser,
			{
				kind,
				isClass: false,
				isGenerator: false,
				isAsync: false,
				isConstructor: false,
			},
		);

		const {body, head} = partial;
		if (body === undefined || body.type !== "JSBlockStatement") {
			throw new Error("Expected body");
		}

		const method: JSObjectMethod = parser.finishNode(
			start,
			{
				head,
				body,
				key: newKey,
				type: "JSObjectMethod",
				kind,
			},
		);
		checkGetterSetterParamCount(parser, method, method.kind);
		return method;
	}

	return undefined;
}

export function parseObjectProperty(
	parser: JSParser,
	key: AnyJSObjectPropertyKey,
	start: Position,
	isPattern: boolean,
	refShorthandDefaultPos: undefined | IndexTracker,
): undefined | JSObjectProperty | JSBindingObjectPatternProperty {
	if (eat(parser, tt.colon)) {
		if (isPattern) {
			const value = parseMaybeDefault(parser);
			return parser.finishNode(
				start,
				{
					key,
					type: "JSBindingObjectPatternProperty",
					value,
				},
			);
		} else {
			const value = parseMaybeAssign(
				parser,
				"object property value",
				false,
				refShorthandDefaultPos,
			);
			return parser.finishNode(
				start,
				{
					key,
					type: "JSObjectProperty",
					value,
				},
			);
		}
	}

	if (key.type === "JSStaticPropertyKey" && key.value.type === "JSIdentifier") {
		checkReservedWord(
			parser,
			key.value.name,
			parser.getLoc(key.value),
			true,
			true,
		);

		if (isPattern) {
			let value: AnyJSBindingPattern = toBindingIdentifier(
				parser,
				cloneNode(parser, key.value),
			);

			if (match(parser, tt.eq) && refShorthandDefaultPos) {
				if (refShorthandDefaultPos.index === ob1Number0) {
					refShorthandDefaultPos.index = parser.getIndex();
				}

				value = parseMaybeDefault(parser, start, value);
			}

			return parser.finishNode(
				start,
				{
					type: "JSBindingObjectPatternProperty",
					key,
					value,
				},
			);
		}

		return parser.finishNode(
			start,
			{
				type: "JSObjectProperty",
				key,
				value: toReferenceIdentifier(parser, cloneNode(parser, key.value)),
			},
		);
	}

	return undefined;
}

type ParseObjectPropValueOpts = {
	key: JSComputedPropertyKey | JSStaticPropertyKey;
	start: Position;
	isGenerator: boolean;
	isAsync: boolean;
	isPattern: boolean;
	refShorthandDefaultPos: undefined | IndexTracker;
	escapePosition: undefined | Number0;
};

export function parseObjectPropertyValue(
	parser: JSParser,
	{
		key,
		start,
		isGenerator,
		isAsync,
		isPattern,
		refShorthandDefaultPos,
		escapePosition,
	}: ParseObjectPropValueOpts,
):
	| undefined
	| JSObjectMethod
	| JSObjectProperty
	| JSBindingObjectPatternProperty {
	// parse type parameters for object method shorthand
	let typeParameters = maybeParseTSTypeParameters(parser);
	if (typeParameters !== undefined && !match(parser, tt.parenL)) {
		unexpectedToken(parser);
	}

	let node:
		| undefined
		| JSObjectMethod
		| JSObjectProperty
		| JSBindingObjectPatternProperty =
		parseObjectMethod(
			parser,
			{
				key,
				start,
				isGenerator,
				isAsync,
				isPattern,
				escapePosition,
			},
		) ||
		parseObjectProperty(parser, key, start, isPattern, refShorthandDefaultPos);

	if (node === undefined) {
		unexpectedToken(parser);
		return undefined;
	}

	if (typeParameters === undefined) {
		return node;
	} else {
		if (
			node.type === "JSObjectProperty" ||
			node.type === "JSBindingObjectPatternProperty"
		) {
			unexpectedDiagnostic(
				parser,
				{
					loc: typeParameters.loc,
					description: descriptions.JS_PARSER.OBJECT_PROPERTY_WITH_TYPE_PARAMETERS,
				},
			);
			return node;
		}

		return {
			...node,
			head: {
				...node.head,
				typeParameters,
			},
		};
	}
}

export function parseObjectPropertyKey(
	parser: JSParser,
): JSStaticPropertyKey | JSComputedPropertyKey {
	const start = parser.getPosition();

	if (match(parser, tt.bracketL)) {
		const openContext = expectOpening(
			parser,
			tt.bracketL,
			tt.bracketR,
			"property name",
		);

		const value = parseMaybeAssign(parser, "property name");
		expectClosing(parser, openContext);
		return parser.finishNode(
			start,
			{
				type: "JSComputedPropertyKey",
				value,
			},
		);
	} else {
		pushScope(parser, "PROPERTY_NAME", true);

		// We check if it's valid for it to be a private name when we push it.
		let value;
		if (match(parser, tt.num)) {
			value = parseNumericLiteral(parser);
		} else if (match(parser, tt.string)) {
			value = parseStringLiteral(parser);
		} else {
			value = parseMaybePrivateName(parser);
		}

		popScope(parser, "PROPERTY_NAME");

		return parser.finishNode(
			start,
			{
				type: "JSStaticPropertyKey",
				value,
			},
		);
	}
}

// Parse object or class method.
export function parseMethod(
	parser: JSParser,
	opts: {
		kind: JSClassMethodKind | JSObjectMethodKind;
		isGenerator: boolean;
		isAsync: boolean;
		isConstructor: boolean;
		isClass: boolean;
	},
): {
	head: JSFunctionHead;
	body: undefined | ParseFunctionBodyReturn["body"];
} {
	const {kind, isClass, isGenerator, isAsync, isConstructor} = opts;

	const oldYieldPos = parser.state.yieldPos;
	const oldAwaitPos = parser.state.awaitPos;
	pushScope(parser, "FUNCTION", true);
	pushScope(parser, "NON_ARROW_FUNCTION");
	pushScope(parser, "METHOD", kind);
	pushScope(parser, "GENERATOR", isGenerator);
	parser.state.yieldPos = ob1Number0;
	parser.state.awaitPos = ob1Number0;

	const allowTSModifiers = isConstructor;
	const headStart = parser.getPosition();
	const {typeParameters, rest, params} = parseFunctionParams(
		parser,
		kind,
		allowTSModifiers,
	);
	const start = parser.getPosition();
	const {body, head} = parseFunctionBodyAndFinish(
		parser,
		{
			headStart,
			rest,
			params,
			id: undefined,
			allowBodiless: isClass,
			isArrowFunction: false,
			isAsync,
			isGenerator,
			isMethod: true,
			start,
		},
	);

	popScope(parser, "METHOD");
	popScope(parser, "GENERATOR");
	popScope(parser, "FUNCTION");
	popScope(parser, "NON_ARROW_FUNCTION");
	parser.state.yieldPos = oldYieldPos;
	parser.state.awaitPos = oldAwaitPos;

	return {
		head: {
			...head,
			typeParameters,
		},
		body,
	};
}

function createFunctionHead(
	parser: JSParser,
	params: Array<AnyJSBindingPattern>,
	rest: undefined | AnyJSTargetBindingPattern,
	opts: Partial<JSFunctionHead>,
): JSFunctionHead {
	const nonRestParams: JSFunctionHead["params"] = [];

	for (const param of params) {
		switch (param.type) {
			case "JSBindingIdentifier":
			case "JSBindingAssignmentPattern":
			case "JSBindingObjectPattern":
			case "JSBindingArrayPattern": {
				nonRestParams.push(param);
				break;
			}

			default:
				throw new Error("TODO");
		}
	}

	return {
		type: "JSFunctionHead",
		rest,
		...splitFunctionParams(nonRestParams),
		...opts,
	};
}

export function parseArrowExpression(
	parser: JSParser,
	start: Position,
	opts: {
		bindingList?: Array<AnyJSBindingPattern>;
		assignmentList?: Array<JSArrayHole | ToReferencedItem>;
		rest?: AnyJSTargetBindingPattern;
	},
	isAsync: boolean = false,
): JSArrowFunctionExpression {
	// if we got there, it's no more "yield in possible arrow parameters";
	// it's just "yield in arrow parameters"
	if (parser.state.yieldInPossibleArrowParameters) {
		unexpectedDiagnostic(
			parser,
			{
				start: parser.state.yieldInPossibleArrowParameters,
				description: descriptions.JS_PARSER.YIELD_NAME_IN_GENERATOR,
			},
		);
	}

	pushScope(parser, "FUNCTION", true);

	const oldYieldPos = parser.state.yieldPos;
	const oldAwaitPos = parser.state.awaitPos;
	const oldMaybeInArrowParameters = parser.state.maybeInArrowParameters;
	pushScope(parser, "GENERATOR", false);
	parser.state.maybeInArrowParameters = false;
	parser.state.yieldPos = ob1Number0;
	parser.state.awaitPos = ob1Number0;

	const headEnd = parser.getLastEndPosition();

	let params: Array<AnyJSBindingPattern> = [];
	let rest: undefined | AnyJSTargetBindingPattern = opts.rest;

	if (opts.bindingList !== undefined) {
		params = opts.bindingList;
	}

	if (opts.assignmentList !== undefined) {
		({params, rest} = toFunctionParamsBindingList(
			parser,
			opts.assignmentList,
			"arrow function parameters",
		));
	}

	let head = parser.finishNodeAt(
		start,
		headEnd,
		createFunctionHead(
			parser,
			params,
			rest,
			{
				hasHoistedVars: false,
				async: isAsync,
			},
		),
	);

	const {body, hasHoistedVars} = parseFunctionBody(
		parser,
		{
			id: undefined,
			allowBodiless: false,
			isArrowFunction: true,
			isMethod: false,
			isAsync,
			isGenerator: false,
			start,
		},
	);

	head = {
		...head,
		hasHoistedVars,
	};

	checkFunctionNameAndParams(
		parser,
		{
			isArrowFunction: true,
			isMethod: false,
			id: undefined,
			params,
			rest,
			start,
		},
		body,
	);

	popScope(parser, "GENERATOR");
	popScope(parser, "FUNCTION");
	parser.state.maybeInArrowParameters = oldMaybeInArrowParameters;
	parser.state.yieldPos = oldYieldPos;
	parser.state.awaitPos = oldAwaitPos;

	// Finish the head again so it's added to the comment stack again so that the arrow
	// finishNode can take comments if necessary
	head = parser.finishNodeAt(start, headEnd, head);

	return parser.finishNode(
		start,
		{
			type: "JSArrowFunctionExpression",
			body,
			head,
		},
	);
}

export function isStrictBody(parser: JSParser, body: AnyNode): boolean {
	if (body.type === "JSBlockStatement" && body.directives !== undefined) {
		for (const directive of body.directives) {
			if (directive.value === "use strict") {
				return true;
			}
		}
	}

	return false;
}

type FunctionBodyParseOpts = {
	allowBodiless: boolean;
	isArrowFunction: boolean;
	isAsync: boolean;
	isGenerator: boolean;
	isMethod: boolean;
	start: Position;
	id: JSBindingIdentifier | undefined;
};

export function parseFunctionBodyAndFinish(
	parser: JSParser,
	opts: CheckFunctionNameParamsOpts &
		FunctionBodyParseOpts & {
			headStart: Position;
		},
): {
	head: JSFunctionHead;
	body: undefined | ParseFunctionBodyReturn["body"];
} {
	let returnType = undefined;

	// For arrow functions, `parseArrow` handles the return type itself.
	if (!opts.isArrowFunction && match(parser, tt.colon)) {
		returnType = parseTSTypeOrTypePredicateAnnotation(parser, tt.colon);
	}

	const headEnd = parser.getLastEndPosition();
	let head = parser.finishNodeAt(
		opts.headStart,
		headEnd,
		createFunctionHead(
			parser,
			opts.params,
			opts.rest,
			{
				generator: opts.isGenerator,
				async: opts.isAsync,
				hasHoistedVars: false,
				returnType,
			},
		),
	);

	if (
		opts.allowBodiless &&
		!match(parser, tt.braceL) &&
		isLineTerminator(parser)
	) {
		return {
			head,
			body: undefined,
		};
	}

	const {body, hasHoistedVars} = parseFunctionBody(parser, opts);

	checkFunctionNameAndParams(
		parser,
		{
			isArrowFunction: opts.isArrowFunction,
			isMethod: opts.isMethod,
			id: opts.id,
			start: opts.start,
			params: opts.params,
			rest: opts.rest,
		},
		body,
	);

	head = {
		...head,
		hasHoistedVars,
	};

	return {
		head,
		body,
	};
}

type ParseFunctionBodyReturn = {
	body: JSBlockStatement | AnyJSExpression;
	hasHoistedVars: boolean;
};

export function parseFunctionBody(
	parser: JSParser,
	opts: FunctionBodyParseOpts,
): ParseFunctionBodyReturn {
	if (opts.isArrowFunction) {
		return forwardNoArrowParamsConversionAt(
			parser,
			opts.start,
			() => _parseFunctionBody(parser, opts),
		);
	} else {
		return _parseFunctionBody(parser, opts);
	}
}

// Parse function body and check parameters.
function _parseFunctionBody(
	parser: JSParser,
	opts: FunctionBodyParseOpts,
): ParseFunctionBodyReturn {
	const {isArrowFunction, isAsync, isGenerator} = opts;

	const isExpression = isArrowFunction && !match(parser, tt.braceL);

	pushScope(parser, "PARAMETERS", false);
	pushScope(parser, "ASYNC", isAsync);

	let hasHoistedVars = false;
	let body: AnyJSExpression | JSBlockStatement;
	if (isExpression) {
		body = parseMaybeAssign(parser, "function body");
	} else {
		// Start a new scope with regard to labels and the `inGenerator`
		// flag (restore them to their old value afterwards).
		const oldLabels = parser.state.labels;
		pushScope(parser, "GENERATOR", isGenerator);
		parser.state.labels = [];

		const oldhasHoistedVars = parser.state.hasHoistedVars;
		parser.state.hasHoistedVars = false;

		body = parseBlock(parser, true);
		hasHoistedVars = parser.state.hasHoistedVars;

		popScope(parser, "GENERATOR");

		parser.state.hasHoistedVars = oldhasHoistedVars;
		parser.state.labels = oldLabels;
	}

	popScope(parser, "ASYNC");
	popScope(parser, "PARAMETERS");

	return {body, hasHoistedVars};
}

type CheckFunctionNameParamsOpts = {
	isArrowFunction: boolean;
	isMethod: boolean;
	id: undefined | JSBindingIdentifier;
	params: Array<AnyJSBindingPattern>;
	rest: undefined | AnyJSTargetBindingPattern;
	start: Position;
};

export function checkFunctionNameAndParams(
	parser: JSParser,
	opts: CheckFunctionNameParamsOpts,
	body: AnyJSExpression | JSBlockStatement,
	force?: boolean,
): void {
	const {isArrowFunction, isMethod, id, rest, start, params} = opts;

	if (
		!isSimpleParamList(params, rest) &&
		body.type === "JSBlockStatement" &&
		body.directives !== undefined
	) {
		const firstDirective = body.directives[0];
		if (firstDirective !== undefined && firstDirective.value === "use strict") {
			unexpectedDiagnostic(
				parser,
				{
					loc: firstDirective.loc,
					description: descriptions.JS_PARSER.STRICT_DIRECTIVE_IN_NON_SIMPLE_PARAMS,
				},
			);
		}
	}

	const startIndex = parser.getIndexFromPosition(start, parser.filename);
	if (
		isArrowFunction &&
		force !== true &&
		parser.state.noArrowParamsConversionAt.includes(startIndex)
	) {
		return undefined;
	}

	// If this is a strict mode function, verify that argument names
	// are not repeated, and it does not try to bind the words `eval`
	const _isStrictBody = isStrictBody(parser, body);
	const isStrict = inScope(parser, "STRICT") || _isStrictBody;

	const isSimpleParams = isSimpleParamList(params, rest);
	const shouldCheckLVal: boolean =
		isStrict || isArrowFunction || isMethod || !isSimpleParams;

	pushScope(parser, "STRICT", isStrict);

	if (shouldCheckLVal) {
		const clashes: Map<string, AnyNode> = new Map();

		if (id !== undefined) {
			checkLVal(parser, id, true, undefined, "function name");
		}

		for (const param of params) {
			if (_isStrictBody && param.type !== "JSBindingIdentifier") {
				unexpectedDiagnostic(
					parser,
					{
						loc: param.loc,
						description: descriptions.JS_PARSER.NON_SIMPLE_PARAM_IN_EXPLICIT_STRICT_FUNCTION,
					},
				);
			}
			checkLVal(parser, param, true, clashes, "function parameter list");
		}
	}

	popScope(parser, "STRICT");
}

function isSimpleParamList(
	params: Array<AnyJSBindingPattern>,
	rest: undefined | AnyJSTargetBindingPattern,
): boolean {
	if (rest !== undefined) {
		return false;
	}

	for (const param of params) {
		if (param.type !== "JSBindingIdentifier") {
			return false;
		}
	}

	return true;
}

export function parseExpressionList(
	parser: JSParser,
	context: ExpressionContext,
	openContext: OpeningContext,
	allowEmpty?: boolean,
	refShorthandDefaultPos?: IndexTracker,
): Array<ReturnType<typeof parseCallArgument>> {
	const elts = [];
	let first = true;

	while (true) {
		if (match(parser, openContext.close) || match(parser, tt.eof)) {
			break;
		}

		if (first) {
			first = false;
		} else {
			expect(parser, tt.comma);

			if (match(parser, openContext.close)) {
				break;
			}
		}

		elts.push(
			parseCallArgument(parser, context, allowEmpty, refShorthandDefaultPos),
		);
	}

	expectClosing(parser, openContext);

	return elts;
}

export function parseExpressionListNonEmpty(
	parser: JSParser,
	context: ExpressionContext,
	openContext: OpeningContext,
	refShorthandDefaultPos?: IndexTracker,
): Array<AnyJSExpression> {
	const val = parseExpressionList(
		parser,
		context,
		openContext,
		false,
		refShorthandDefaultPos,
	);
	// @ts-ignore: Passed allowEmpty: false above
	return val;
}

export function parseCallArgument(
	parser: JSParser,
	context: ExpressionContext,
	allowHoles: boolean = false,
	refShorthandDefaultPos?: IndexTracker,
	refNeedsArrowPos?: IndexTracker,
	refTrailingCommaPos?: IndexTracker,
):
	| JSArrayHole
	| AnyJSExpression
	| JSSpreadElement
	| JSAmbiguousFlowTypeCastExpression {
	if (allowHoles && match(parser, tt.comma)) {
		return parseArrayHole(parser);
	} else if (match(parser, tt.ellipsis)) {
		const spreadNodeStart = parser.state.startPos;

		const elt = parseParenItem(
			parser,
			parseSpread(parser, refShorthandDefaultPos, refNeedsArrowPos),
			spreadNodeStart,
		);

		if (refTrailingCommaPos && match(parser, tt.comma)) {
			refTrailingCommaPos.index = parser.getIndex();
		}

		return elt;
	} else {
		return parseMaybeAssign<ReturnType<typeof parseParenItem>>(
			parser,
			context,
			false,
			refShorthandDefaultPos,
			parseParenItem,
			refNeedsArrowPos,
		);
	}
}

// Parse the next token as an identifier. If `liberal` is true (used
// when parsing properties), it will also convert keywords into
// identifiers.
export function parseIdentifier(
	parser: JSParser,
	liberal?: boolean,
): JSIdentifier {
	const start = parser.getPosition();
	const name = parseIdentifierName(parser, liberal);
	return createIdentifier(parser, start, name);
}

export function parseBindingIdentifier(
	parser: JSParser,
	liberal?: boolean,
): JSBindingIdentifier {
	return toBindingIdentifier(parser, parseIdentifier(parser, liberal));
}

export function parseReferenceIdentifier(
	parser: JSParser,
	liberal?: boolean,
): JSReferenceIdentifier {
	return toReferenceIdentifier(parser, parseIdentifier(parser, liberal));
}

export function parseTSConstKeyword(parser: JSParser): TSConstKeyword {
	return toTSConstKeyword(parser, parseIdentifier(parser));
}

export function toTSConstKeyword(
	parser: JSParser,
	node: JSReferenceIdentifier | JSIdentifier | JSBindingIdentifier,
): TSConstKeyword {
	return parser.finalizeNode({
		...node,
		type: "TSConstKeyword",
	});
}

export function toBindingIdentifier(
	parser: JSParser,
	node: JSReferenceIdentifier | JSIdentifier | JSAssignmentIdentifier,
): JSBindingIdentifier {
	return parser.finalizeNode({
		...node,
		type: "JSBindingIdentifier",
	});
}

export function toAssignmentIdentifier(
	parser: JSParser,
	node: JSReferenceIdentifier | JSIdentifier | JSBindingIdentifier,
): JSAssignmentIdentifier {
	return parser.finalizeNode({
		...node,
		type: "JSAssignmentIdentifier",
	});
}

export function toReferenceIdentifier(
	parser: JSParser,
	node: JSBindingIdentifier | JSIdentifier | JSAssignmentIdentifier,
): JSReferenceIdentifier {
	return parser.finalizeNode({
		...node,
		type: "JSReferenceIdentifier",
	});
}

export function toIdentifier(
	parser: JSParser,
	node: JSBindingIdentifier | JSReferenceIdentifier | JSAssignmentIdentifier,
): JSIdentifier {
	return {
		...node,
		type: "JSIdentifier",
	};
}

export function createIdentifier(
	parser: JSParser,
	start: Position,
	name: string,
): JSIdentifier {
	const node: JSIdentifier = parser.finishNode(
		start,
		{
			type: "JSIdentifier",
			name,
		},
	);
	parser.getLoc(node).identifierName = name;
	return node;
}

export function parseIdentifierName(
	parser: JSParser,
	liberal: boolean = false,
): string {
	const loc = parser.finishLocAt(parser.state.startPos, parser.state.endPos);

	if (!liberal) {
		checkReservedWord(
			parser,
			String(parser.state.tokenValue),
			loc,
			!!parser.state.tokenType.keyword,
			false,
		);
	}

	let name: string;

	if (match(parser, tt.name)) {
		name = String(parser.state.tokenValue);
	} else if (parser.state.tokenType.keyword !== undefined) {
		name = parser.state.tokenType.keyword;

		// `class` and `function` keywords push new context into this.context.
		// But there is no chance to pop the context if the keyword is consumed
		// as an identifier such as a property name.
		// If the previous token is a dot, this does not apply because the
		// context-managing code already ignored the keyword
		if (
			(name === "class" || name === "function") &&
			(parser.state.lastEndIndex !== ob1Inc(parser.state.lastStartIndex) ||
			parser.input.charCodeAt(ob1Get0(parser.state.lastStartIndex)) !==
			charCodes.dot)
		) {
			parser.state.context.pop();
		}
	} else {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.EXPECTED_IDENTIFIER,
			},
		);
		name = "";
	}

	if (!liberal) {
		checkReservedWord(
			parser,
			name,
			loc,
			parser.state.tokenType.keyword !== undefined,
			false,
		);
	}

	next(parser);
	return name;
}

export function checkReservedWord(
	parser: JSParser,
	word: string,
	loc: SourceLocation,
	checkKeywords: boolean,
	isBinding: boolean,
): void {
	if (isSyntaxEnabled(parser, "ts")) {
		// TypeScript support in Babel disables reserved word checking...
		// This is mostly because TS allows reserved words in certain scenarios
		// TODO we should just allow those rather than relying on this hack
		return undefined;
	}

	if (inScope(parser, "GENERATOR") && word === "yield") {
		unexpectedDiagnostic(
			parser,
			{
				loc,
				description: descriptions.JS_PARSER.YIELD_NAME_IN_GENERATOR,
			},
		);
	}

	if (inScope(parser, "ASYNC") && word === "await") {
		unexpectedDiagnostic(
			parser,
			{
				loc,
				description: descriptions.JS_PARSER.AWAIT_NAME_IN_ASYNC,
			},
		);
	}

	if (inScope(parser, "CLASS_PROPERTY") && word === "arguments") {
		unexpectedDiagnostic(
			parser,
			{
				loc,
				description: descriptions.JS_PARSER.ARGUMENTS_IN_CLASS_FIELD,
			},
		);
	}

	if (checkKeywords && isKeyword(word)) {
		unexpectedDiagnostic(
			parser,
			{
				loc,
				description: descriptions.JS_PARSER.UNEXPECTED_KEYWORD(word),
			},
		);
	}

	let isReserved: boolean;
	if (inScope(parser, "STRICT")) {
		if (isBinding) {
			isReserved = isStrictBindReservedWord(word, parser.meta.inModule);
		} else {
			isReserved = isStrictReservedWord(word, parser.meta.inModule);
		}
	} else {
		isReserved = isReservedWord(word, parser.meta.inModule);
	}

	if (isReserved) {
		if (!inScope(parser, "ASYNC") && word === "await") {
			unexpectedDiagnostic(
				parser,
				{
					loc,
					description: descriptions.JS_PARSER.AWAIT_OUTSIDE_ASYNC,
				},
			);
		} else {
			unexpectedDiagnostic(
				parser,
				{
					loc,
					description: descriptions.JS_PARSER.RESERVED_WORD(word),
				},
			);
		}
	}
}

// Parses await expression inside async function.
export function parseAwait(parser: JSParser): JSAwaitExpression {
	if (!parser.state.awaitPos) {
		parser.state.awaitPos = parser.state.index;
	}

	if (!inScope(parser, "ASYNC")) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.AWAIT_OUTSIDE_ASYNC,
			},
		);
	}

	const start = parser.getPosition();
	next(parser);

	if (inScope(parser, "PARAMETERS")) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.AWAIT_IN_ASYNC_PARAMS,
			},
		);
	}

	if (eat(parser, tt.star)) {
		unexpectedDiagnostic(
			parser,
			{
				start,
				description: descriptions.JS_PARSER.AWAIT_STAR,
			},
		);
	}

	const argument = parseMaybeUnary(parser, "await argument");
	return parser.finishNode(start, {type: "JSAwaitExpression", argument});
}

// Parses yield expression inside generator.
export function parseYield(parser: JSParser, noIn?: boolean): JSYieldExpression {
	if (!parser.state.yieldPos) {
		parser.state.yieldPos = parser.state.index;
	}

	const start = parser.getPosition();

	if (inScope(parser, "PARAMETERS")) {
		unexpectedDiagnostic(
			parser,
			{
				start,
				description: descriptions.JS_PARSER.YIELD_IN_GENERATOR_PARAMS,
			},
		);
	}

	if (
		parser.state.maybeInArrowParameters &&
		// We only set yieldInPossibleArrowParameters if we haven't already
		// found a possible invalid JSYieldExpression.
		parser.state.yieldInPossibleArrowParameters === undefined
	) {
		parser.state.yieldInPossibleArrowParameters = start;
	}

	next(parser);

	let delegate: undefined | boolean;
	let argument: undefined | AnyJSExpression;
	if (
		match(parser, tt.semi) ||
		(!match(parser, tt.star) && !parser.state.tokenType.startsExpr) ||
		canInsertSemicolon(parser)
	) {
		delegate = false;
	} else {
		delegate = eat(parser, tt.star);
		argument = parseMaybeAssign<AnyJSExpression>(parser, "yield argument", noIn);
	}

	return parser.finishNode(
		start,
		{
			type: "JSYieldExpression",
			delegate,
			argument,
		},
	);
}

function parseNullLiteral(parser: JSParser): JSNullLiteral {
	const start = parser.getPosition();
	next(parser);
	return parser.finishNode(start, {type: "JSNullLiteral"});
}

export function parseStringLiteral(parser: JSParser): JSStringLiteral {
	const start = parser.getPosition();
	const value = String(parser.state.tokenValue);
	next(parser);
	return parser.finishNode(
		start,
		{
			type: "JSStringLiteral",
			value,
		},
	);
}

function parseBigIntLiteral(parser: JSParser): JSBigIntLiteral {
	const start = parser.getPosition();
	const value = String(parser.state.tokenValue);
	next(parser);
	return parser.finishNode(
		start,
		{
			type: "JSBigIntLiteral",
			value,
		},
	);
}

export function parseNumericLiteral(parser: JSParser): JSNumericLiteral {
	const start = parser.getPosition();
	const {tokenValue} = parser.state;
	if (!(tokenValue instanceof NumberTokenValue)) {
		throw new Error("Expected NumberTokenValue");
	}

	const {value, format} = tokenValue;
	next(parser);
	return parser.finishNode(
		start,
		{
			type: "JSNumericLiteral",
			format,
			value,
		},
	);
}

function parseRegExpLiteral(parser: JSParser): JSRegExpLiteral {
	const start = parser.getPosition();
	const value = parser.state.tokenValue;
	if (!(value instanceof RegExpTokenValue)) {
		throw new Error("Expected regex token value");
	}
	next(parser);

	const {flags, pattern} = value;

	const {diagnostics, expression} = parseRegExp({
		// Advance past first slash
		offsetPosition: {
			line: start.line,
			column: ob1Inc(start.column),
		},
		path: parser.filename,
		input: pattern,
		unicode: flags.has("u"),
	});

	for (const diagnostic of diagnostics) {
		parser.addDiagnostic(diagnostic);
	}

	return parser.finishNode(
		start,
		{
			type: "JSRegExpLiteral",
			expression,
			global: flags.has("g"),
			multiline: flags.has("m"),
			sticky: flags.has("y"),
			insensitive: flags.has("i"),
			noDotNewline: flags.has("s"),
			unicode: flags.has("u"),
		},
	);
}

function parseImportOrMetaProperty(
	parser: JSParser,
): JSImportCall | JSMetaProperty {
	if (lookaheadState(parser).tokenType === tt.dot) {
		return parseImportMetaProperty(parser);
	} else {
		return parseImportCall(parser);
	}
}

function parseImportCall(parser: JSParser): JSImportCall {
	expect(parser, tt._import);

	const start = parser.getPosition();
	const openContext = expectOpening(parser, tt.parenL, tt.parenR, "array");

	let argument: ReturnType<typeof parseCallArgument>;

	if (match(parser, tt.parenR)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.IMPORT_EXACT_ARGUMENTS,
			},
		);

		argument = toReferenceIdentifier(
			parser,
			createUnknownIdentifier(parser, "import call argument"),
		);
	} else {
		const callArg = parseCallArgument(parser, "call expression argument", false);
		if (callArg.type === "JSArrayHole") {
			throw new Error(
				"Expected argument, parseExpressionListItem was passed maybeAllowEmpty: false",
			);
		} else {
			argument = callArg;
		}
	}

	// TODO warn on multiple arguments
	if (eat(parser, tt.comma)) {
		unexpectedDiagnostic(
			parser,
			{
				start: parser.state.lastStartPos,
				end: parser.state.lastEndPos,
				description: descriptions.JS_PARSER.IMPORT_TRAILING_COMMA,
			},
		);
	}

	if (argument.type === "JSSpreadElement") {
		unexpectedDiagnostic(
			parser,
			{
				loc: argument.loc,
				description: descriptions.JS_PARSER.IMPORT_SPREAD,
			},
		);
	}

	expectClosing(parser, openContext);

	const spreadOrExpression: AnyJSExpression | JSSpreadElement =
		argument.type === "JSAmbiguousFlowTypeCastExpression"
			? argument.expression
			: argument;

	const expression: AnyJSExpression =
		spreadOrExpression.type === "JSSpreadElement"
			? spreadOrExpression.argument
			: spreadOrExpression;

	return parser.finishNode(start, {type: "JSImportCall", argument: expression});
}

function parseSuper(parser: JSParser): JSSuper {
	if (
		!inScope(parser, "METHOD") &&
		!inScope(parser, "CLASS_PROPERTY") &&
		parser.options.sourceType !== "template"
	) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.SUPER_OUTSIDE_METHOD,
			},
		);
	}

	const start = parser.getPosition();
	next(parser);

	if (
		!match(parser, tt.parenL) &&
		!match(parser, tt.bracketL) &&
		!match(parser, tt.dot)
	) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.INVALID_SUPER_SUFFIX,
			},
		);
	}

	const loc = parser.finishLoc(start);

	if (
		match(parser, tt.parenL) &&
		(getLastScope(parser, "METHOD") !== "constructor" ||
		getLastScope(parser, "CLASS") !== "derived") &&
		parser.options.sourceType !== "template"
	) {
		unexpectedDiagnostic(
			parser,
			{
				loc,
				description: descriptions.JS_PARSER.SUPER_CALL_OUTSIDE_CONSTRUCTOR,
			},
		);
	}

	return parser.finalizeNode({
		type: "JSSuper",
		loc,
	});
}

function parseDoExpression(parser: JSParser): JSDoExpression {
	const start = parser.getPosition();
	next(parser);
	const oldLabels = parser.state.labels;
	parser.state.labels = [];
	pushScope(parser, "FUNCTION", false);
	const body = parseBlock(parser, false);
	popScope(parser, "FUNCTION");
	parser.state.labels = oldLabels;
	return parser.finishNode(
		start,
		{
			type: "JSDoExpression",
			body,
		},
	);
}

export function parseArrayHole(parser: JSParser): JSArrayHole {
	return parser.finishNode(
		parser.getPosition(),
		{
			type: "JSArrayHole",
		},
	);
}

function parseArrayExpression(
	parser: JSParser,
	refShorthandDefaultPos?: IndexTracker,
): JSArrayExpression {
	const start = parser.getPosition();
	const openContext = expectOpening(parser, tt.bracketL, tt.bracketR, "array");

	const elements = toReferencedListOptional(
		parser,
		parseExpressionList(
			parser,
			"array element",
			openContext,
			true,
			refShorthandDefaultPos,
		),
	);

	return parser.finishNode(
		start,
		{
			type: "JSArrayExpression",
			elements,
		},
	);
}
