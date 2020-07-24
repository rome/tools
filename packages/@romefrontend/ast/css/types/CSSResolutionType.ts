import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSResolutionType extends NodeBaseWithComments {
	type: "CSSResolutionType";
	// TODO
}

export const cssResolutionType = createBuilder<CSSResolutionType>(
	"CSSResolutionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
