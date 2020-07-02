import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesAtStatement = NodeBaseWithComments & {
	type: "CSSKeyframesAtStatement";
};

export const cssKeyframesAtStatement = createBuilder<CSSKeyframesAtStatement>(
	"CSSKeyframesAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
