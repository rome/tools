import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorAttribute = JSNodeBase & {
	type: "CSSSelectorAttribute";
};

export const cssSelectorAttribute = createBuilder<CSSSelectorAttribute>(
	"CSSSelectorAttribute",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
