import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyTomlValue} from "@internal/ast/toml/unions";

export interface TomlValueArray extends NodeBaseWithComments {
	readonly type: "TomlValueArray";
	readonly value: Array<AnyTomlValue>;
}

export const tomlValueArray = createBuilder<TomlValueArray>(
	"TomlValueArray",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
