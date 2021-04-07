import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export type CSSPseudoPageValue = "left" | "right" | "first" | "blank";

export interface CSSPseudoPage extends NodeBaseWithComments {
	readonly type: "CSSPseudoPage";
	readonly value: CSSPseudoPageValue;
}

export const cssPseudoPage = createBuilder<CSSPseudoPage>(
	"CSSPseudoPage",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
