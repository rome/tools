import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueInlineTable extends NodeBaseWithComments {
	readonly type: "TomlValueInlineTable";
}

export const tomlValueInlineTable = createBuilder<TomlValueInlineTable>(
	"TomlValueInlineTable",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
