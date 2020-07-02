import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSDimensionType = NodeBaseWithComments & {
	type: "CSSDimensionType";
};

export const cssDimensionType = createBuilder<CSSDimensionType>(
	"CSSDimensionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
