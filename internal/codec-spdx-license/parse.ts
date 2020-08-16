/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	ComplexNode,
	ParserCore,
	ParserOptions,
	SimpleToken,
	ValueToken,
	createParser,
	isAlpha,
	isDigit,
} from "@internal/parser-core";
import {getSPDXLicense, licenseNames} from "./index";
import {descriptions} from "@internal/diagnostics";
import {Number0} from "@internal/ob1";

//# Tokens
type Tokens = BaseTokens & {
	ParenOpen: SimpleToken<"ParenOpen">;
	ParenClose: SimpleToken<"ParenClose">;
	Plus: SimpleToken<"Plus">;
	And: SimpleToken<"And">;
	With: SimpleToken<"With">;
	Or: SimpleToken<"Or">;
	Word: ValueToken<"Word", string>;
};

//# Nodes
export type ExpressionNode = LicenseNode | AndNode | OrNode;

type AndNode = ComplexNode<
	"And",
	{
		left: ExpressionNode;
		right: ExpressionNode;
	}
>;

type OrNode = ComplexNode<
	"Or",
	{
		left: ExpressionNode;
		right: ExpressionNode;
	}
>;

type LicenseNode = ComplexNode<
	"License",
	{
		plus: boolean;
		id: string;
		exception: undefined | string;
	}
>;

function isWordChar(char: string) {
	return isAlpha(char) || isDigit(char) || char === "-" || char === ".";
}

type SPDXLicenseParserOptions = ParserOptions & {
	loose?: boolean;
};

type SPDXParserTypes = {
	tokens: Tokens;
	state: {};
	options: SPDXLicenseParserOptions;
	meta: void;
};

type SPDXParser = ParserCore<SPDXParserTypes>;

const createSPDXLicenseParser = createParser<SPDXParserTypes>({
	diagnosticCategory: "parse/spdxLicense",
	ignoreWhitespaceTokens: true,
	tokenize(parser: SPDXParser, index: Number0) {
		const char = parser.getInputCharOnly(index);

		if (char === "+") {
			return parser.finishToken("Plus");
		}

		if (char === "(") {
			return parser.finishToken("ParenOpen");
		}

		if (char === ")") {
			return parser.finishToken("ParenClose");
		}

		if (isWordChar(char)) {
			const [value, end] = parser.readInputFrom(index, isWordChar);

			if (value === "AND") {
				return parser.finishToken("And", end);
			} else if (value === "OR") {
				return parser.finishToken("Or", end);
			} else if (value === "WITH") {
				return parser.finishToken("With", end);
			} else {
				return parser.finishValueToken("Word", value, end);
			}
		}

		return undefined;
	},
});

function parseLicense(parser: SPDXParser, token: Tokens["Word"]): LicenseNode {
	const startPos = parser.getPosition();
	parser.nextToken();

	// Validate id
	const id = token.value;
	let licenseInfo = getSPDXLicense(id);
	const nextToken = parser.getToken();

	// Sometimes licenses will be specified as "Apache 2.0" but what they actually meant was "Apache-2.0"

	// In loose mode, just make it equivalent, otherwise, complain
	if (licenseInfo === undefined && nextToken.type === "Word") {
		const possibleCorrectLicense = `${id}-${nextToken.value}`;
		const possibleLicenseInfo = getSPDXLicense(possibleCorrectLicense);

		if (possibleLicenseInfo !== undefined) {
			if (parser.options.loose) {
				// Just allow it...
				licenseInfo = possibleLicenseInfo;
				parser.nextToken();
			} else {
				throw parser.unexpected({
					description: descriptions.SPDX.VALID_LICENSE_WITH_MISSING_DASH(
						possibleCorrectLicense,
					),
					start: parser.getPositionFromIndex(token.start),
					end: parser.getPositionFromIndex(nextToken.end),
				});
			}
		}
	}

	if (licenseInfo === undefined) {
		throw parser.unexpected({
			description: descriptions.SPDX.UNKNOWN_LICENSE(id, licenseNames),
			start: parser.getPositionFromIndex(token.start),
			end: parser.getPositionFromIndex(token.end),
		});
	}

	// Is this a plus? (wtf is this)
	const plus = parser.eatToken("Plus") !== undefined;

	// Get exception
	let exception;
	if (parser.eatToken("With")) {
		const token = parser.getToken();
		if (token.type === "Word") {
			exception = token.value;
			parser.nextToken();
		} else {
			throw parser.unexpected({
				description: descriptions.SPDX.WITH_RIGHT_LICENSE_ONLY,
			});
		}
	}

	return {
		type: "License",
		loc: parser.finishLoc(startPos),
		id: licenseInfo.licenseId,
		exception,
		plus,
	};
}

function parseExpression(parser: SPDXParser): ExpressionNode {
	const startPos = parser.getPosition();
	const startToken = parser.getToken();

	let value;

	switch (startToken.type) {
		case "ParenOpen": {
			parser.nextToken();
			value = parseExpression(parser);
			parser.expectToken("ParenClose");
			break;
		}

		case "Word": {
			value = parseLicense(parser, startToken);
			break;
		}

		case "Or":
		case "And":
			throw parser.unexpected({
				description: descriptions.SPDX.OPERATOR_NOT_BETWEEN_EXPRESSION,
			});

		case "Plus":
			throw parser.unexpected({
				description: descriptions.SPDX.PLUS_NOT_AFTER_LICENSE,
			});

		case "ParenClose":
			throw parser.unexpected({
				description: descriptions.SPDX.UNOPENED_PAREN,
			});

		default:
			throw parser.unexpected();
	}

	// Parse and/or
	const nextToken = parser.getToken();
	switch (nextToken.type) {
		case "Or": {
			parser.nextToken();
			return {
				type: "Or",
				loc: parser.finishLoc(startPos),
				left: value,
				right: parseExpression(parser),
			};
		}

		case "And": {
			parser.nextToken();
			return {
				type: "And",
				loc: parser.finishLoc(startPos),
				left: value,
				right: parseExpression(parser),
			};
		}

		default:
			return value;
	}
}

export default function parse(opts: SPDXLicenseParserOptions): ExpressionNode {
	const parser = createSPDXLicenseParser(opts);
	const expr = parseExpression(parser);
	parser.finalize();
	return expr;
}
