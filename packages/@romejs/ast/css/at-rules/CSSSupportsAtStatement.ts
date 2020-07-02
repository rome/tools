import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSupportsAtStatement = NodeBaseWithComments & {
	type: "CSSSupportsAtStatement";
};

export const cssSupportsAtStatement = createBuilder<CSSSupportsAtStatement>(
	"CSSSupportsAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
