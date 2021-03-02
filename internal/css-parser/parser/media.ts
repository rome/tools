import {CSSParser} from "@internal/css-parser/types";
import {CSSMediaType, ValidFeatures} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";


const SUPPORTED_FEATURES = [
	"width",
 "height",
 "device-width",
 "device-height",
 "orientation",
 "aspect-ratio",
 "device-aspect-ratio",
 "color",
 "color-index",
 "monochrome",
 "resolution",
 "scan",
 "grid",
]

function isValidFeature(value: string): value is ValidFeatures {
	return SUPPORTED_FEATURES.includes(value);
}

function parseMediaType(parser: CSSParser): CSSMediaType | undefined {
	const start = parser.getPosition();
	const token = parser.eatToken("Ident");

	if (token) {
		if (isValidFeature(token.value)) {
			return parser.finishNode(start, {
				type: "CSSMediaType",
				value: token.value
			})
		}
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.MEDIA_QUERY_UNKNOWN_MEDIA_FEATURE_VALUE(token.value, SUPPORTED_FEATURES),
			token: parser.getToken()
		});
		parser.nextToken();
	}


	return undefined;
}
