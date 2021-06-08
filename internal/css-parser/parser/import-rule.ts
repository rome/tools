// https://www.w3.org/TR/css-cascade-4/#conditional-import
import {CSSParser} from "@internal/css-parser/types";
import {CSSImport, CSSSelector} from "@internal/ast";
import {matchToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {parseSelectors} from "@internal/css-parser/parser/selectors";
import {parseDeclarationBlock} from "@internal/css-parser/parser/declaration";

export function parseImportRule(parser: CSSParser): CSSImport | undefined {
	const start = parser.getPosition();
	let prelude: CSSSelector[] = [];
	while (!matchToken(parser, "EOF")) {
		if (matchToken(parser, "LeftCurlyBracket")) {
			return parser.finishNode(
				start,
				{
					type: "CSSImport",
					prelude,
					block: parseDeclarationBlock({parser}),
				},
			);
		}
		prelude = parseSelectors(parser);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.UNEXPECTED_TOKEN,
		token: parser.getToken(),
	});
	return undefined;
}
