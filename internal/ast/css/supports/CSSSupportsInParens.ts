import {
	CSSSupportsCondition,
	CSSSupportsFeature,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSSupportsInParens extends NodeBaseWithComments {
	readonly type: "CSSSupportsInParens";
	readonly prefix?: string | undefined;
	readonly value: CSSSupportsCondition | CSSSupportsFeature;
}

export const cssSupportsInParens = createBuilder<CSSSupportsInParens>(
	"CSSSupportsInParens",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
