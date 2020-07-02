import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorChain = NodeBaseWithComments & {
	type: "CSSSelectorChain";
};

export const cssSelectorChain = createBuilder<CSSSelectorChain>(
	"CSSSelectorChain",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
