import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorClass = JSNodeBase & {
	type: "CSSSelectorClass";
};

export const cssSelectorClass = createBuilder<CSSSelectorClass>(
	"CSSSelectorClass",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
