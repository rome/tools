import {AnyCSSRuleStatement, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRoot = NodeBaseWithComments & {
	type: "CSSRoot";
	body: Array<AnyCSSRuleStatement>;
};

export const cssRoot = createBuilder<CSSRoot>(
	"CSSRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
