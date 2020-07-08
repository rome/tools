import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesFromKeyword = NodeBaseWithComments & {
	type: "CSSKeyframesFromKeyword";
};

export const cssKeyframesFromKeyword = createBuilder<CSSKeyframesFromKeyword>(
	"CSSKeyframesFromKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
