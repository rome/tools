import {
	CSSMediaAnd,
	CSSMediaInParens,
	CSSMediaNot,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type CSSMediaConditionWithoutOrWithParens =
	| CSSMediaInParens
	| [CSSMediaInParens, ...CSSMediaAnd[]];

export interface CSSMediaConditionWithoutOr extends NodeBaseWithComments {
	readonly type: "CSSMediaConditionWithoutOr";
	readonly value: CSSMediaNot | CSSMediaConditionWithoutOrWithParens;
}

export const cssMediaConditionWithoutOr = createBuilder<CSSMediaConditionWithoutOr>(
	"CSSMediaConditionWithoutOr",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
