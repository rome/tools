import {CSSGridRepeatParams, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSRepeatFunction extends NodeBaseWithComments {
	readonly type: "CSSRepeatFunction";
	readonly name: string;
	readonly params: CSSGridRepeatParams;
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
