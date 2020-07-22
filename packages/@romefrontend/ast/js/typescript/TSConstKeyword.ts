import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type TSConstKeyword = NodeBaseWithComments & {
	type: "TSConstKeyword";
};

export const tsConstKeyword = createBuilder<TSConstKeyword>(
	"TSConstKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
