import {
	CSSFunction,
	CSSMediaCondition,
	CSSMediaFeature,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaInParens extends NodeBaseWithComments {
	readonly type: "CSSMediaInParens";
	readonly value: CSSMediaFeature | CSSMediaCondition | CSSFunction;
}

export const cssMediaInParens = createBuilder<CSSMediaInParens>(
	"CSSMediaInParens",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
