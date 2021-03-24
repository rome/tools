import {AnyCSSValue, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSPseudoElementSelector extends NodeBaseWithComments {
	readonly type: "CSSPseudoElementSelector";
	readonly value: string;
	readonly params?: AnyCSSValue[];
}

export const cssPseudoElementSelector = createBuilder<CSSPseudoElementSelector>(
	"CSSPseudoElementSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
