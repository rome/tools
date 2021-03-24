import {AnyCSSValue, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSPseudoClassSelector extends NodeBaseWithComments {
	readonly type: "CSSPseudoClassSelector";
	readonly value: string;
	readonly params?: AnyCSSValue[];
}

export const cssPseudoClassSelector = createBuilder<CSSPseudoClassSelector>(
	"CSSPseudoClassSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
