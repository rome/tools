import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlText extends NodeBaseWithComments {
	readonly type: "TomlText";
	readonly value: string;
}

export const TomlText = createBuilder<TomlText>(
	"TomlText",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
