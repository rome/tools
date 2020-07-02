import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorCombinator = JSNodeBase & {
	type: "CSSSelectorCombinator";
};

export const cssSelectorCombinator = createBuilder<CSSSelectorCombinator>(
	"CSSSelectorCombinator",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
