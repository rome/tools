import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// TODO @viewport - Only in Edge/IE, do we want this?
export interface CSSViewportAtStatement extends NodeBaseWithComments {
	type: "CSSViewportAtStatement";
}

export const cssViewportAtStatement = createBuilder<CSSViewportAtStatement>(
	"CSSViewportAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
