/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	IndexTracker,
	createIndexTracker,
	isStrictBindReservedWord,
} from "@internal/js-parser-utils";
import {
	Position,
	SourceLocation,
	comparePositions,
} from "@internal/parser-core";
import {types as tt} from "../tokenizer/types";
import {
	AnyJSAssignmentPattern,
	AnyJSAuxiliary,
	AnyJSBindingPattern,
	AnyJSExpression,
	AnyJSParamBindingPattern,
	AnyJSTargetAssignmentPattern,
	AnyJSTargetBindingPattern,
	AnyNode,
	ConstTSAccessibility,
	JSAmbiguousFlowTypeCastExpression,
	JSArrayHole,
	JSAssignmentIdentifier,
	JSAssignmentObjectPatternProperty,
	JSBindingArrayPattern,
	JSBindingAssignmentPattern,
	JSBindingIdentifier,
	JSBindingObjectPattern,
	JSBindingObjectPatternProperty,
	JSReferenceIdentifier,
	JSSpreadElement,
	JSSpreadProperty,
} from "@internal/ast";
import {
	JSParser,
	OpeningContext,
	createUnknownIdentifier,
	eat,
	expectClosing,
	expectOpening,
	inScope,
	isParenthesized,
	isSyntaxEnabled,
	match,
	next,
	unexpectedDiagnostic,
} from "../parser";
import {
	ambiguousTypeCastToParameter,
	hasTSModifier,
	parseArrayHole,
	parseBindingIdentifier,
	parseMaybeAssign,
	parseObjectPattern,
	parseTSAccessModifier,
	parseTSTypeAnnotation,
	toAssignmentIdentifier,
	toBindingIdentifier,
	toReferenceIdentifier,
} from "./index";
import {descriptions} from "@internal/diagnostics";
import {ob1Get0} from "@internal/ob1";

const VALID_REST_ARGUMENT_TYPES = ["JSIdentifier", "JSMemberExpression"];

type ToAssignmentPatternNode =
	| AnyJSExpression
	| AnyJSAssignmentPattern
	| AnyJSTargetBindingPattern
	| AnyJSAuxiliary;

// Convert existing expression atom to assignable pattern if possible.
export function toAssignmentPattern(
	parser: JSParser,
	node: AnyNode,
	contextDescription: string,
): AnyJSAssignmentPattern {
	switch (node.type) {
		case "JSAssignmentObjectPattern":
		case "JSAssignmentArrayPattern":
		case "JSAssignmentAssignmentPattern":
		case "JSAssignmentObjectPatternProperty":
		case "JSAssignmentIdentifier":
		case "JSMemberExpression":
			return node;

		case "JSAmbiguousFlowTypeCastExpression":
			return toAssignmentPattern(
				parser,
				ambiguousTypeCastToParameter(parser, node),
				contextDescription,
			);

		case "JSBindingIdentifier":
		case "JSReferenceIdentifier":
			return toAssignmentIdentifier(parser, node);

		case "TSAsExpression":
			return {
				...node,
				type: "TSAssignmentAsExpression",
				expression: toTargetAssignmentPattern(
					parser,
					node.expression,
					contextDescription,
				),
			};

		case "TSNonNullExpression":
			return {
				...node,
				type: "TSAssignmentNonNullExpression",
				expression: toTargetAssignmentPattern(
					parser,
					node.expression,
					contextDescription,
				),
			};

		case "TSTypeAssertion":
			return {
				...node,
				type: "TSAssignmentTypeAssertion",
				expression: toTargetAssignmentPattern(
					parser,
					node.expression,
					contextDescription,
				),
			};

		case "JSObjectExpression": {
			const props = [];
			let rest: undefined | JSAssignmentIdentifier;
			for (let index = 0; index < node.properties.length; index++) {
				const prop = node.properties[index];
				if (prop.type === "JSSpreadProperty") {
					const arg = toTargetAssignmentPattern(
						parser,
						prop.argument,
						contextDescription,
					);
					if (arg.type === "JSAssignmentIdentifier") {
						rest = arg;
					} else {
						unexpectedDiagnostic(
							parser,
							{
								loc: arg.loc,
								description: descriptions.JS_PARSER.INVALID_OBJECT_REST_ARGUMENT,
							},
						);
					}
					continue;
				}

				props.push(toAssignmentObjectProperty(parser, prop));
			}
			return {
				type: "JSAssignmentObjectPattern",
				loc: node.loc,
				properties: props,
				rest,
			};
		}

		case "JSArrayExpression": {
			const {list: elements, rest} = toAssignableList(
				parser,
				node.elements,
				contextDescription,
			);
			return {
				type: "JSAssignmentArrayPattern",
				loc: node.loc,
				elements,
				rest,
			};
		}

		case "JSAssignmentExpression": {
			if (node.operator !== "=") {
				unexpectedDiagnostic(
					parser,
					{
						loc: parser.getLoc(node.left),
						description: descriptions.JS_PARSER.INVALID_ASSIGNMENT_PATTERN_OPERATOR,
					},
				);
			}

			return {
				...node,
				type: "JSAssignmentAssignmentPattern",
				left: toTargetAssignmentPattern(parser, node.left, contextDescription),
				right: node.right,
				loc: node.loc,
			};
		}

		default: {
			unexpectedDiagnostic(
				parser,
				{
					loc: node.loc,
					description: descriptions.JS_PARSER.INVALID_LEFT_HAND_SIDE(
						contextDescription,
					),
				},
			);
			return toAssignmentIdentifier(
				parser,
				createUnknownIdentifier(parser, contextDescription),
			);
		}
	}
}

export function toTargetAssignmentPattern(
	parser: JSParser,
	node: ToAssignmentPatternNode,
	contextDescription: string,
): AnyJSTargetAssignmentPattern {
	const binding = toAssignmentPattern(parser, node, contextDescription);

	switch (binding.type) {
		case "JSAssignmentIdentifier":
		case "JSAssignmentArrayPattern":
		case "JSAssignmentObjectPattern":
		case "JSMemberExpression":
		case "TSAssignmentAsExpression":
		case "TSAssignmentNonNullExpression":
		case "TSAssignmentTypeAssertion":
			return binding;

		default: {
			unexpectedDiagnostic(
				parser,
				{
					loc: node.loc,
					description: descriptions.JS_PARSER.INVALID_ASSIGNMENT_TARGET,
				},
			);
			return {
				type: "JSAssignmentIdentifier",
				loc: node.loc,
				name: "X",
			};
		}
	}
}

export function toTargetBindingPattern(
	parser: JSParser,
	node: ToAssignmentPatternNode,
	contextDescription: string,
): AnyJSTargetBindingPattern {
	const binding = toBindingPattern(parser, node, contextDescription);

	switch (binding.type) {
		case "JSBindingIdentifier":
		case "JSBindingArrayPattern":
		case "JSBindingObjectPattern":
			return binding;

		default:
			// TODO return Unknown
			throw new Error(`TODO ${binding.type}`);
	}
}

export function toParamBindingPattern(
	parser: JSParser,
	node: ToAssignmentPatternNode,
	contextDescription: string,
): AnyJSParamBindingPattern {
	const binding = toBindingPattern(parser, node, contextDescription);

	switch (binding.type) {
		case "JSBindingIdentifier":
		case "JSBindingArrayPattern":
		case "JSBindingObjectPattern":
		case "JSBindingAssignmentPattern":
			return binding;

		default:
			// TODO return Unknown
			throw new Error(`TODO ${binding.type}`);
	}
}

export function toBindingPattern(
	parser: JSParser,
	node: ToAssignmentPatternNode,
	contextDescription: string,
): AnyJSBindingPattern {
	const binding = toAssignmentPattern(parser, node, contextDescription);

	if (binding.type === "JSMemberExpression") {
		unexpectedDiagnostic(
			parser,
			{
				loc: node.loc,
				description: descriptions.JS_PARSER.BINDING_MEMBER_EXPRESSION,
			},
		);

		return {
			type: "JSBindingIdentifier",
			name: "X",
			loc: node.loc,
		};
	}

	switch (binding.type) {
		case "JSAssignmentObjectPattern": {
			const newNode: JSBindingObjectPattern = {
				...binding,
				type: "JSBindingObjectPattern",
				rest: binding.rest === undefined
					? undefined
					: toBindingIdentifier(parser, binding.rest),
				properties: binding.properties.map((prop) => {
					const bindingProp = toBindingPattern(parser, prop, contextDescription);

					if (bindingProp.type !== "JSBindingObjectPatternProperty") {
						throw new Error("impossible condition");
					}

					return bindingProp;
				}),
			};
			return newNode;
		}

		case "JSAssignmentAssignmentPattern": {
			const newNode: JSBindingAssignmentPattern = {
				...binding,
				type: "JSBindingAssignmentPattern",
				left: toTargetBindingPattern(parser, binding.left, contextDescription),
			};
			return newNode;
		}

		case "JSAssignmentArrayPattern": {
			const newNode: JSBindingArrayPattern = {
				...binding,
				type: "JSBindingArrayPattern",
				elements: binding.elements.map((elem) =>
					elem.type === "JSArrayHole"
						? elem
						: toParamBindingPattern(parser, elem, contextDescription)
				),
				rest: binding.rest === undefined
					? undefined
					: toTargetBindingPattern(parser, binding.rest, contextDescription),
			};
			return newNode;
		}

		case "JSAssignmentIdentifier": {
			const newNode: JSBindingIdentifier = {
				...binding,
				type: "JSBindingIdentifier",
			};
			return newNode;
		}

		case "JSAssignmentObjectPatternProperty": {
			const newNode: JSBindingObjectPatternProperty = {
				...binding,
				type: "JSBindingObjectPatternProperty",
				value: toBindingPattern(parser, binding.value, contextDescription),
			};
			return newNode;
		}

		default:
			throw new Error(`Unknown node ${node.type}`);
	}
}

export function toAssignmentObjectProperty(
	parser: JSParser,
	prop: AnyNode,
): JSAssignmentObjectPatternProperty {
	switch (prop.type) {
		case "JSObjectMethod": {
			unexpectedDiagnostic(
				parser,
				{
					loc: prop.key.loc,
					description: descriptions.JS_PARSER.OBJECT_PATTERN_CANNOT_CONTAIN_METHODS,
				},
			);

			const fakeProp: JSAssignmentObjectPatternProperty = {
				type: "JSAssignmentObjectPatternProperty",
				loc: prop.loc,
				key: {
					type: "JSStaticPropertyKey",
					value: {
						type: "JSIdentifier",
						name: "X",
						loc: prop.loc,
					},
					loc: prop.loc,
				},
				value: {
					type: "JSAssignmentIdentifier",
					name: "X",
					loc: prop.loc,
				},
			};

			return fakeProp;
		}

		case "JSObjectProperty":
			return {
				...prop,
				type: "JSAssignmentObjectPatternProperty",
				value: toAssignmentPattern(
					parser,
					prop.value,
					"assignment object property value",
				),
			};

		default: {
			unexpectedDiagnostic(
				parser,
				{
					loc: prop.loc,
					description: descriptions.JS_PARSER.INVALID_OBJECT_PATTERN_PROPERTY,
				},
			);
			return {
				type: "JSAssignmentObjectPatternProperty",
				loc: prop.loc,
				key: {
					type: "JSStaticPropertyKey",
					loc: prop.loc,
					value: {
						type: "JSIdentifier",
						loc: prop.loc,
						name: "X",
					},
				},
				value: {
					type: "JSAssignmentIdentifier",
					loc: prop.loc,
					name: "X",
				},
			};
		}
	}
}

export function toAssignableList(
	parser: JSParser,
	exprList: Array<
		| JSArrayHole
		| AnyJSAssignmentPattern
		| JSAmbiguousFlowTypeCastExpression
		| JSSpreadElement
		| AnyJSExpression
	>,
	contextDescription: string,
): {
	list: Array<JSArrayHole | AnyJSAssignmentPattern>;
	rest: undefined | AnyJSTargetAssignmentPattern;
} {
	const newList: Array<JSArrayHole | AnyJSAssignmentPattern> = [];
	let rest: undefined | AnyJSTargetAssignmentPattern;

	let end = exprList.length;

	// Validate last element
	if (end > 0) {
		let last = exprList[end - 1];

		if (last !== undefined && last.type === "JSSpreadElement") {
			const arg = toTargetAssignmentPattern(
				parser,
				last.argument,
				contextDescription,
			);
			rest = arg;
			end--;
		}

		if (
			last !== undefined &&
			last.type === "JSAmbiguousFlowTypeCastExpression" &&
			last.expression.type === "JSSpreadElement"
		) {
			rest = ambiguousTypeCastToParameter(
				parser,
				{
					...last,
					expression: last.expression.argument,
				},
			);
			end--;
		}
	}

	// Turn type casts that we found in function parameter head into type annotated params
	for (let i = 0; i < end; i++) {
		const expr = exprList[i];

		if (expr.type === "JSAmbiguousFlowTypeCastExpression") {
			exprList[i] = ambiguousTypeCastToParameter(parser, expr);
		}

		if (expr.type === "TSAsExpression" || expr.type === "TSTypeAssertion") {
			unexpectedDiagnostic(
				parser,
				{
					loc: expr.loc,
					description: descriptions.JS_PARSER.TS_UNEXPECTED_CAST_IN_PARAMETER_POSITION,
				},
			);
		}
	}

	for (let i = 0; i < end; i++) {
		const elt = exprList[i];

		if (elt.type === "JSSpreadElement") {
			raiseRestNotLast(parser, parser.getLoc(elt));
		}

		if (elt.type === "JSArrayHole") {
			newList.push(elt);
			continue;
		}

		const assign = toAssignmentPattern(parser, elt, contextDescription);
		newList.push(assign);
	}

	return {list: newList, rest};
}

export function toFunctionParamsBindingList(
	parser: JSParser,
	exprList: Array<JSArrayHole | ToReferencedItem>,
	contextDescription: string,
): {
	params: Array<JSBindingAssignmentPattern | AnyJSTargetBindingPattern>;
	rest: undefined | AnyJSTargetBindingPattern;
} {
	const bindingList: Array<
		JSBindingAssignmentPattern | AnyJSTargetBindingPattern
	> = [];

	const {list: assignmentList, rest: assignmentRest} = toAssignableList(
		parser,
		exprList,
		contextDescription,
	);

	const bindingRest =
		assignmentRest === undefined
			? assignmentRest
			: toTargetBindingPattern(parser, assignmentRest, contextDescription);

	for (const item of assignmentList) {
		if (item === undefined) {
			// TODO should never happen?
			continue;
		}

		if (item.type === "JSAssignmentAssignmentPattern") {
			const binding = toBindingPattern(parser, item, contextDescription);
			if (binding.type !== "JSBindingAssignmentPattern") {
				throw new Error("TODO");
			}

			bindingList.push(binding);
			continue;
		}

		const binding = toTargetBindingPattern(parser, item, contextDescription);
		bindingList.push(binding);
	}

	return {params: bindingList, rest: bindingRest};
}

// this is a list of nodes, from 'something like a call expression, we need to filter the
// type casts that we've found that are illegal in this context
export function toReferencedList(
	parser: JSParser,
	exprList: Array<ToReferencedItem>,
	isParenthesizedExpr?: boolean,
): Array<JSSpreadElement | AnyJSExpression> {
	for (let i = 0; i < exprList.length; i++) {
		const expr = exprList[i];
		exprList[i] = normalizeReferencedItem(
			parser,
			expr,
			exprList.length > 1,
			isParenthesizedExpr,
		);
	}

	// @ts-ignore: We actually filtered them out
	return exprList;
}

export function toReferencedListOptional(
	parser: JSParser,
	exprList: Array<JSArrayHole | ToReferencedItem>,
	isParenthesizedExpr?: boolean,
): Array<JSArrayHole | JSSpreadElement | AnyJSExpression> {
	for (let i = 0; i < exprList.length; i++) {
		const expr = exprList[i];
		if (expr.type !== "JSArrayHole") {
			exprList[i] = normalizeReferencedItem(
				parser,
				expr,
				exprList.length > 1,
				isParenthesizedExpr,
			);
		}
	}

	// @ts-ignore: We actually filtered them out
	return exprList;
}

export type ToReferencedItem =
	| JSAmbiguousFlowTypeCastExpression
	| JSSpreadElement
	| AnyJSExpression;

export function normalizeReferencedItem(
	parser: JSParser,
	expr: ToReferencedItem,
	multiple?: boolean,
	isParenthesizedExpr?: boolean,
): AnyJSExpression | JSSpreadElement {
	if (expr.type !== "JSAmbiguousFlowTypeCastExpression") {
		return expr;
	}

	unexpectedDiagnostic(
		parser,
		{
			loc: expr.loc,
			description: descriptions.JS_PARSER.FLOW_TYPE_CAST_IN_TS,
		},
	);

	if (!isParenthesized(parser, expr) && (multiple || !isParenthesizedExpr)) {
		unexpectedDiagnostic(
			parser,
			{
				loc: expr.loc,
				description: descriptions.JS_PARSER.TYPE_CAST_EXPECTED_PARENS,
			},
		);
	}

	if (expr.optional) {
		unexpectedDiagnostic(
			parser,
			{
				loc: expr.loc,
				description: descriptions.JS_PARSER.TYPE_CAST_CANNOT_BE_OPTIONAL,
			},
		);
	}

	const {typeAnnotation, expression} = expr;

	if (typeAnnotation === undefined) {
		unexpectedDiagnostic(
			parser,
			{
				loc: expr.loc,
				description: descriptions.JS_PARSER.TYPE_CAST_WITHOUT_ANNOTATION,
			},
		);
		return expression;
	}

	return expression;
}

export function filterSpread<T extends AnyNode>(
	parser: JSParser,
	elems: Array<T | JSReferenceIdentifier | JSSpreadElement>,
): Array<JSReferenceIdentifier | T> {
	for (let i = 0; i < elems.length; i++) {
		const elem = elems[i];
		if (elem.type === "JSSpreadElement") {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.UNEXPECTED_SPREAD,
				},
			);

			elems[i] = toReferenceIdentifier(
				parser,
				createUnknownIdentifier(parser, "spread substitute"),
			);
		}
	}
	// @ts-ignore Technically wrong but we removed all JSSpreadElement
	return elems;
}

export function toReferencedListDeep(
	parser: JSParser,
	exprList: Array<ToReferencedItem>,
	isParenthesizedExpr?: boolean,
): Array<AnyJSExpression | JSSpreadElement> {
	const refList = toReferencedList(parser, exprList, isParenthesizedExpr);
	toReferencedListDeepItems(parser, refList);
	return refList;
}

export function toReferencedListDeepOptional(
	parser: JSParser,
	exprList: Array<JSArrayHole | ToReferencedItem>,
	isParenthesizedExpr?: boolean,
): Array<JSArrayHole | AnyJSExpression | JSSpreadElement> {
	const refList = toReferencedListOptional(
		parser,
		exprList,
		isParenthesizedExpr,
	);
	toReferencedListDeepItems(parser, refList);
	return refList;
}

function toReferencedListDeepItems(
	parser: JSParser,
	exprList: Array<JSArrayHole | ToReferencedItem>,
) {
	for (let i = 0; i < exprList.length; i++) {
		const expr = exprList[i];
		if (expr.type === "JSArrayExpression") {
			toReferencedListDeepOptional(parser, expr.elements);
		}
	}
}

export function parseSpread(
	parser: JSParser,
	refShorthandDefaultPos?: IndexTracker,
	refNeedsArrowPos?: IndexTracker,
): JSSpreadElement {
	const start = parser.getPosition();
	next(parser);

	const argument = parseMaybeAssign<AnyJSExpression>(
		parser,
		"spread argument",
		false,
		refShorthandDefaultPos,
		undefined,
		refNeedsArrowPos,
	);

	if (ob1Get0(parser.state.commaAfterSpreadAt) === -1 && match(parser, tt.comma)) {
		parser.state.commaAfterSpreadAt = parser.state.index;
	}

	return parser.finishNode(
		start,
		{
			type: "JSSpreadElement",
			argument,
		},
	);
}

// Parses lvalue (assignable) atom.
export function parseTargetBindingPattern(
	parser: JSParser,
): AnyJSTargetBindingPattern {
	switch (parser.state.tokenType) {
		case tt.bracketL:
			return parseArrayPattern(parser);

		case tt.braceL:
			return parseObjectPattern(parser, createIndexTracker());
	}

	return parseBindingIdentifier(parser);
}

function parseArrayPattern(parser: JSParser): JSBindingArrayPattern {
	const start = parser.getPosition();
	const openContext = expectOpening(
		parser,
		tt.bracketL,
		tt.bracketR,
		"array pattern",
	);
	const {list: elements, rest} = parseBindingList(parser, openContext, true);
	return parser.finishNode(
		start,
		{
			type: "JSBindingArrayPattern",
			elements,
			rest,
		},
	);
}

export function parseBindingList(
	parser: JSParser,
	openContext: OpeningContext,
	allowHoles: boolean = false,
	allowTSModifiers: boolean = false,
): {
	list: Array<JSArrayHole | AnyJSParamBindingPattern>;
	rest: undefined | AnyJSTargetBindingPattern;
} {
	const elts: Array<JSArrayHole | AnyJSParamBindingPattern> = [];
	let rest: undefined | AnyJSTargetBindingPattern;

	let first = true;
	while (true) {
		if (match(parser, openContext.close) || match(parser, tt.eof)) {
			expectClosing(parser, openContext);
			break;
		}

		if (first) {
			first = false;
		} else {
			if (!eat(parser, tt.comma)) {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.EXPECTED_COMMA_SEPARATOR(
							openContext.name,
						),
					},
				);
				break;
			}
		}

		if (allowHoles && match(parser, tt.comma)) {
			elts.push(parseArrayHole(parser));
		} else if (match(parser, openContext.close)) {
			expectClosing(parser, openContext);
			break;
		} else if (match(parser, tt.ellipsis)) {
			next(parser);

			rest = parseBindingListItemTypes(
				parser,
				parser.getPosition(),
				parseTargetBindingPattern(parser),
			);

			if (!hasCommaAfterRest(parser)) {
				expectClosing(parser, openContext);
				break;
			}
		} else {
			elts.push(parseBindingListItem(parser, allowTSModifiers));
		}
	}
	return {list: elts, rest};
}

export function parseBindingListNonEmpty(
	parser: JSParser,
	openContext: OpeningContext,
	allowTSModifiers?: boolean,
): {
	list: Array<AnyJSBindingPattern>;
	rest: undefined | AnyJSTargetBindingPattern;
} {
	const list = parseBindingList(parser, openContext, false, allowTSModifiers);
	// @ts-ignore: Need to make this more explicit we set `allowEmpty: false` above
	return list;
}

export function parseBindingListItem(
	parser: JSParser,
	allowTSModifiers: boolean,
): AnyJSParamBindingPattern {
	const start = parser.getPosition();

	let accessibility: undefined | ConstTSAccessibility;
	let readonly = false;
	if (allowTSModifiers) {
		accessibility = parseTSAccessModifier(parser);
		readonly = hasTSModifier(parser, ["readonly"]);
	}

	const left = parseBindingListItemTypes(
		parser,
		start,
		parseTargetBindingPattern(parser),
	);
	const elt = parseMaybeDefault(parser, start, left);

	if (accessibility !== undefined || readonly) {
		if (!isSyntaxEnabled(parser, "ts")) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.TS_DISABLED_BUT_ACCESSIBILITY_OR_READONLY,
				},
			);
		}

		if (
			elt.type !== "JSBindingIdentifier" &&
			elt.type !== "JSBindingAssignmentPattern"
		) {
			unexpectedDiagnostic(
				parser,
				{
					start,
					description: descriptions.JS_PARSER.TS_PARAMETER_PROPERTY_BINDING_PATTERN,
				},
			);
		}

		return parser.finishNode(
			start,
			{
				...elt,
				meta: parser.finishNode(
					start,
					{
						type: "JSPatternMeta",
						accessibility,
						readonly,
					},
				),
			},
		);
	}

	return elt;
}

export function parseBindingListItemTypes(
	parser: JSParser,
	start: Position,
	param: AnyJSTargetBindingPattern,
): AnyJSTargetBindingPattern {
	let typeAnnotation;
	let optional;

	if (eat(parser, tt.question)) {
		if (param.type !== "JSBindingIdentifier") {
			unexpectedDiagnostic(
				parser,
				{
					loc: param.loc,
					description: descriptions.JS_PARSER.TYPE_BINDING_PARAMETER_OPTIONAL,
				},
			);
		}

		optional = true;
	}

	if (match(parser, tt.colon)) {
		typeAnnotation = parseTSTypeAnnotation(parser, true);
	}

	return parser.finalizeNode({
		...param,
		meta: parser.finishNode(
			start,
			{
				type: "JSPatternMeta",
				optional,
				typeAnnotation,
			},
		),
	});
}

// Parses assignment pattern around given atom if possible.
export function parseMaybeDefault(
	parser: JSParser,
	start: Position = parser.getPosition(),
	left: AnyJSTargetBindingPattern = parseTargetBindingPattern(parser),
): AnyJSTargetBindingPattern | JSBindingAssignmentPattern {
	let target: AnyJSBindingPattern;

	if (eat(parser, tt.eq)) {
		const right = parseMaybeAssign<AnyJSExpression>(
			parser,
			"assignment pattern right",
		);
		const assign: JSBindingAssignmentPattern = parser.finishNode(
			start,
			{
				type: "JSBindingAssignmentPattern",
				left,
				right,
			},
		);
		target = assign;
	} else {
		target = left;
	}

	if (
		target.type === "JSBindingAssignmentPattern" &&
		target.meta !== undefined &&
		target.meta.typeAnnotation !== undefined &&
		comparePositions(
			parser.getLoc(target.right).start,
			parser.getLoc(target.meta.typeAnnotation).start,
		) === -1
	) {
		unexpectedDiagnostic(
			parser,
			{
				loc: target.meta.typeAnnotation.loc,
				description: descriptions.JS_PARSER.TYPE_ANNOTATION_AFTER_ASSIGNMENT,
			},
		);
	}

	return target;
}

const ALLOWED_PARENTHESIZED_LVAL_TYPES = [
	"JSIdentifier",
	"JSMemberExpression",
	"TSAsExpression",
	"TSTypeAssertion",
	"TSAssignmentTypeAssertion",
	"TSAssignmentAsExpression",
	"TSAssignmentNonNullExpression",
];

// Verify that a node is an lval — something that can be assigned
// to.
export function checkLVal(
	parser: JSParser,
	expr:
		| JSArrayHole
		| AnyJSAssignmentPattern
		| AnyJSBindingPattern
		| AnyJSExpression,
	maybeIsBinding: undefined | boolean,
	checkClashes: undefined | Map<string, AnyNode>,
	contextDescription: string,
): void {
	const isBinding: boolean =
		maybeIsBinding === undefined ? false : maybeIsBinding;

	// Verify that nodes aren't parenthesized
	if (
		isParenthesized(parser, expr) &&
		!ALLOWED_PARENTHESIZED_LVAL_TYPES.includes(expr.type)
	) {
		let patternType: "object" | "array" | undefined;
		if (expr.type === "JSBindingObjectPattern") {
			patternType = "object";
		}
		if (expr.type === "JSBindingArrayPattern") {
			patternType = "array";
		}
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.INVALID_PARENTEHSIZED_LVAL(
					patternType,
				),
				loc: expr.loc,
			},
		);
	}

	switch (expr.type) {
		case "TSAsExpression":
		case "TSNonNullExpression":
		case "TSTypeAssertion": {
			checkLVal(
				parser,
				expr.expression,
				isBinding,
				checkClashes,
				contextDescription,
			);
			return undefined;
		}

		case "JSBindingIdentifier":
		case "JSReferenceIdentifier":
		case "JSAssignmentIdentifier": {
			if (
				inScope(parser, "STRICT") &&
				isStrictBindReservedWord(expr.name, parser.meta.inModule)
			) {
				unexpectedDiagnostic(
					parser,
					{
						loc: expr.loc,
						description: descriptions.JS_PARSER.RESERVED_WORD(expr.name),
					},
				);
			}

			if (checkClashes !== undefined) {
				const clash = checkClashes.get(expr.name);

				if (clash === undefined) {
					checkClashes.set(expr.name, expr);
				} else {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.ARGUMENT_CLASH_IN_STRICT(
								expr.name,
								expr.loc,
							),
							loc: expr.loc,
						},
					);
				}
			}
			break;
		}

		case "JSAssignmentObjectPattern":
		case "JSBindingObjectPattern": {
			if (expr.rest !== undefined) {
				checkLVal(parser, expr.rest, isBinding, checkClashes, "rest property");
			}

			for (let prop of expr.properties) {
				if (prop.type === "JSBindingObjectPatternProperty") {
					checkLVal(
						parser,
						prop.value,
						isBinding,
						checkClashes,
						"object destructuring pattern",
					);
				} else {
					checkLVal(
						parser,
						prop,
						isBinding,
						checkClashes,
						"object destructuring pattern",
					);
				}
			}
			break;
		}

		case "JSAssignmentObjectPatternProperty":
		case "JSBindingObjectPatternProperty":
			break;

		case "JSAssignmentArrayPattern":
		case "JSBindingArrayPattern": {
			if (expr.rest !== undefined) {
				checkLVal(parser, expr.rest, isBinding, checkClashes, "rest element");
			}

			for (const elem of expr.elements) {
				checkLVal(
					parser,
					elem,
					isBinding,
					checkClashes,
					"array destructuring pattern",
				);
			}
			break;
		}

		case "JSBindingAssignmentPattern": {
			checkLVal(
				parser,
				expr.left,
				isBinding,
				checkClashes,
				"assignment pattern",
			);
			break;
		}
	}
}

export function checkToRestConversion(
	parser: JSParser,
	node: JSSpreadProperty | JSSpreadElement,
): void {
	if (VALID_REST_ARGUMENT_TYPES.includes(node.argument.type) === false) {
		unexpectedDiagnostic(
			parser,
			{
				loc: node.argument.loc,
				description: descriptions.JS_PARSER.REST_INVALID_ARGUMENT,
			},
		);
	}
}

export function hasCommaAfterRest(parser: JSParser): boolean {
	if (match(parser, tt.comma)) {
		raiseRestNotLast(parser);
		return true;
	}

	return false;
}

export function raiseRestNotLast(
	parser: JSParser,
	loc?: SourceLocation,
	start?: Position,
) {
	unexpectedDiagnostic(
		parser,
		{
			start,
			loc,
			description: descriptions.JS_PARSER.DESTRUCTURING_REST_ELEMENT_NOT_LAST,
		},
	);
}

export function checkCommaAfterRestFromSpread(parser: JSParser): void {
	if (ob1Get0(parser.state.commaAfterSpreadAt) > -1) {
		raiseRestNotLast(
			parser,
			undefined,
			parser.getPositionFromIndex(parser.state.commaAfterSpreadAt),
		);
	}
}
