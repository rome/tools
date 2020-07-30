import {NodeBaseWithComments} from "@romefrontend/ast";
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
