import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyTomlValue} from "@internal/ast/toml/unions";

export interface TomlKeyValue extends NodeBaseWithComments {
	readonly type: "TomlKeyValue";
	readonly key: string;
	readonly value: AnyTomlValue;
}

export const TomlKeyValue = createBuilder<TomlKeyValue>(
	"TomlKeyValue",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
