import {AnyCSSRuleStatement, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// @media
export type CSSMediaAtStatement = NodeBaseWithComments & {
	type: "CSSMediaAtStatement";
	body: Array<AnyCSSRuleStatement>;
	// TODO media query list
};

export const cssMediaAtStatement = createBuilder<CSSMediaAtStatement>(
	"CSSMediaAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
