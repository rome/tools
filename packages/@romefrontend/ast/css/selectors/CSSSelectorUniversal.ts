import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// *
export interface CSSSelectorUniversal extends NodeBaseWithComments {
	type: "CSSSelectorUniversal";
}

export const cssSelectorUniversal = createBuilder<CSSSelectorUniversal>(
	"CSSSelectorUniversal",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
