import {
	AnyCSSValue,
	CSSCustomProperty,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSVarFunction extends NodeBaseWithComments {
	readonly type: "CSSVarFunction";
	readonly name: string;
	readonly params: [CSSCustomProperty, ...AnyCSSValue[]];
}

export const cssVarFunction = createBuilder<CSSVarFunction>(
	"CSSVarFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
