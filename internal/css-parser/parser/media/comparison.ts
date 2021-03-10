import {CSSParser} from "@internal/css-parser/types";
import {
	CSSMediaFeatureComparison,
	CSSMediaFeatureGT,
	CSSMediaFeatureEQ,
	CSSMediaFeatureLT,
	CSSMediaCondition
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export function parseMediaFeatureGT(parser: CSSParser): CSSMediaFeatureGT | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Ident" && token.value === ">") {
			const equalToken = parser.nextToken();
			const hasEqual = equalToken.type === "Ident" && equalToken.value === "+"
			return parser.finishNode(start, {
				type: "CSSMediaFeatureGT",
				hasEqual
			})
	}
	return undefined
}

export function parseMediaFeatureLT(parser: CSSParser): CSSMediaFeatureLT | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Ident" && token.value === "<") {
			const equalToken = parser.nextToken();
			const hasEqual = equalToken.type === "Ident" && equalToken.value === "+"
			return parser.finishNode(start, {
				type: "CSSMediaFeatureLT",
				hasEqual
			})
	}
	return undefined
}

export function parseMediaFeatureEQ(parser: CSSParser): CSSMediaFeatureEQ | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();

	if (token.type === "Ident" &&  token.value === "=") {

	return parser.finishNode(start, {
		type: "CSSMediaFeatureEQ",
	})
	}
	return undefined
}

export function parseMediaFeatureComparison(parser: CSSParser): CSSMediaFeatureComparison | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();
	let value:  CSSMediaFeatureGT | CSSMediaFeatureEQ | CSSMediaFeatureLT | undefined = undefined;

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
		return parser.finishNode(start, {
			type: "CSSMediaFeatureComparison",
			value
		})
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.MEDIA_QUERY_EXPECTED_COMPARISON,
		token
	})


	return undefined;
}


export function parseMediaCondition(parser: CSSParser): CSSMediaCondition | undefined {
	return undefined
}
