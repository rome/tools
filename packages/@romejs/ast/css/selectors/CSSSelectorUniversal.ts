import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorUniversal = NodeBaseWithComments & {
	type: "CSSSelectorUniversal";
};

export const cssSelectorUniversal = createBuilder<CSSSelectorUniversal>(
	"CSSSelectorUniversal",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
