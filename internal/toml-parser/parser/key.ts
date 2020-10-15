import {TomlParser} from "@internal/toml-parser/types";
import {TomlKey} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export function parseKey(parser: TomlParser): TomlKey | undefined {
	const pos = parser.getPosition();
	// keys can have single/double quotes
	parser.eatToken("DoubleQuote");
	parser.eatToken("SingleQuote");
	const textToken = parser.getToken();

	if (textToken.type === "Text") {
		parser.nextToken();

		const oneOf =
			parser.eatToken("Space") ||
			// keys can have single/double quotes
			parser.eatToken("DoubleQuote") ||
			parser.eatToken("SingleQuote");

		if (!oneOf) {
			parser.nextToken();
		}
		parser.eatToken("Space");
		if (!parser.eatToken("Equals")) {
			parser.unexpectedDiagnostic({
				description: descriptions.TOML_PARSER.NO_VALUE_FOR_KEY(textToken.value),
			});
			parser.nextToken();
			return undefined;
		}
		return parser.finishNode(
			pos,
			{
				type: "TomlKey",
				value: textToken.value,
			},
		);
	}

	return undefined;
}
