import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorTag = NodeBaseWithComments & {
	type: "CSSSelectorTag";
};

export const cssSelectorTag = createBuilder<CSSSelectorTag>(
	"CSSSelectorTag",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
