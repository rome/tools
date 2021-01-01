import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSIdSelector extends NodeBaseWithComments {
	readonly type: "CSSIdSelector";
	readonly value: string;
}

export const cssIdSelector = createBuilder<CSSIdSelector>(
	"CSSIdSelector",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
