import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSMediaAtStatement = NodeBaseWithComments & {
	type: "CSSMediaAtStatement";
};

export const cssMediaAtStatement = createBuilder<CSSMediaAtStatement>(
	"CSSMediaAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
