import {TomlParser} from "@internal/toml-parser/types";
import {TomlValueString} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";

export function parseText(parser: TomlParser): TomlValueString | undefined {
	const pos = parser.getPosition();
	const token = parser.eatToken("DoubleQuote");
	if (token && token.type === "Text") {
		parser.nextToken();
		const quote = parser.eatToken("DoubleQuote");
		if (!quote) {
			parser.unexpectedDiagnostic({
				description: descriptions.TOML_PARSER.UNCLOSED_VALUE(token.value),
			});
			return undefined;
		}

		return parser.finishNode(
			pos,
			{
				type: "TomlValueString",
				value: token.value,
			},
		);
	}
	return parser.finishNode(
		pos,
		{
			type: "TomlValueString",
			value: "",
		},
	);
}
