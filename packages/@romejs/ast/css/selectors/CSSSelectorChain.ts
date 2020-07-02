import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorChain = JSNodeBase & {
	type: "CSSSelectorChain";
};

export const cssSelectorChain = createBuilder<CSSSelectorChain>(
	"CSSSelectorChain",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
