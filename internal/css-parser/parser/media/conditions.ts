import {CSSParser} from "@internal/css-parser/types";
import {CSSMediaAnd, CSSMediaNot, CSSMediaOr} from "@internal/ast";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseMediaInParens} from "@internal/css-parser/parser/media/inParens";
import {descriptions} from "@internal/diagnostics";


const CONDITIONS = ["not" , "and" , "or"]

export function isCondition(value: string) {
	return CONDITIONS.includes(value);
}


export function parseMediaNot(parser: CSSParser): CSSMediaNot | undefined {
	const start = parser.getPosition();
	const token = parser.getToken();

	if (token.type === "Ident" && token.value === "not") {
		// move forward
		parser.nextToken();
		// remove white spaces between keyword and next important token
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		const value = parseMediaInParens(parser);
		if (value) {
			return parser.finishNode(
				start,
				{
					type: "CSSMediaNot",
					value,
				},
			);
		}
	}

	return undefined;
}

export function parseMediaAnd(parser: CSSParser): CSSMediaAnd | undefined {
	const value = parseMediaInParens(parser);
	if (!value) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.MALFORMED_MEDIA_QUERY,
			token: parser.getToken()
		})
	}
	return undefined;
}

export function parseMediaOr(parser: CSSParser): CSSMediaOr | undefined {
	return undefined;
}
