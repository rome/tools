import {
	CSSAtRule,
	CSSDeclaration,
	CSSRule,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSBlock extends NodeBaseWithComments {
	readonly type: "CSSBlock";
	readonly startingTokenValue?: string;
	readonly value?: Array<
		AnyCSSValue | CSSRule | CSSAtRule | CSSDeclaration | undefined
	>;
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
