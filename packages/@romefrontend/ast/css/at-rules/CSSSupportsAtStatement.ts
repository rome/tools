import {AnyCSSRuleStatement, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// @supports
export type CSSSupportsAtStatement = NodeBaseWithComments & {
	type: "CSSSupportsAtStatement";
	body: Array<AnyCSSRuleStatement>;
	// TODO supports condition
};

export const cssSupportsAtStatement = createBuilder<CSSSupportsAtStatement>(
	"CSSSupportsAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
