import {CSSCustomProperty, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyCSSValue} from "@internal/css-parser/types";

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
