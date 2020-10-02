import {TomlParser} from "@internal/toml-parser/types";

export function parseText(parser: TomlParser) {
	const pos = parser.getPosition();
	const token = parser.getToken();
	if (token.type === "Text") {
		return parser.finishNode(
			pos,
			{
				type: "TomlText",
				value: token.value,
			},
		);
	}
	return parser.finishNode(
		pos,
		{
			type: "TomlText",
			value: "",
		},
	);
}
