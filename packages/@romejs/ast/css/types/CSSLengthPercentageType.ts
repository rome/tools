import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSLengthPercentageType = JSNodeBase & {
	type: "CSSLengthPercentageType";
};

export const cssLengthPercentageType = createBuilder<CSSLengthPercentageType>(
	"CSSLengthPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
