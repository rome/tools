// https://www.w3.org/TR/css-cascade-4/#conditional-import
import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSAtImport} from "@internal/ast";
import {parseKeyframe} from "@internal/css-parser/parser/keyframe";
import {
	OnAtDeclaration,
	OnAtKeyword,
	parseDeclarationBlock,
} from "@internal/css-parser/parser/declaration";
import {parseMediaList} from "@internal/css-parser/parser/media";
import {parseAtSupports} from "@internal/css-parser/parser/supports";
import {parseFontFace} from "@internal/css-parser/parser/font";
import {parseAtPage} from "@internal/css-parser/parser/page";

interface ParseAtImport {
	parser: CSSParser;
	onAtKeyword?: OnAtKeyword;
	onAtDeclaration?: OnAtDeclaration;
}

export function parseAtImport(
	{parser, onAtDeclaration, onAtKeyword}: ParseAtImport,
): CSSAtImport {
	const start = parser.getPosition();
	const previousToken = parser.getToken() as Tokens["AtKeyword"];
	const token = parser.expectToken("AtKeyword");
	const prelude: AnyCSSValue[] = [];
	const name = token.value;
	let block = undefined;
	while (true) {
		if (matchToken(parser, "Semi")) {
			break;
		}
		if (matchToken(parser, "EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_AT_RULE,
				token: parser.getToken(),
			});
			break;
		}
		if (previousToken.value === "media") {
			block = parseMediaList(parser);
			break;
		}
		if (previousToken.value === "keyframes") {
			block = parseKeyframe(parser);
			break;
		}

		if (previousToken.value === "font-face") {
			block = parseFontFace(parser);
			break;
		}

		if (previousToken.value === "page") {
			block = parseAtPage(parser);
			break;
		}

		if (previousToken.value === "supports") {
			const value = parseAtSupports(parser);
			if (value) {
				prelude.push(value);
			}
		}
		if (matchToken(parser, "LeftCurlyBracket")) {
			block = parseDeclarationBlock({
				parser,
				parentAtKeywordToken: previousToken,
				onAtDeclaration,
				onAtKeyword,
			});
			break;
		}
		const parsedValue = parseComponentValue(parser);
		if (parsedValue) {
			prelude.push(parsedValue);
		}
	}
	return parser.finishNode(
		start,
		{
			type: "CSSAtRule",
			name,
			prelude,
			block,
		},
	);
}
