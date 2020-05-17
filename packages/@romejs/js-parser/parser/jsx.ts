/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {types as tt} from "../tokenizer/types";
import {Position, SourceLocation} from "@romejs/parser-core";
import {JSParser} from "../parser";
import {
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
	StringLiteral,
} from "@romejs/js-ast";
import {
	parseExpression,
	parseMaybeAssign,
	parseStringLiteral,
	parseTSTypeArguments,
} from "./index";
import {descriptions} from "@romejs/diagnostics";
import {isValidIdentifierName} from "@romejs/js-ast-utils";

// Indicates whether we should create a JSXIdentifier or a JSXReferenceIdentifier
function isHTMLTagName(tagName: string): boolean {
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
	if (parser.match(tt.jsxName)) {
		name = String(parser.state.tokenValue);
	} else if (parser.state.tokenType.keyword !== undefined) {
		name = parser.state.tokenType.keyword;
	} else {
		parser.addDiagnostic({
			description: descriptions.JS_PARSER.JSX_UNKNOWN_IDENTIFIER_TOKEN,
		});
		name = "";
	}

	parser.next();
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
	if (!parser.eat(tt.colon)) {
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
		!isHTMLTagName(namespacedName.name)
	) {
		node = {
			...namespacedName,
			type: "JSXReferenceIdentifier",
		};
	} else {
		node = namespacedName;
	}

	while (parser.eat(tt.dot)) {
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
): StringLiteral | JSXElement | JSXFragment | JSXExpressionContainer {
	let node;
	switch (parser.state.tokenType) {
		case tt.braceL: {
			node = parseJSXExpressionContainer(parser);
			if (node.expression.type === "JSXEmptyExpression") {
				parser.addDiagnostic({
					loc: node.loc,
					description: descriptions.JS_PARSER.JSX_EMPTY_ATTRIBUTE_VALUE,
				});
			}
			return node;
		}

		case tt.jsxTagStart:
			return parseJSXElement(parser);

		case tt.string:
			return parseStringLiteral(parser);

		default: {
			parser.addDiagnostic({
				description: descriptions.JS_PARSER.JSX_INVALID_ATTRIBUTE_VALUE,
			});
			return parser.finishNode(
				parser.getPosition(),
				{
					type: "StringLiteral",
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
	const openContext = parser.expectOpening(
		tt.braceL,
		tt.braceR,
		"jsx spread child",
	);
	parser.expect(tt.ellipsis);
	const expression = parseExpression(parser, "jsx spread child expression");
	parser.expectClosing(openContext);

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
	const openContext = parser.expectOpening(
		tt.braceL,
		tt.braceR,
		"jsx expression container",
	);
	let expression;
	if (parser.match(tt.braceR)) {
		expression = parseJSXEmptyExpression(parser);
	} else {
		expression = parseExpression(parser, "jsx inner expression container");
	}
	parser.expectClosing(openContext);
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

	if (parser.match(tt.braceL)) {
		const openContext = parser.expectOpening(
			tt.braceL,
			tt.braceR,
			"jsx attribute spread",
		);
		parser.expect(tt.ellipsis);
		const argument = parseMaybeAssign(parser, "jsx attribute spread");
		parser.expectClosing(openContext);
		return parser.finishNode(
			start,
			{
				type: "JSXSpreadAttribute",
				argument,
			},
		);
	}

	const name = parseJSXNamespacedName(parser);
	const value = parser.eat(tt.eq) ? parseJSXAttributeValue(parser) : undefined;
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
	if (parser.match(tt.jsxTagEnd)) {
		parser.expect(tt.jsxTagEnd);
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
	if (parser.isRelational("<")) {
		if (!parser.isSyntaxEnabled("ts")) {
			parser.addDiagnostic({
				description: descriptions.JS_PARSER.JSX_ELEM_TYPE_ARGUMENTS_OUTSIDE_TS,
			});
		}

		typeArguments = parseTSTypeArguments(parser);
	}

	// We need to check for isRelational('>') here as the above type arguments parsing can put the tokenizer

	// into an unusual state for: <foo<bar>></foo>
	while (
		!parser.match(tt.slash) &&
		!parser.match(tt.jsxTagEnd) &&
		!parser.atEOF()
	) {
		attributes.push(parseJSXAttribute(parser));
	}
	const selfClosing = parser.eat(tt.slash);
	if (!parser.eat(tt.jsxTagEnd)) {
		parser.addDiagnostic({
			description: descriptions.JS_PARSER.JSX_UNCLOSED_SELF_CLOSING_TAG,
		});
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
	if (parser.eat(tt.jsxTagEnd)) {
		return undefined;
	}

	const name = parseJSXElementName(parser);

	if (!parser.eat(tt.jsxTagEnd)) {
		parser.addDiagnostic({
			description: descriptions.JS_PARSER.JSX_UNCLOSED_CLOSING_TAG,
		});
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
	if (openingDef.selfClosing === false) {
		contents: while (true) {
			switch (parser.state.tokenType) {
				case tt.jsxTagStart: {
					const start = parser.getPosition();
					parser.next();
					if (parser.eat(tt.slash)) {
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
					if (parser.lookaheadState().tokenType === tt.ellipsis) {
						children.push(parseJSXSpreadChild(parser));
					} else {
						children.push(parseJSXExpressionContainer(parser));
					}
					break;
				}

				case tt.eof: {
					parser.addDiagnostic({
						description: descriptions.JS_PARSER.JSX_UNCLOSED_ELEMENT(
							getQualifiedJSXName(openingDef.name),
							openingDef.loc,
						),
					});
					break contents;
				}

				default: {
					parser.addDiagnostic({
						description: descriptions.JS_PARSER.JSX_UNKNOWN_CHILD_START(
							getQualifiedJSXName(openingDef.name),
							openingDef.loc,
						),
					});

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
			parser.addDiagnostic({
				loc: openingDef.loc,
				description: descriptions.JS_PARSER.JSX_EXPECTED_CLOSING_FRAGMENT_TAG(
					getQualifiedJSXName(openingDef.name),
					openingDef.loc,
				),
			});
		}

		// Element open, fragment close
		if (openingDef.name !== undefined && closingName === undefined) {
			parser.addDiagnostic({
				loc: openingDef.loc,
				description: descriptions.JS_PARSER.JSX_EXPECTED_CLOSING_TAG(
					getQualifiedJSXName(openingDef.name),
					openingDef.loc,
				),
			});
		}

		// Validate element names: Element open, element close
		if (openingDef.name !== undefined && closingName !== undefined) {
			if (
				getQualifiedJSXName(closingName) !==
				getQualifiedJSXName(openingDef.name)
			) {
				parser.addDiagnostic({
					loc: openingDef.loc,
					description: descriptions.JS_PARSER.JSX_EXPECTED_CLOSING_TAG(
						getQualifiedJSXName(openingDef.name),
						openingDef.loc,
					),
				});
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
	if (parser.match(tt.relational) && parser.state.tokenValue === "<") {
		parser.addDiagnostic({
			description: descriptions.JS_PARSER.UNWRAPPED_ADJACENT_JHX,
		});
	}
}

export function parseJSXText(parser: JSParser): JSXText {
	// No need to assert syntax here because we wont get that far as parseJSXElement would have already been called
	const start = parser.getPosition();
	const value = String(parser.state.tokenValue);
	parser.next();
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
	if (!parser.isSyntaxEnabled("jsx")) {
		if (parser.isSyntaxEnabled("ts")) {
			parser.addDiagnostic({
				description: descriptions.JS_PARSER.JSX_IN_TS_EXTENSION,
			});
		} else {
			parser.addDiagnostic({
				description: descriptions.JS_PARSER.JSX_DISABLED,
			});
		}
	}

	const start = parser.getPosition();
	parser.next();
	return parseJSXElementAt(parser, start);
}
