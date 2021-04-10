import {CSSDeclaration, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSSupportsDeclaration extends NodeBaseWithComments {
	readonly type: "CSSSupportsDeclaration";
	readonly value: CSSDeclaration;
}

export const cssSupportsDeclaration = createBuilder<CSSSupportsDeclaration>(
	"CSSSupportsDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
