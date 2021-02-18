import {CSSMediaCondition, CSSMediaConditionWithoutOr, CSSMediaType, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaQuery extends NodeBaseWithComments {
	readonly type: "CSSMediaQuery";
	readonly condition?: "not" | "only";
	readonly conditionWithoutOr?: CSSMediaConditionWithoutOr;
	readonly value: CSSMediaCondition | CSSMediaType
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
