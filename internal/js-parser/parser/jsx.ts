/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {types as tt} from "../tokenizer/types";
import {Position, SourceLocation} from "@internal/parser-core";
import {
	JSParser,
	atEOF,
	eat,
	expect,
	expectClosing,
	expectOpening,
	isRelational,
	isSyntaxEnabled,
	lookaheadState,
	match,
	next,
	unexpectedDiagnostic,
} from "../parser";
import {
	JSStringLiteral,
	JSXAttribute,
	JSXElement,
	JSXEmptyExpression,
	JSXExpressionContainer,
	JSXFragment,
	JSXIdentifier,
	JSXNamespacedName,
	JSXSpreadAttribute,
	JSXSpreadChild,
	JSXText,
} from "@internal/ast";
import {
	parseExpression,
	parseMaybeAssign,
	parseStringLiteral,
	parseTSTypeArguments,
} from "./index";
import {descriptions} from "@internal/diagnostics";
import {isValidIdentifierName} from "@internal/js-ast-utils";

// Indicates whether we should create a JSXIdentifier or a JSXReferenceIdentifier
function isHTMLElementName(tagName: string): boolean {
	return /^[a-z]|-/.test(tagName) && isValidIdentifierName(tagName);
}

// Transforms JSX element name to string.
function getQualifiedJSXName(
	node: undefined | JSXElement["name"] | JSXIdentifier,
): string {
	if (node === undefined) {
		return "";
	}

	switch (node.type) {
		case "JSXIdentifier":
		case "JSXReferenceIdentifier":
			return node.name;

		case "JSXNamespacedName":
			return `${node.namespace.name}:${node.name.name}`;

		case "JSXMemberExpression":
			return `${getQualifiedJSXName(node.object)}.${getQualifiedJSXName(
				node.property,
			)}`;
	}
}

// Parse next token as JSX identifier
function parseJSXIdentifier(parser: JSParser): JSXIdentifier {
	const start = parser.getPosition();
	let name;
	if (match(parser, tt.jsxName)) {
		name = String(parser.state.tokenValue);
	} else if (parser.state.tokenType.keyword !== undefined) {
		name = parser.state.tokenType.keyword;
	} else {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.JSX_UNKNOWN_IDENTIFIER_TOKEN,
			},
		);
		name = "";
	}

	next(parser);
	return parser.finishNode(
		start,
		{
			type: "JSXIdentifier",
			name,
		},
	);
}

// Parse namespaced identifier.
function parseJSXNamespacedName(
	parser: JSParser,
): JSXIdentifier | JSXNamespacedName {
	const start = parser.getPosition();

	const namespace = parseJSXIdentifier(parser);
	if (!eat(parser, tt.colon)) {
		return namespace;
	}

	const name = parseJSXIdentifier(parser);
	return parser.finishNode(
		start,
		{
			type: "JSXNamespacedName",
			name,
			namespace,
		},
	);
}

// Parses element name in any form - namespaced, member
// or single identifier.
function parseJSXElementName(parser: JSParser): JSXElement["name"] {
	const start = parser.getPosition();

	const namespacedName = parseJSXNamespacedName(parser);

	let node: JSXElement["name"];
	if (
		namespacedName.type === "JSXIdentifier" &&
		!isHTMLElementName(namespacedName.name)
	) {
		node = {
			...namespacedName,
			type: "JSXReferenceIdentifier",
		};
	} else {
		node = namespacedName;
	}

	while (eat(parser, tt.dot)) {
		const property = parseJSXIdentifier(parser);
		node = parser.finishNode(
			start,
			{
				type: "JSXMemberExpression",
				object: node,
				property,
			},
		);
	}

	return node;
}

// Parses any type of JSX attribute value.
function parseJSXAttributeValue(
	parser: JSParser,
): JSStringLiteral | JSXElement | JSXFragment | JSXExpressionContainer {
	let node;
	switch (parser.state.tokenType) {
		case tt.braceL: {
			node = parseJSXExpressionContainer(parser);
			if (node.expression.type === "JSXEmptyExpression") {
				unexpectedDiagnostic(
					parser,
					{
						loc: node.loc,
						description: descriptions.JS_PARSER.JSX_EMPTY_ATTRIBUTE_VALUE,
					},
				);
			}
			return node;
		}

		case tt.jsxTagStart:
			return parseJSXElement(parser);

		case tt.string:
			return parseStringLiteral(parser);

		default: {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.JSX_INVALID_ATTRIBUTE_VALUE,
				},
			);
			return parser.finishNode(
				parser.getPosition(),
				{
					type: "JSStringLiteral",
					value: "?",
				},
			);
		}
	}
}

// JSXEmptyExpression is unique type since it doesn't actually parse anything,
// and so it should start at the end of last read token (left brace) and finish
// at the beginning of the next one (right brace).
function parseJSXEmptyExpression(parser: JSParser): JSXEmptyExpression {
	return parser.finishNode(
		parser.state.lastEndPos,
		{
			type: "JSXEmptyExpression",
		},
	);
}

// Parse JSX spread child
function parseJSXSpreadChild(parser: JSParser): JSXSpreadChild {
	const start = parser.getPosition();
	const openContext = expectOpening(
		parser,
		tt.braceL,
		tt.braceR,
		"jsx spread child",
	);
	expect(parser, tt.ellipsis);
	const expression = parseExpression(parser, "jsx spread child expression");
	expectClosing(parser, openContext);

	return parser.finishNode(
		start,
		{
			type: "JSXSpreadChild",
			expression,
		},
	);
}

// Parses JSX expression enclosed into curly brackets.
function parseJSXExpressionContainer(parser: JSParser): JSXExpressionContainer {
	const start = parser.getPosition();
	const openContext = expectOpening(
		parser,
		tt.braceL,
		tt.braceR,
		"jsx expression container",
	);
	let expression;
	if (match(parser, tt.braceR)) {
		expression = parseJSXEmptyExpression(parser);
	} else {
		expression = parseExpression(parser, "jsx inner expression container");
	}
	expectClosing(parser, openContext);
	return parser.finishNode(
		start,
		{
			type: "JSXExpressionContainer",
			expression,
		},
	);
}

// Parses following JSX attribute name-value pair.
function parseJSXAttribute(parser: JSParser): JSXSpreadAttribute | JSXAttribute {
	const start = parser.getPosition();

	if (match(parser, tt.braceL)) {
		const openContext = expectOpening(
			parser,
			tt.braceL,
			tt.braceR,
			"jsx attribute spread",
		);
		expect(parser, tt.ellipsis);
		const argument = parseMaybeAssign(parser, "jsx attribute spread");
		expectClosing(parser, openContext);
		return parser.finishNode(
			start,
			{
				type: "JSXSpreadAttribute",
				argument,
			},
		);
	}

	const name = parseJSXNamespacedName(parser);
	const value = eat(parser, tt.eq) ? parseJSXAttributeValue(parser) : undefined;
	return parser.finishNode(
		start,
		{
			type: "JSXAttribute",
			name,
			value,
		},
	);
}

type OpeningElementDef = {
	name: undefined | JSXElement["name"];
	typeArguments: JSXElement["typeArguments"];
	attributes: JSXElement["attributes"];
	selfClosing: boolean;
	loc: SourceLocation;
};

// Parses JSX opening tag starting after "<".
function parseJSXOpeningElementAt(
	parser: JSParser,
	start: Position,
): OpeningElementDef {
	if (match(parser, tt.jsxTagEnd)) {
		expect(parser, tt.jsxTagEnd);
		return {
			typeArguments: undefined,
			name: undefined,
			loc: {
				filename: parser.filename,
				start,
				end: parser.getPosition(),
			},
			attributes: [],
			selfClosing: false,
		};
	}

	const attributes = [];
	const name = parseJSXElementName(parser);

	let typeArguments;
	if (isRelational(parser, "<")) {
		if (!isSyntaxEnabled(parser, "ts")) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.JSX_ELEM_TYPE_ARGUMENTS_OUTSIDE_TS,
				},
			);
		}

		typeArguments = parseTSTypeArguments(parser);
	}

	// We need to check for isRelational('>') here as the above type arguments parsing can put the tokenizer

	// into an unusual state for: <foo<bar>></foo>
	while (
		!match(parser, tt.slash) &&
		!match(parser, tt.jsxTagEnd) &&
		!atEOF(parser)
	) {
		attributes.push(parseJSXAttribute(parser));
	}
	const selfClosing = eat(parser, tt.slash);
	if (!eat(parser, tt.jsxTagEnd)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.JSX_UNCLOSED_SELF_CLOSING_TAG,
			},
		);
	}
	return {
		typeArguments,
		name,
		attributes,
		selfClosing,
		loc: parser.getLoc(name),
	};
}

// Parses JSX closing tag starting after "</".
function parseJSXClosingElementAt(
	parser: JSParser,
): undefined | JSXElement["name"] {
	if (eat(parser, tt.jsxTagEnd)) {
		return undefined;
	}

	const name = parseJSXElementName(parser);

	if (!eat(parser, tt.jsxTagEnd)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.JSX_UNCLOSED_CLOSING_TAG,
			},
		);
	}

	return name;
}

function recoverFromUnclosedJSX(parser: JSParser) {
	// jsxOpenTag
	parser.state.context.pop();
	parser.state.exprAllowed = false;
}

// Parses entire JSX element, including it"s opening tag
// (starting after "<"), attributes, contents and closing tag.
function parseJSXElementAt(
	parser: JSParser,
	start: Position,
): JSXElement | JSXFragment {
	const children = [];
	const openingDef = parseJSXOpeningElementAt(parser, start);

	let closingNameLoc: undefined | SourceLocation;
	let closingName: undefined | JSXElement["name"];

	// Parse children for unclosed elements
	if (!openingDef.selfClosing) {
		contents: while (true) {
			switch (parser.state.tokenType) {
				case tt.jsxTagStart: {
					const start = parser.getPosition();
					next(parser);
					if (eat(parser, tt.slash)) {
						closingName = parseJSXClosingElementAt(parser);
						closingNameLoc = {
							filename: parser.filename,
							start,
							end: parser.getPosition(),
						};
						break contents;
					}
					children.push(parseJSXElementAt(parser, start));
					break;
				}

				case tt.jsxText: {
					children.push(parseJSXText(parser));
					break;
				}

				case tt.braceL: {
					if (lookaheadState(parser).tokenType === tt.ellipsis) {
						children.push(parseJSXSpreadChild(parser));
					} else {
						children.push(parseJSXExpressionContainer(parser));
					}
					break;
				}

				case tt.eof: {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.JSX_UNCLOSED_ELEMENT(
								getQualifiedJSXName(openingDef.name),
								openingDef.loc,
							),
						},
					);
					break contents;
				}

				default: {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.JSX_UNKNOWN_CHILD_START(
								getQualifiedJSXName(openingDef.name),
								openingDef.loc,
							),
						},
					);

					// We don't need to do it for the tt.eof case above because nothing will ever be parsed after
					recoverFromUnclosedJSX(parser);

					break contents;
				}
			}
		}

		// Unclosed element, would have produced an error above but we still want to produce a valid AST and avoid the below error conditions
		if (closingNameLoc === undefined) {
			closingName = openingDef.name;
			closingNameLoc = openingDef.loc;
		}

		// Fragment open, element close
		if (openingDef.name === undefined && closingName !== undefined) {
			unexpectedDiagnostic(
				parser,
				{
					loc: openingDef.loc,
					description: descriptions.JS_PARSER.JSX_EXPECTED_CLOSING_FRAGMENT_TAG(
						getQualifiedJSXName(openingDef.name),
						openingDef.loc,
					),
				},
			);
		}

		// Element open, fragment close
		if (openingDef.name !== undefined && closingName === undefined) {
			unexpectedDiagnostic(
				parser,
				{
					loc: openingDef.loc,
					description: descriptions.JS_PARSER.JSX_EXPECTED_CLOSING_TAG(
						getQualifiedJSXName(openingDef.name),
						openingDef.loc,
					),
				},
			);
		}

		// Validate element names: Element open, element close
		if (openingDef.name !== undefined && closingName !== undefined) {
			if (
				getQualifiedJSXName(closingName) !==
				getQualifiedJSXName(openingDef.name)
			) {
				unexpectedDiagnostic(
					parser,
					{
						loc: openingDef.loc,
						description: descriptions.JS_PARSER.JSX_EXPECTED_CLOSING_TAG(
							getQualifiedJSXName(openingDef.name),
							openingDef.loc,
						),
					},
				);
			}
		}
	}

	checkAccidentalFragment(parser);

	const openingName = openingDef.name;
	if (openingName === undefined) {
		return parser.finishNode(
			start,
			{
				type: "JSXFragment",
				children,
			},
		);
	} else {
		return parser.finishNode(
			start,
			{
				type: "JSXElement",
				name: openingName,
				typeArguments: openingDef.typeArguments,
				attributes: openingDef.attributes,
				selfClosing: openingDef.selfClosing,
				children,
			},
		);
	}
}

function checkAccidentalFragment(parser: JSParser) {
	if (match(parser, tt.relational) && parser.state.tokenValue === "<") {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.UNWRAPPED_ADJACENT_JHX,
			},
		);
	}
}

export function parseJSXText(parser: JSParser): JSXText {
	// No need to assert syntax here because we wont get that far as parseJSXElement would have already been called
	const start = parser.getPosition();
	const value = String(parser.state.tokenValue);
	next(parser);
	return parser.finishNode(
		start,
		{
			type: "JSXText",
			value,
		},
	);
}

// Parses entire JSX element from 'current position.
export function parseJSXElement(parser: JSParser): JSXElement | JSXFragment {
	// Only necessary here as this is the only JSX entry point
	if (!isSyntaxEnabled(parser, "jsx")) {
		if (isSyntaxEnabled(parser, "ts")) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.JSX_IN_TS_EXTENSION,
				},
			);
		} else {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.JSX_DISABLED,
				},
			);
		}
	}

	const start = parser.getPosition();
	next(parser);
	return parseJSXElementAt(parser, start);
}
