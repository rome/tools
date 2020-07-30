import {CSSBlock, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSRule extends NodeBaseWithComments {
	readonly type: "CSSRule";
	readonly prelude: Array<AnyCSSValue>;
	readonly block?: CSSBlock;
}
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
