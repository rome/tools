import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlKey extends NodeBaseWithComments {
	readonly type: "TomlKey";
	readonly value: string;
}

export const tomlKey = createBuilder<TomlKey>(
	"TomlKey",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
