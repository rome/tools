import {CSSSupportsInParens, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSSupportsCondition extends NodeBaseWithComments {
	readonly type: "CSSSupportsCondition";
	readonly value: CSSSupportsInParens[];
}

export const cssSupportsCondition = createBuilder<CSSSupportsCondition>(
	"CSSSupportsCondition",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
