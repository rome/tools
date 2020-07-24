import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSBlendModeType extends NodeBaseWithComments {
	type: "CSSBlendModeType";
	// TODO
}

export const cssBlendModeType = createBuilder<CSSBlendModeType>(
	"CSSBlendModeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
