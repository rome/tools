import {
	CSSMediaAnd,
	CSSMediaInParens,
	CSSMediaNot,
	CSSMediaOr,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaCondition extends NodeBaseWithComments {
	readonly type: "CSSMediaCondition";
	readonly value: CSSMediaAnd | CSSMediaOr | CSSMediaNot | CSSMediaInParens;
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
