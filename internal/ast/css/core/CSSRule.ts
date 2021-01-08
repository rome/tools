import {CSSBlock, CSSSelector, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSRule extends NodeBaseWithComments {
	readonly type: "CSSRule";
	readonly prelude: CSSSelector[];
	readonly block?: CSSBlock;
}
export const cssRule = createBuilder<CSSRule>(
	"CSSRule",
	{
		bindingKeys: {},
		visitorKeys: {
			prelude: true,
			block: true,
		},
	},
);
