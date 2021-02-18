import {CSSMediaQuery, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaQueryList extends NodeBaseWithComments {
	readonly type: "CSSMediaQueryList";
	readonly value: [CSSMediaQuery]
}

export const cssMediaQueryList = createBuilder<CSSMediaQueryList>(
	"CSSMediaQueryList",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true
		},
	},
);
