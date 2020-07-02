import {AnyCSSSelector, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// foo > bar
// foo bar
// foo + bar
export type CSSSelectorCombinator = NodeBaseWithComments & {
	type: "CSSSelectorCombinator";
	kind:
		| "descendant"
		| "child"
		| "general-sibling"
		| "adjacent-sibling"
		| "column";
	left: AnyCSSSelector;
	right: AnyCSSSelector;
};

export const cssSelectorCombinator = createBuilder<CSSSelectorCombinator>(
	"CSSSelectorCombinator",
	{
		bindingKeys: {},
		visitorKeys: {
			left: true,
			right: true,
		},
	},
);
