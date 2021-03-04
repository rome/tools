import {CSSMediaInParens, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaAnd extends NodeBaseWithComments {
	readonly type: "CSSMediaAnd";
	readonly value: [CSSMediaInParens, CSSMediaInParens];
}

export const cssMediaAnd = createBuilder<CSSMediaAnd>(
	"CSSMediaAnd",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
