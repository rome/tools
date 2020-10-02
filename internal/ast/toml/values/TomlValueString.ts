import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueString extends NodeBaseWithComments {
	readonly type: "TomlValueString";
	readonly value: string;
}

export const TomlValueString = createBuilder<TomlValueString>(
	"TomlValueString",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
