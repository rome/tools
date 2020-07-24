import {AnyCSSRuleStatement, NodeBaseWithComments, RootBase} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSRoot extends NodeBaseWithComments,
RootBase {
	type: "CSSRoot";
	body: Array<AnyCSSRuleStatement>;
}

export const cssRoot = createBuilder<CSSRoot>(
	"CSSRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
			comments: true,
		},
	},
);
