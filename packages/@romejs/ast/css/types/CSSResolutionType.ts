import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSResolutionType = JSNodeBase & {
	type: "CSSResolutionType";
};

export const cssResolutionType = createBuilder<CSSResolutionType>(
	"CSSResolutionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
