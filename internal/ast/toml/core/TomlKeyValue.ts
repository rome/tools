import {NodeBaseWithComments, TomlKey} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyTomlValue} from "@internal/ast/toml/unions";

export interface TomlKeyValue extends NodeBaseWithComments {
	readonly type: "TomlKeyValue";
	readonly key: TomlKey;
	readonly value: AnyTomlValue;
}

export const tomlKeyValue = createBuilder<TomlKeyValue>(
	"TomlKeyValue",
	{
		bindingKeys: {},
		visitorKeys: {
			key: true,
			value: true,
		},
	},
);
