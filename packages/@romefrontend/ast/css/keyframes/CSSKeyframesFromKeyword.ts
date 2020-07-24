import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSKeyframesFromKeyword extends NodeBaseWithComments {
	type: "CSSKeyframesFromKeyword";
}

export const cssKeyframesFromKeyword = createBuilder<CSSKeyframesFromKeyword>(
	"CSSKeyframesFromKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
