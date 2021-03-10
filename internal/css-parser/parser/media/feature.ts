import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSDimension,
	CSSMediaFeature, CSSMediaFeatureName,
	CSSMediaFeaturePlain,
	CSSMediaFeatureValue,
	CSSNumber,
	CSSString,
	CSSMediaFeatureBoolean
} from "@internal/ast";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
// import {isCondition} from "@internal/css-parser/parser/media/conditions";

export function parseMediaFeatureName(parser: CSSParser): CSSMediaFeatureName | undefined {

	const ident = parser.eatToken("Ident");
	const namePosition = parser.getPosition();
	const colon = parser.eatToken("Colon");
	if (!ident || !colon) {
		//	 TODO: error
		return undefined
	}

	return parser.finishNode(namePosition, {
		type: "CSSMediaFeatureName",
		value: ident.value
	});
}

export function parseMediaFeatureValue(parser: CSSParser ): CSSMediaFeatureValue | undefined {
	// move forward and get rid of all the white spaces
	parser.nextToken();
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();
	const start  = parser.getPosition();
	let value: CSSDimension | CSSString | CSSNumber | undefined = undefined;

	if (token.type === "Ident") {
		parser.nextToken();
		value = parser.finishNode(start, {
			type: "CSSString",
			value: token.value
		});
	} else if (token.type === "Dimension") {
		parser.nextToken();
		value = parser.finishNode(start, {
			type: "CSSDimension",
			unit: token.unit,
			value: token.value,
		});
	} else if (token.type === "Number") {
		parser.nextToken();
		value = parser.finishNode(start, {
			type: "CSSNumber",
			raw: token.raw,
			value: token.value,
		});
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.MEDIA_QUERY_FEATURE_UNEXPECTED_VALUE,

			token
		});
		parser.nextToken();
		return undefined;
	}

	return  parser.finishNode(start, {
		type: "CSSMediaFeatureValue",
		value: value
	})
}

export function parseMediaFeaturePlain(parser: CSSParser): CSSMediaFeaturePlain | undefined {
	const start = parser.getPosition();
	// remove white spaces between keyword and next important token
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const name = parseMediaFeatureName(parser);
	const value = parseMediaFeatureValue(parser);

	if (name && value) {
		return parser.finishNode(start,{
			type: "CSSMediaFeaturePlain",
			name,
			value
		})
	}
		//	 TODO: error
		return undefined

}


export function parseMediaFeature(parser: CSSParser): CSSMediaFeature | undefined {
	// TODO: implement me
	const start = parser.getPosition();
	let  value: CSSMediaFeatureBoolean | CSSMediaFeaturePlain | undefined = undefined;

	// in every case, the first token must but an Ident
	const startToken = readToken(parser, "Ident") as Tokens["Ident"];
	// the value of the feature can be a:
	// - plain: "(max-width: 600px)", "(hover: hover)"
	// - boolean: "(color)"
	//
	// we now remove possible white spaces
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	const nextToken = parser.getToken();

	// if we have a right parenthesis, it means we have a boolean
	if (nextToken.type === "RightParen") {
		value = parser.finishNode(start, {
			type: "CSSMediaFeatureBoolean",
			value: startToken.value,

		})
	} else if (nextToken.type === "Colon") {
		const name = parser.finishNode(start, {
			type: "CSSMediaFeatureName",
			value: startToken.value
		})
		const featureValue = parseMediaFeatureValue(parser)
		if (featureValue) {

			value = parser.finishNode(start, {
				type: "CSSMediaFeaturePlain",
				value: featureValue,
				name
			})
		}
	}

	// if (isCondition(token.value)) {
	//
	// } else {
	// 	const value =
	// }

	if (value) {
		console.log('token before media feature', parser.getToken())
		return parser.finishNode(start, {
			type: "CSSMediaFeature",
			value
		})
	}

	return undefined;
}
