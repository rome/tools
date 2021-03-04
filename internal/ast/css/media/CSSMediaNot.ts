import {CSSMediaInParens, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaNot extends NodeBaseWithComments {
	readonly type: "CSSMediaNot";
	readonly value: CSSMediaInParens;
}

export const cssMediaNot = createBuilder<CSSMediaNot>(
	"CSSMediaNot",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
