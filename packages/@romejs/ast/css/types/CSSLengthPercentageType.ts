import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSLengthPercentageType = NodeBaseWithComments & {
	type: "CSSLengthPercentageType";
};

export const cssLengthPercentageType = createBuilder<CSSLengthPercentageType>(
	"CSSLengthPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
