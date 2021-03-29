import {
	CSSAtRule,
	CSSDeclaration,
	CSSKeyframeSelector,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSKeyframeBlock extends NodeBaseWithComments {
	readonly type: "CSSKeyframeBlock";
	readonly name: CSSKeyframeSelector;
	readonly value: Array<CSSAtRule | CSSDeclaration>;
}

export const cssKeyframeBlock = createBuilder<CSSKeyframeBlock>(
	"CSSKeyframeBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			value: true,
		},
	},
);
