import {
	CSSMediaAnd,
	CSSMediaInParens,
	CSSMediaNot,
	CSSMediaOr,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type MediaAndOr = CSSMediaAnd | CSSMediaOr

export interface CSSMediaCondition extends NodeBaseWithComments {
	readonly type: "CSSMediaCondition";
	readonly value: CSSMediaNot | [CSSMediaInParens, ...MediaAndOr[]] ;
}

export const cssMediaCondition = createBuilder<CSSMediaCondition>(
	"CSSMediaCondition",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
