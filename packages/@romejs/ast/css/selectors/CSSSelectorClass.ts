import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorClass = NodeBaseWithComments & {
	type: "CSSSelectorClass";
};

export const cssSelectorClass = createBuilder<CSSSelectorClass>(
	"CSSSelectorClass",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
