import {
	AnyCSSValue,
	CSSAtRule,
	CSSDeclaration,
	CSSRule,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSBlock extends NodeBaseWithComments {
	readonly type: "CSSBlock";
	readonly startingTokenValue?: string;
	readonly value?: Array<AnyCSSValue | CSSRule | CSSAtRule | CSSDeclaration>;
}
export const cssBlock = createBuilder<CSSBlock>(
	"CSSBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
