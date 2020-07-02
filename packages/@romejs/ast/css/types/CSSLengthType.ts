import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSLengthType = NodeBaseWithComments & {
	type: "CSSLengthType";
};

export const cssLengthType = createBuilder<CSSLengthType>(
	"CSSLengthType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
