import {CSSNumber, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSRatio extends NodeBaseWithComments {
	readonly type: "CSSRatio";
	readonly numerator: CSSNumber;
	readonly denominator: CSSNumber;
}

export const cssRatio = createBuilder<CSSRatio>(
	"CSSRatio",
	{
		bindingKeys: {},
		visitorKeys: {
			numerator: true,
			denominator: true,
		},
	},
);
