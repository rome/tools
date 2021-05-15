import {
	CSSMediaFeatureComparison,
	CSSMediaFeatureRange,
	RangeNameAndValue,
	RangeValueAndName,
	RangeValueGTValue,
	RangeValueLTValue,
} from "@internal/ast";
import {CSSParser} from "@internal/css-parser/types";
import {
	parseMediaFeatureName,
	parseMediaFeatureValue,
} from "@internal/css-parser/parser/media/feature";
import {
	parseMediaFeatureComparison,
	parseMediaFeatureGT,
} from "@internal/css-parser/parser/media/comparison";
import {descriptions} from "@internal/diagnostics";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";

type MaybeValue =
	| RangeNameAndValue
	| RangeValueAndName
	| RangeValueGTValue
	| RangeValueLTValue
	| undefined;

export function assertRangeNameAndValue(
	value: MaybeValue,
	parser: CSSParser,
): value is RangeNameAndValue {
	const token = parser.getToken();
	return token.type === "Ident";
}

export function assertRangeValueGTValue(
	value: MaybeValue,
	parser: CSSParser,
): value is RangeValueGTValue {
	const token = parser.getToken();
	return token.type === "Ident" && token.value === ">";
}
export function assertRangeValueLTValue(
	value: MaybeValue,
	parser: CSSParser,
): value is RangeValueLTValue {
	const token = parser.getToken();
	return token.type === "Ident" && token.value === "<";
}

export function assertRangeValueAndName(
	value: MaybeValue,
	parser: CSSParser,
): value is RangeValueAndName {
	return parser.matchToken("RightParen");
}

export function parseMediaFeatureRange(
	parser: CSSParser,
): CSSMediaFeatureRange | undefined {
	// TODO: terminate implementation
	const start = parser.getPosition();
	let rangeValue: MaybeValue = undefined;
	// let's check if the first value is an identifier. If so, we already know what kind of range we have
	if (assertRangeNameAndValue(rangeValue, parser)) {
		const name = parseMediaFeatureName(parser);
		const comparison = parseMediaFeatureComparison(parser);
		const value = parseMediaFeatureValue(parser);

		if (name && comparison && value) {
			rangeValue = [name, comparison, value];
		}
	} else {
		let maybeComparison: CSSMediaFeatureComparison | undefined = undefined;

		// first value
		const value = parseMediaFeatureValue(parser);

		// if (assertRangeValueGTValue(rangeValue, parser)) {
		// possible comparison
		const startComparison = parser.getPosition();
		const firstGT = parseMediaFeatureGT(parser);
		if (firstGT) {
			maybeComparison = parser.finishNode(
				startComparison,
				{
					type: "CSSMediaFeatureComparison",
					value: firstGT,
				},
			);
		}
		// name
		const name = parseMediaFeatureName(parser);

		// skip possible comments and spaces
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		if (assertRangeValueAndName(rangeValue, parser)) {
			if (value && maybeComparison && name) {
				rangeValue = [value, maybeComparison, name] as RangeValueAndName;
				return parser.finishNode(
					start,
					{
						type: "CSSMediaFeatureRange",
						value: rangeValue,
					},
				);
			}
		}
		// rangeValue = []
		// }
	}

	if (rangeValue) {
		return parser.finishNode(
			start,
			{
				type: "CSSMediaFeatureRange",
				value: rangeValue,
			},
		);
	}

	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.MEDIA_QUERY_MALFORMED_RANGE,
		start,
		end: parser.getPosition(),
	});

	return undefined;
}
