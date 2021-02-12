import {CSSPercentage, CSSRaw, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSKeyframeSelector extends NodeBaseWithComments {
	readonly type: "CSSKeyframeSelector";
	readonly value: CSSRaw | CSSPercentage;
}

export const cssKeyframeSelector = createBuilder<CSSKeyframeSelector>(
	"CSSKeyframeSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
