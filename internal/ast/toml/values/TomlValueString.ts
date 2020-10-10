import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueString extends NodeBaseWithComments {
	readonly type: "TomlValueString";
	readonly value: string;
}

export const tomlValueString = createBuilder<TomlValueString>(
	"TomlValueString",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
