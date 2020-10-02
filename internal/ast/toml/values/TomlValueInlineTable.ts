import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TomlValueInlineTable extends NodeBaseWithComments {
	readonly type: "TomlValueInlineTable";
}

export const TomlValueInlineTable = createBuilder<TomlValueInlineTable>(
	"TomlValueInlineTable",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
