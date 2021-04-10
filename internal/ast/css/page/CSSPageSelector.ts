import {CSSPseudoPage, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSPageSelector extends NodeBaseWithComments {
	readonly type: "CSSPageSelector";
	readonly ident?: string;
	readonly pseudo?: CSSPseudoPage;
}

export const cssPageSelector = createBuilder<CSSPageSelector>(
	"CSSPageSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			pseudo: true,
		},
	},
);
