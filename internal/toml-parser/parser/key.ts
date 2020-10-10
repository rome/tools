import {TomlParser} from "@internal/toml-parser/types";
import {TomlKey} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export function parseKey(parser: TomlParser): TomlKey | undefined {
	const pos = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Text") {
		parser.nextToken();
		if (!parser.eatToken("Equals")) {
			parser.unexpectedDiagnostic({
				description: descriptions.TOML_PARSER.NO_VALUE_FOR_KEY(token.value),
			});
			parser.nextToken();
			return undefined;
		}

		return parser.finishNode(
			pos,
			{
				type: "TomlKey",
				value: token.value,
			},
		);
	}

	return undefined;
}
