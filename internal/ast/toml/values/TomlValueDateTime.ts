import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueDateTime extends NodeBaseWithComments {
	readonly type: "TomlValueDateTime";
}

export const TomlValueDateTime = createBuilder<TomlValueDateTime>(
	"TomlValueDateTime",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
