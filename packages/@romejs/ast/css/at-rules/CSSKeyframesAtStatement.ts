import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesAtStatement = JSNodeBase & {
	type: "CSSKeyframesAtStatement";
};

export const cssKeyframesAtStatement = createBuilder<CSSKeyframesAtStatement>(
	"CSSKeyframesAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
