import {CSSParser} from "@internal/css-parser/types";
import {CSSMediaType, CSSMediaValidType} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";

const VALID_MEDIA_TYPES = ["all", "print", "screen"];

const DEPRECATED_MEDIA_TYPES = [
	"tty",
	"tv",
	"projection",
	"handheld",
	"braille",
	"embossed",
	"aural",
	"speech",
];

function isValidType(value: string): value is CSSMediaValidType {
	return VALID_MEDIA_TYPES.includes(value);
}

function isDeprecatedType(value: string): boolean {
	return DEPRECATED_MEDIA_TYPES.includes(value);
}

export function parseMediaType(parser: CSSParser): CSSMediaType | undefined {
	// read white spaces and comments
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const start = parser.getPosition();
	const token = parser.getToken();

	if (token.type === "Ident") {
		if (isValidType(token.value)) {
			parser.nextToken();
			return parser.finishNode(
				start,
				{
					type: "CSSMediaType",
					value: token.value,
				},
			);
		}
		if (isDeprecatedType(token.value)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.MEDIA_QUERY_DEPRECATED_MEDIA_TYPE(
					token.value,
				),
				token: parser.getToken(),
			});
		} else {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.MEDIA_QUERY_UNKNOWN_MEDIA_TYPES(
					token.value,
					VALID_MEDIA_TYPES,
				),
				token: parser.getToken(),
			});
		}
		parser.nextToken();
	}
	return undefined;
}
