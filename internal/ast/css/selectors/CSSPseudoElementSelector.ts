import {NodeBaseWithComments} from "@internal/ast";
import {AnyCSSValue} from "@internal/css-parser/types";
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
