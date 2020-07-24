import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSLengthPercentageType extends NodeBaseWithComments {
	type: "CSSLengthPercentageType";
	// TODO
}

export const cssLengthPercentageType = createBuilder<CSSLengthPercentageType>(
	"CSSLengthPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
