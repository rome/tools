import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorCombinator = NodeBaseWithComments & {
	type: "CSSSelectorCombinator";
};

export const cssSelectorCombinator = createBuilder<CSSSelectorCombinator>(
	"CSSSelectorCombinator",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
