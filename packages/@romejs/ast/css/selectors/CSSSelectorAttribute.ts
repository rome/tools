import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorAttribute = NodeBaseWithComments & {
	type: "CSSSelectorAttribute";
};

export const cssSelectorAttribute = createBuilder<CSSSelectorAttribute>(
	"CSSSelectorAttribute",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
