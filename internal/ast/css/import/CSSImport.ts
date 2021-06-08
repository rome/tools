import {CSSBlock, CSSSelector, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSImport extends NodeBaseWithComments {
	readonly type: "CSSImport";
	readonly prelude: CSSSelector[];
	readonly block?: CSSBlock;
}

export const cssImport = createBuilder<CSSImport>(
	"CSSImport",
	{
		bindingKeys: {},
		visitorKeys: {
			prelude: true,
			block: true,
		},
	},
);
