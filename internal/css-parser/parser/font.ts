import {CSSParser} from "@internal/css-parser/types";
import {CSSFontFace} from "@internal/ast/css/font/CSSFontFace";
import {skipWhitespaces} from "@internal/css-parser/tokenizer";
import {parseDeclarationBlock} from "@internal/css-parser/parser/declaration";
import {descriptions} from "@internal/diagnostics";

export function parseFontFace(parser: CSSParser): CSSFontFace | undefined {
	const start = parser.getPosition();
	const fontFaceToken = parser.getPreviousToken();
	let sourceFound: boolean = false;
	skipWhitespaces(parser);

	const block = parseDeclarationBlock({
		parser,
		onAtDeclaration: (token) => {
			if (token.value === "src") {
				sourceFound = true;
			}
			return true;
		},
	});

	if (!sourceFound) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.AT_FONT_FACE_MISSING_SRC,
			token: fontFaceToken,
		});
		return undefined;
	}

	if (block) {
		return parser.finishNode(
			start,
			{
				type: "CSSFontFace",
				value: block,
			},
		);
	}

	return undefined;
}
