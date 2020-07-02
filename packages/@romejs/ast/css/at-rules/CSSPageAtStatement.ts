import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSPageAtStatement = NodeBaseWithComments & {
	type: "CSSPageAtStatement";
};

export const cssPageAtStatement = createBuilder<CSSPageAtStatement>(
	"CSSPageAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
