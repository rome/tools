import {
	AnyCSSValue,
	CSSCustomProperty,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSDeclaration extends NodeBaseWithComments {
	readonly type: "CSSDeclaration";
	readonly name: string | CSSCustomProperty;
	readonly important: boolean;
	readonly value: Array<AnyCSSValue | undefined>;
}
export const cssDeclaration = createBuilder<CSSDeclaration>(
	"CSSDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
			name: true,
		},
	},
);
