import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSCharSetAtStatement = NodeBaseWithComments & {
	type: "CSSCharSetAtStatement";
};

export const cssCharSetAtStatement = createBuilder<CSSCharSetAtStatement>(
	"CSSCharSetAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
