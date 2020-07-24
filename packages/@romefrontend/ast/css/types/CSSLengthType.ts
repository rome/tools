import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSLengthType extends NodeBaseWithComments {
	type: "CSSLengthType";
	// TODO
}

export const cssLengthType = createBuilder<CSSLengthType>(
	"CSSLengthType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
