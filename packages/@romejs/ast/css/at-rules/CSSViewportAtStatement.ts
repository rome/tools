import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSViewportAtStatement = NodeBaseWithComments & {
	type: "CSSViewportAtStatement";
};

export const cssViewportAtStatement = createBuilder<CSSViewportAtStatement>(
	"CSSViewportAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
