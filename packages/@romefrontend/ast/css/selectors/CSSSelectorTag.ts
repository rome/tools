import {CSSIdentifierType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// foo
export interface CSSSelectorTag extends NodeBaseWithComments {
	type: "CSSSelectorTag";
	tagName: CSSIdentifierType;
}

export const cssSelectorTag = createBuilder<CSSSelectorTag>(
	"CSSSelectorTag",
	{
		bindingKeys: {},
		visitorKeys: {
			tagName: true,
		},
	},
);
