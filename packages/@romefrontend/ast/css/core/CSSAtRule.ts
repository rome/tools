import {CSSBlock, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export type CSSAtRule = NodeBaseWithComments & {
	type: "CSSAtRule";
	name: string;
	prelude: Array<AnyCSSValue>;
	block: CSSBlock | null;
};

export const cssAtRule = createBuilder<CSSAtRule>(
	"CSSAtRule",
	{
		bindingKeys: {},
		visitorKeys: {
			block: true,
			prelude: true,
		},
	},
);
