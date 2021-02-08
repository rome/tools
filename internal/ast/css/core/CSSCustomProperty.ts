import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCustomProperty extends NodeBaseWithComments {
	readonly type: "CSSCustomProperty";
	readonly value: string;
}

export const cssCustomProperty = createBuilder<CSSCustomProperty>(
	"CSSCustomProperty",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
