import {
	AnyCSSSelector,
	CSSSelectorTag,
	NodeBaseWithComments,
} from "../../index";
import {createBuilder} from "../../utils";

// foo#bar.yes
export interface CSSSelectorChain extends NodeBaseWithComments {
	type: "CSSSelectorChain";
	// Can only be one per chain and must appear at the start
	tagName: undefined | CSSSelectorTag;
	selectors: Array<Exclude<AnyCSSSelector, CSSSelectorTag | CSSSelectorChain>>;
}

export const cssSelectorChain = createBuilder<CSSSelectorChain>(
	"CSSSelectorChain",
	{
		bindingKeys: {},
		visitorKeys: {
			tagName: true,
			selectors: true,
		},
	},
);
