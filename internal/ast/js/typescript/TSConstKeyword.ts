import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSConstKeyword extends NodeBaseWithComments {
	readonly type: "TSConstKeyword";
}

export const tsConstKeyword = createBuilder<TSConstKeyword>(
	"TSConstKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
