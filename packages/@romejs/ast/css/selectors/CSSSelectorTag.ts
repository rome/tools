import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorTag = JSNodeBase & {
	type: "CSSSelectorTag";
};

export const cssSelectorTag = createBuilder<CSSSelectorTag>(
	"CSSSelectorTag",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
