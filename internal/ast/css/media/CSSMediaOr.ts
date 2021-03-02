import {CSSMediaInParens, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaOr extends NodeBaseWithComments {
	readonly type: "CSSMediaOr";
	readonly value: [CSSMediaInParens, CSSMediaInParens?];
}

export const cssMediaOr = createBuilder<CSSMediaOr>(
	"CSSMediaOr",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true
		},
	},
);
