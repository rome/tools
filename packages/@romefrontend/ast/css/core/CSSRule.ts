import {CSSBlock, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export type CSSRule = NodeBaseWithComments & {
	type: "CSSRule";
	prelude: Array<AnyCSSValue>;
	block: CSSBlock | null;
};

export const cssRule = createBuilder<CSSRule>(
	"CSSRule",
	{
		bindingKeys: {},
		visitorKeys: {
			prelude: true,
			block: true,
		},
	},
);
