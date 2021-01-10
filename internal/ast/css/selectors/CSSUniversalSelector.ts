import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSUniversalSelector extends NodeBaseWithComments {
	readonly type: "CSSUniversalSelector";
}

export const cssUniversalSelector = createBuilder<CSSUniversalSelector>(
	"CSSUniversalSelector",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
