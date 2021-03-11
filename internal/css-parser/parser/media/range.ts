import {
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
	return false;
}

export function parseMediaFeatureRange(
	parser: CSSParser,
): CSSMediaFeatureRange | undefined {
	const start = parser.getPosition();
	let rangeValue: MaybeValue = undefined;

	if (assertRangeNameAndValue(rangeValue, parser)) {
		const name = parseMediaFeatureName(parser);
		const comparison = parseMediaFeatureComparison(parser);
		const value = parseMediaFeatureValue(parser);

		if (name && comparison && value) {
			rangeValue = [name, comparison, value];
		}
	} else {
		const value = parseMediaFeatureValue(parser);

		if (assertRangeValueGTValue(rangeValue, parser)) {
			const firstGT = parseMediaFeatureGT(parser);
		}
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
		description: descriptions.CSS_PARSER.MALFORMED_MEDIA_QUERY,
		start,
		end: parser.getPosition(),
	});

	return undefined;
}
