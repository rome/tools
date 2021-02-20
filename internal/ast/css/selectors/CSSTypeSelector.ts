import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSTypeSelector extends NodeBaseWithComments {
	readonly type: "CSSTypeSelector";
	readonly value: string;
}

export const cssTypeSelector = createBuilder<CSSTypeSelector>(
	"CSSTypeSelector",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
