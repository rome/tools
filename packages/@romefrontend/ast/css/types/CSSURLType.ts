import {CSSStringType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSURLType extends NodeBaseWithComments {
	type: "CSSURLType";
	url: CSSStringType;
}

export const cssurlType = createBuilder<CSSURLType>(
	"CSSURLType",
	{
		bindingKeys: {},
		visitorKeys: {
			url: true,
		},
	},
);
