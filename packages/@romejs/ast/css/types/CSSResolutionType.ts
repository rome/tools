import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSResolutionType = NodeBaseWithComments & {
	type: "CSSResolutionType";
};

export const cssResolutionType = createBuilder<CSSResolutionType>(
	"CSSResolutionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
