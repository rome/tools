import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorId = NodeBaseWithComments & {
	type: "CSSSelectorId";
};

export const cssSelectorId = createBuilder<CSSSelectorId>(
	"CSSSelectorId",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
