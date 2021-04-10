import {CSSSupportsDeclaration, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSSupportsFeature extends NodeBaseWithComments {
	readonly type: "CSSSupportsFeature";
	readonly value: CSSSupportsDeclaration;
}

export const cssSupportsFeature = createBuilder<CSSSupportsFeature>(
	"CSSSupportsFeature",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
