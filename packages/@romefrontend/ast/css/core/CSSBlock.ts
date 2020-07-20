import {
	CSSAtRule,
	CSSDeclaration,
	CSSRule,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export type CSSBlock = NodeBaseWithComments & {
	type: "CSSBlock";
	startingTokenValue: string | null;
	value?: Array<AnyCSSValue | CSSRule | CSSAtRule | CSSDeclaration | null>;
};

export const cssBlock = createBuilder<CSSBlock>(
	"CSSBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			startingTokenValue: true,
		},
	},
);
