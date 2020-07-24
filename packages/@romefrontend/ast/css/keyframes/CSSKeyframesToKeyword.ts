import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSKeyframesToKeyword extends NodeBaseWithComments {
	type: "CSSKeyframesToKeyword";
}

export const cssKeyframesToKeyword = createBuilder<CSSKeyframesToKeyword>(
	"CSSKeyframesToKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
