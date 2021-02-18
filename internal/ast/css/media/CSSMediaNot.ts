import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaNot extends NodeBaseWithComments {
	readonly type: "CSSMediaNot";
}

export const cssMediaNot = createBuilder<CSSMediaNot>(
	"CSSMediaNot",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
