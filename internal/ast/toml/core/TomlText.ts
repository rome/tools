import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlText extends NodeBaseWithComments {
	readonly type: "TomlText";
	readonly value: string;
}

export const tomlText = createBuilder<TomlText>(
	"TomlText",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
