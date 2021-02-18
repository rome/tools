// Source: https://www.w3.org/TR/css-values-3/#calc-syntax
import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSCalcNumberProduct,
	CSSCalcNumberSum,
	CSSCalcNumberValue,
	CSSCalcOperation,
	CSSCalcProduct,
	CSSCalcProductValue,
	CSSCalcSum,
	CSSCalcSumValue,
	CSSCalcValue,
	CSSDimension,
	CSSNumber,
	CSSPercentage,
} from "@internal/ast";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

/**
 * As rule, the only functions that are allowed to proceed to next tokens are
 * only the tokens that emit values (operators and numbers), the rest of the functions
 * are only allowed to do checks on the current tokens.
 *
 * Following this logic, allows to not mess up the complex rule around the calc.
 */

function tryParseNumberProduct(
	parser: CSSParser,
): CSSCalcNumberProduct | undefined {
	const numberValue = tryParseNumberValue(parser);
	const start = parser.getPosition();
	if (numberValue) {
		let value: Array<CSSCalcNumberValue | CSSCalcOperation> = [numberValue];

		while (true) {
			if (matchToken(parser, "RightParen")) {
				break;
			}
			const operation = tryParseOperator(parser);
			if (operation) {
				if (operation.value === "*") {
					const numberValue = tryParseNumberValue(parser);
					if (numberValue) {
						value.push(...[operation, numberValue]);
					}
				} else if (operation.value === "/") {
					const calcValue = tryParseNumberValue(parser);
					if (calcValue) {
						value.push(...[operation, calcValue]);
					}
				} else {
					break;
				}
			} else {
				break;
			}
		}

		return parser.finishNode(
			start,
			{
				type: "CSSCalcNumberProduct",
				value,
			},
		);
	}

	return undefined;
}

function tryParseNumberSum(parser: CSSParser): CSSCalcNumberSum | undefined {
	parser.expectToken("LeftParen");
	const start = parser.getPosition();
	let value: Array<CSSCalcNumberProduct | CSSCalcOperation>;

	const numberProduct = tryParseNumberProduct(parser);
	if (numberProduct) {
		value = [numberProduct];
		while (!matchToken(parser, "EOF")) {
			if (matchToken(parser, "RightParen")) {
				parser.eatToken("RightParen");
				break;
			}
			const operation = tryParseOperator(parser, true);
			if (operation) {
				const product = tryParseNumberProduct(parser);
				if (product) {
					value.push(...[operation, product]);
				} else {
					break;
				}
			} else {
				break;
			}
		}
		return parser.finishNode(
			start,
			{
				type: "CSSCalcNumberSum",
				value,
			},
		);
	}
	return undefined;
}

function tryParseNumberValue(parser: CSSParser): CSSCalcNumberValue | undefined {
	const start = parser.getPosition();
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();

	if (token.type === "Number") {
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSCalcNumberValue",
				value: parser.finishNode(
					start,
					{
						type: "CSSNumber",
						raw: token.raw,
						value: token.value,
					},
				),
			},
		);
	} else if (token.type === "LeftParen") {
		const maybeValue = tryParseNumberSum(parser);
		if (maybeValue) {
			return parser.finishNode(
				start,
				{
					type: "CSSCalcNumberValue",
					value: maybeValue,
				},
			);
		}
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.CALC_INCORRECT_NUMBER_VALUE,
		token: parser.getToken(),
	});
	return undefined;
}

function tryParseCalcValue(parser: CSSParser): CSSCalcValue | undefined {
	const start = parser.getPosition();
	if (
		matchToken(parser, "Number") ||
		matchToken(parser, "Percentage") ||
		matchToken(parser, "Dimension")
	) {
		const token = parser.getToken();
		let value: CSSNumber | CSSPercentage | CSSDimension;
		if (token.type === "Number") {
			value = parser.finishNode(
				start,
				{
					type: "CSSNumber",
					raw: token.raw,
					value: token.value,
				},
			);
		} else if (token.type === "Dimension") {
			value = parser.finishNode(
				start,
				{
					type: "CSSDimension",
					value: token.value,
					unit: token.unit,
				},
			);
		} else {
			value = parser.finishNode(
				start,
				{
					type: "CSSPercentage",
					value: (token as Tokens["Percentage"]).value,
				},
			);
		}
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSCalcValue",
				value,
			},
		);
	} else if (matchToken(parser, "LeftParen")) {
		const result = parseCalcSum(parser);

		if (parser.getToken().type !== "RightParen") {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.CALC_UNTERMITED_SUM,
				token: parser.getToken(),
			});
			parser.nextToken();
			return undefined;
		}
		if (result) {
			return parser.finishNode(
				start,
				{
					type: "CSSCalcValue",
					value: result,
				},
			);
		}
	}

	parser.unexpectedDiagnostic({});

	return undefined;
}

function tryParseOperator(
	parser: CSSParser,
	checkWhitespace = false,
): CSSCalcOperation | undefined {
	// let's eat all the possible whitespaces we have
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	if (checkWhitespace) {
		// let's eat the delimiter and read the its previous token
		const op = parser.eatToken("Delim");
		const previousToken = parser.getPreviousToken();
		const start = parser.getPosition();
		if (!(op && (op.value === "+" || op.value === "-"))) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.CALC_OPERATOR_ADD_OR_PLUS_NEEDED,
				token: parser.getToken(),
			});
			return undefined;
		}
		if (!previousToken && checkWhitespace) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.CALC_MISSING_LEFT_SPACE,
				token: previousToken,
			});
			return undefined;
		}
		const rightWhitespace = parser.eatToken("Whitespace");
		if (!rightWhitespace && checkWhitespace) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.CALC_MISSING_RIGHT_SPACE,
				token: rightWhitespace,
			});
			return undefined;
		}
		// let's eat all the possible whitespaces we have
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		return parser.finishNode(
			start,
			{
				type: "CSSCalcOperation",
				value: op.value,
			},
		);
	} else {
		const token = parser.getToken();
		const start = parser.getPosition();
		if (token.type === "Delim") {
			if (token.value === "*" || token.value === "/") {
				// advance and remove all the whitespaces
				parser.nextToken();
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}
				return parser.finishNode(
					start,
					{
						type: "CSSCalcOperation",
						value: token.value,
					},
				);
			}
		}

		return undefined;
	}
}

function tryParseCalcProduct(parser: CSSParser): CSSCalcProduct | undefined {
	const calcValue = tryParseCalcValue(parser);
	const start = parser.getPosition();
	if (calcValue) {
		let value: CSSCalcProductValue = [calcValue];
		while (true) {
			if (matchToken(parser, "RightParen")) {
				break;
			}
			const operation = tryParseOperator(parser);
			if (operation) {
				if (operation.value === "*") {
					const numberValue = tryParseCalcValue(parser);
					if (numberValue) {
						value.push(...[operation, numberValue]);
					}
				} else if (operation.value === "/") {
					const calcValue = tryParseNumberValue(parser);
					if (calcValue) {
						value.push(...[operation, calcValue]);
					}
				} else {
					break;
				}
			} else {
				break;
			}
		}

		return parser.finishNode(
			start,
			{
				type: "CSSCalcProduct",
				value,
			},
		);
	}

	return undefined;
}

export function parseCalcSum(parser: CSSParser): CSSCalcSum | undefined {
	const start = parser.getPosition();
	let value: CSSCalcSumValue;

	const calcProduct = tryParseCalcProduct(parser);
	if (calcProduct) {
		value = [calcProduct];
		while (!matchToken(parser, "EOF")) {
			if (matchToken(parser, "RightParen")) {
				break;
			}
			const operation = tryParseOperator(parser, true);
			if (operation) {
				const product = tryParseCalcProduct(parser);
				if (product) {
					value.push(...[operation, product]);
				} else {
					break;
				}
			} else {
				break;
			}
		}
		return parser.finishNode(
			start,
			{
				type: "CSSCalcSum",
				value,
			},
		);
	}

	return undefined;
}
