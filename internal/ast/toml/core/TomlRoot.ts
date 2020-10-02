import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlRoot extends NodeBaseWithComments {
	readonly type: "TomlRoot";
}

export const TomlRoot = createBuilder<TomlRoot>(
	"TomlRoot",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
