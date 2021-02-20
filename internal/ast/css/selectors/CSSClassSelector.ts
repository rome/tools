import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSClassSelector extends NodeBaseWithComments {
	readonly type: "CSSClassSelector";
	readonly value: string;
}

export const cssClassSelector = createBuilder<CSSClassSelector>(
	"CSSClassSelector",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
