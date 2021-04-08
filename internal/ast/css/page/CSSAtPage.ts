import {
	CSSBlock,
	CSSPageSelectorList,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSAtPage extends NodeBaseWithComments {
	readonly type: "CSSAtPage";
	readonly prelude?: CSSPageSelectorList;
	readonly block: CSSBlock;
}

export const cssAtPage = createBuilder<CSSAtPage>(
	"CSSAtPage",
	{
		bindingKeys: {},
		visitorKeys: {
			prelude: true,
			block: true,
		},
	},
);
