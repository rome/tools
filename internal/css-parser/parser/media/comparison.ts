import {CSSParser} from "@internal/css-parser/types";
import {
	CSSMediaCondition,
	CSSMediaFeatureComparison,
	CSSMediaFeatureEQ,
	CSSMediaFeatureGT,
	CSSMediaFeatureLT,
	CSSMediaInParens,
	MediaAndOr,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {
	parseMediaAnd,
	parseMediaNot,
	parseMediaOr,
} from "@internal/css-parser/parser/media/conditions";
import {Position} from "@internal/parser-core";
import {parseMediaInParens} from "@internal/css-parser/parser/media/inParens";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";

export function parseMediaFeatureGT(
	parser: CSSParser,
): CSSMediaFeatureGT | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Ident" && token.value === ">") {
		const equalToken = parser.nextToken();
		const hasEqual = equalToken.type === "Ident" && equalToken.value === "+";
		return parser.finishNode(
			start,
			{
				type: "CSSMediaFeatureGT",
				hasEqual,
			},
		);
	}
	return undefined;
}

export function parseMediaFeatureLT(
	parser: CSSParser,
): CSSMediaFeatureLT | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Ident" && token.value === "<") {
		const equalToken = parser.nextToken();
		const hasEqual = equalToken.type === "Ident" && equalToken.value === "+";
		return parser.finishNode(
			start,
			{
				type: "CSSMediaFeatureLT",
				hasEqual,
			},
		);
	}
	return undefined;
}

export function parseMediaFeatureEQ(
	parser: CSSParser,
): CSSMediaFeatureEQ | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();

	if (token.type === "Ident" && token.value === "=") {
		return parser.finishNode(
			start,
			{
				type: "CSSMediaFeatureEQ",
			},
		);
	}
	return undefined;
}

export function parseMediaFeatureComparison(
	parser: CSSParser,
): CSSMediaFeatureComparison | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	let value:
		| CSSMediaFeatureGT
		| CSSMediaFeatureEQ
		| CSSMediaFeatureLT
		| undefined = undefined;

	const eq = parseMediaFeatureEQ(parser);
	if (eq) {
		value = eq;
	} else {
		const lt = parseMediaFeatureLT(parser);
		if (lt) {
			value = lt;
		} else {
			const gt = parseMediaFeatureGT(parser);
			if (gt) {
				value = gt;
			}
		}
	}

	if (value) {
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSMediaFeatureComparison",
				value,
			},
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.MEDIA_QUERY_EXPECTED_COMPARISON,
		token,
	});

	return undefined;
}

export function parseMediaCondition(
	parser: CSSParser,
	startOfNotToken?: Position,
): CSSMediaCondition | undefined {
	const start = startOfNotToken ?? parser.getPosition();
	const token = parser.getToken();

	if (startOfNotToken) {
		const value = parseMediaNot(parser, startOfNotToken);
		if (value) {
			return parser.finishNode(
				start,
				{
					type: "CSSMediaCondition",
					value,
				},
			);
		}
	}

	if (token.type === "LeftParen") {
		let value: [CSSMediaInParens, ...MediaAndOr[]] | undefined = undefined;
		const mediaInParens = parseMediaInParens(parser);
		if (mediaInParens) {
			value = [mediaInParens];
			while (matchToken(parser, "Whitespace")) {
				readToken(parser, "Whitespace");
			}
			while (!parser.matchToken("EOF")) {
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}

				if (parser.matchToken("LeftCurlyBracket")) {
					break;
				}
				const token = parser.getToken();

				if (token.type === "Ident") {
					if (token.value === "and") {
						const mediaAnd = parseMediaAnd(parser);
						if (mediaAnd) {
							value.push(mediaAnd);
						}
					} else if (token.value === "or") {
						const mediaOr = parseMediaOr(parser);
						if (mediaOr) {
							value.push(mediaOr);
						}
					}
				}
			}

			return parser.finishNode(
				start,
				{
					type: "CSSMediaCondition",
					value,
				},
			);
		}
	}

	return undefined;
}
