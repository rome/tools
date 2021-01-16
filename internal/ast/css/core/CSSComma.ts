import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSComma extends NodeBaseWithComments {
	readonly type: "CSSComma";
}
export const cssComma = createBuilder<CSSComma>(
	"CSSComma",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
