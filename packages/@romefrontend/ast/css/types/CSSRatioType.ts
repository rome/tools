import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSRatioType extends NodeBaseWithComments {
	type: "CSSRatioType";
	// TODO
}

export const cssRatioType = createBuilder<CSSRatioType>(
	"CSSRatioType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
