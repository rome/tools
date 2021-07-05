import {CSSGridRepeatParams, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSRepeatFunction extends NodeBaseWithComments {
	readonly type: "CSSRepeatFunction";
	readonly params: CSSGridRepeatParams;
	readonly name: string;
}

export const cssRepeatFunction = createBuilder<CSSRepeatFunction>(
	"CSSRepeat",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
