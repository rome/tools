import {CSSRaw, CSSString, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSKeyframeName extends NodeBaseWithComments {
	readonly type: "CSSKeyframeName";
	readonly value: CSSRaw | CSSString;
}

export const cssKeyframeName = createBuilder<CSSKeyframeName>(
	"CSSKeyframeName",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
