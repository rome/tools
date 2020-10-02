import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueFloat extends NodeBaseWithComments {
	readonly type: "TomlValueFloat";
}

export const TomlValueFloat = createBuilder<TomlValueFloat>(
	"TomlValueFloat",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
