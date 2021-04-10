import {
	CSSMediaCondition,
	CSSMediaConditionWithoutOr,
	CSSMediaType,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type CSSMediaQueryCondition = "not" | "only" | undefined;

export interface CSSMediaQuery extends NodeBaseWithComments {
	readonly type: "CSSMediaQuery";
	readonly condition?: CSSMediaQueryCondition | undefined;
	readonly conditionWithoutOr?: CSSMediaConditionWithoutOr | undefined;
	readonly value: CSSMediaCondition | CSSMediaType;
}

export const cssMediaQuery = createBuilder<CSSMediaQuery>(
	"CSSMediaQuery",
	{
		bindingKeys: {},
		visitorKeys: {
			conditionWithoutOr: true,
			value: true,
		},
	},
);
