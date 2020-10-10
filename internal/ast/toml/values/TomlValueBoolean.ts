import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueBoolean extends NodeBaseWithComments {
	readonly type: "TomlValueBoolean";
}

export const tomlValueBoolean = createBuilder<TomlValueBoolean>(
	"TomlValueBoolean",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
