import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesFromKeyword = JSNodeBase & {
	type: "CSSKeyframesFromKeyword";
};

export const cssKeyframesFromKeyword = createBuilder<CSSKeyframesFromKeyword>(
	"CSSKeyframesFromKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
