import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSDimensionType extends NodeBaseWithComments {
	type: "CSSDimensionType";
	// TODO
}

export const cssDimensionType = createBuilder<CSSDimensionType>(
	"CSSDimensionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
