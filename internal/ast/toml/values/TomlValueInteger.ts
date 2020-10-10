import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueInteger extends NodeBaseWithComments {
	readonly type: "TomlValueInteger";
	readonly value: string;
}

export const tomlValueInteger = createBuilder<TomlValueInteger>(
	"TomlValueInteger",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
