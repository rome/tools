import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSPseudoClassSelector extends NodeBaseWithComments {
	readonly type: "CSSPseudoClassSelector";
	readonly value: string;
}

export const cssPseudoClassSelector = createBuilder<CSSPseudoClassSelector>(
	"CSSPseudoClassSelector",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
