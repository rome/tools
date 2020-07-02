import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSURLType = NodeBaseWithComments & {
	type: "CSSURLType";
};

export const cssurlType = createBuilder<CSSURLType>(
	"CSSURLType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
