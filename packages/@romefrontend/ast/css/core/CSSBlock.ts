import {
	CSSAtRule,
	CSSDeclaration,
	CSSRule,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSBlock extends NodeBaseWithComments {
	type: "CSSBlock";
	startingTokenValue?: string;
	value?: Array<AnyCSSValue | CSSRule | CSSAtRule | CSSDeclaration | undefined>;
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
