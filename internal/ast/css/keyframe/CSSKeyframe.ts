import {
	CSSKeyframeBlock,
	CSSKeyframeName,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSKeyframe extends NodeBaseWithComments {
	readonly type: "CSSKeyframe";
	readonly name: CSSKeyframeName;
	readonly value: CSSKeyframeBlock[];
}

export const cssKeyframe = createBuilder<CSSKeyframe>(
	"CSSKeyframe",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			value: true,
		},
	},
);
