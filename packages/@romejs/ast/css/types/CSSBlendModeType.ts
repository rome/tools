import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSBlendModeType = JSNodeBase & {
	type: "CSSBlendModeType";
};

export const cssBlendModeType = createBuilder<CSSBlendModeType>(
	"CSSBlendModeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
