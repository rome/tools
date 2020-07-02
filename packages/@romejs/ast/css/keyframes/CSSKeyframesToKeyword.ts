import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesToKeyword = JSNodeBase & {
	type: "CSSKeyframesToKeyword";
};

export const cssKeyframesToKeyword = createBuilder<CSSKeyframesToKeyword>(
	"CSSKeyframesToKeyword",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
