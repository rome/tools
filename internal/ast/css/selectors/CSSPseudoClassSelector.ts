import {NodeBaseWithComments} from "@internal/ast";
import {AnyCSSValue} from "@internal/css-parser/types";
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
