import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRatioType = JSNodeBase & {
	type: "CSSRatioType";
};

export const cssRatioType = createBuilder<CSSRatioType>(
	"CSSRatioType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
