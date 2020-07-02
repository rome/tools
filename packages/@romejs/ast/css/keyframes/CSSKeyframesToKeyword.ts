import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesToKeyword = NodeBaseWithComments & {
	type: "CSSKeyframesToKeyword";
};

export const cssKeyframesToKeyword = createBuilder<CSSKeyframesToKeyword>(
	"CSSKeyframesToKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
