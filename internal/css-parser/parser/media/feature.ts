import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSDimension,
	CSSIdentifier,
	CSSMediaFeature,
	CSSMediaFeatureBoolean,
	CSSMediaFeatureName,
	CSSMediaFeaturePlain,
	CSSMediaFeatureValue,
	CSSNumber,
	CSSRatio,
} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";

export function parseMediaFeatureName(
	parser: CSSParser,
): CSSMediaFeatureName | undefined {
	// skip possible comments and spaces
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();
	const ident = parser.eatToken("Ident");
	// skip possible comments and spaces
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const namePosition = parser.getPosition();
	const colon = parser.eatToken("Colon");
	if (!(ident && colon)) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.MEDIA_QUERY_FEATURE_MALFORMED_PLAN,
			token,
		});
		nextToken(parser);
		return undefined;
	}

	return parser.finishNode(
		namePosition,
		{
			type: "CSSMediaFeatureName",
			value: ident.value,
		},
	);
}

export function parseMediaFeatureValue(
	parser: CSSParser,
): CSSMediaFeatureValue | undefined {
	// move forward and get rid of all the white spaces
	nextToken(parser);
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();
	const start = parser.getPosition();
	let value: CSSDimension | CSSIdentifier | CSSNumber | CSSRatio | undefined = undefined;

	if (token.type === "Ident") {
		nextToken(parser);
		value = parser.finishNode(
			start,
			{
				type: "CSSIdentifier",
				value: token.value,
			},
		);
	} else if (token.type === "Dimension") {
		nextToken(parser);
		value = parser.finishNode(
			start,
			{
				type: "CSSDimension",
				unit: token.unit,
				value: token.value,
			},
		);
	} else if (token.type === "Number") {
		const firstNumber = parser.finishNode(
			start,
			{
				type: "CSSNumber",
				raw: token.raw,
				value: token.value,
			},
		);
		nextToken(parser);

		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}

		if (
			matchToken(parser, "Delim") &&
			(parser.getToken() as Tokens["Delim"]).value === "/"
		) {
			nextToken(parser);
			while (matchToken(parser, "Whitespace")) {
				readToken(parser, "Whitespace");
			}
			const maybeNumberToken = parser.getToken();
			if (maybeNumberToken.type === "Number") {
				const secondStart = parser.getPosition();
				nextToken(parser);
				value = parser.finishNode(
					start,
					{
						type: "CSSRatio",
						numerator: firstNumber,
						denominator: parser.finishNode(
							secondStart,
							{
								type: "CSSNumber",
								raw: maybeNumberToken.raw,
								value: maybeNumberToken.value,
							},
						),
					},
				);
			} else {
				nextToken(parser);
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.MEDIA_QUERY_FEATURE_INCORRENT_RATIO,
					token: maybeNumberToken,
				});
				return undefined;
			}
		} else {
			value = firstNumber;
		}
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.MEDIA_QUERY_FEATURE_UNEXPECTED_VALUE,

			token,
		});
		nextToken(parser);
		return undefined;
	}

	return parser.finishNode(
		start,
		{
			type: "CSSMediaFeatureValue",
			value,
		},
	);
}

export function parseMediaFeaturePlain(
	parser: CSSParser,
): CSSMediaFeaturePlain | undefined {
	const start = parser.getPosition();
	// remove white spaces between keyword and next important token
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const name = parseMediaFeatureName(parser);
	const value = parseMediaFeatureValue(parser);

	if (name && value) {
		return parser.finishNode(
			start,
			{
				type: "CSSMediaFeaturePlain",
				name,
				value,
			},
		);
	}
	return undefined;
}

export function parseMediaFeature(
	parser: CSSParser,
): CSSMediaFeature | undefined {
	const start = parser.getPosition();
	let value: CSSMediaFeatureBoolean | CSSMediaFeaturePlain | undefined = undefined;

	/**
	 * At the moment we don't support media ranges, which means that every media
	 * feature must start with an ident token.
	 *
	 * When we will implement the ranges, this will change
	 */
	// in every case, the first token must but an Ident
	const startToken = readToken(parser, "Ident") as Tokens["Ident"];
	// the value of the feature can be a:
	// - plain: "(max-width: 600px)", "(hover: hover)"
	// - boolean: "(color)"
	// we now remove possible white spaces
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	const nextToken = parser.getToken();

	// if we have a right parenthesis, it means we have a boolean
	if (nextToken.type === "RightParen") {
		value = parser.finishNode(
			start,
			{
				type: "CSSMediaFeatureBoolean",
				value: startToken.value,
			},
		);
	} else if (nextToken.type === "Colon") {
		const name = parser.finishNode(
			start,
			{
				type: "CSSMediaFeatureName",
				value: startToken.value,
			},
		);
		const featureValue = parseMediaFeatureValue(parser);
		if (featureValue) {
			value = parser.finishNode(
				start,
				{
					type: "CSSMediaFeaturePlain",
					value: featureValue,
					name,
				},
			);
		}
	}
	// TODO: to implement range features once we have the logic and it's more widely supported

	if (value) {
		return parser.finishNode(
			start,
			{
				type: "CSSMediaFeature",
				value,
			},
		);
	}

	return undefined;
}
