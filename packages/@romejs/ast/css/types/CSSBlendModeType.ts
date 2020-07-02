import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSBlendModeType = NodeBaseWithComments & {
	type: "CSSBlendModeType";
	// TODO
};

export const cssBlendModeType = createBuilder<CSSBlendModeType>(
	"CSSBlendModeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
