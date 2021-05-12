import {CSSBlock, CSSMediaQuery, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaQueryList extends NodeBaseWithComments {
	readonly type: "CSSMediaQueryList";
	readonly prelude: CSSMediaQuery[];
	readonly block: CSSBlock;
}

export const cssMediaQueryList = createBuilder<CSSMediaQueryList>(
	"CSSMediaQueryList",
	{
		bindingKeys: {},
		visitorKeys: {
			prelude: true,
			block: true,
		},
	},
);
