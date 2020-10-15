import {TomlParser} from "@internal/toml-parser/types";
import {TomlKey, TomlKeyValue} from "@internal/ast";
import {parseText} from "@internal/toml-parser/parser/string";
import {AnyTomlValue} from "@internal/ast/toml/unions";
import {descriptions} from "@internal/diagnostics";
import {parseKey} from "@internal/toml-parser/parser/key";

export function parseKeyValue(parser: TomlParser): TomlKeyValue | undefined {
	const pos = parser.getPosition();
	let value: AnyTomlValue | undefined = undefined;
	const key: TomlKey | undefined = parseKey(parser);
	if (!key) {
		return undefined;
	}

	parser.nextToken();

	const valueToken = parser.getToken();

	switch (valueToken.type) {
		case "DoubleQuote": {
			value = parseText(parser);
			if (!value) {
				return undefined;
			}
			parser.nextToken();
			return parser.finishNode(
				pos,
				{
					type: "TomlKeyValue",
					key,
					value,
				},
			);
		}
	}

	if (!value) {
		parser.unexpectedDiagnostic({
			description: descriptions.TOML_PARSER.VALUE_NOT_RECOGNISED(key.value),
		});
		parser.nextToken();
		return undefined;
	}

	parser.nextToken();
	return undefined;
}
