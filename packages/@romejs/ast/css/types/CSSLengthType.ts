import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSLengthType = JSNodeBase & {
	type: "CSSLengthType";
};

export const cssLengthType = createBuilder<CSSLengthType>(
	"CSSLengthType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
