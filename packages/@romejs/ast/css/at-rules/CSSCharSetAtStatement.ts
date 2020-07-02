import {CSSStringType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// @charset "UTF-8";
export type CSSCharSetAtStatement = NodeBaseWithComments & {
	type: "CSSCharSetAtStatement";
	charset: CSSStringType;
};

export const cssCharSetAtStatement = createBuilder<CSSCharSetAtStatement>(
	"CSSCharSetAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			charset: true,
		},
	},
);
