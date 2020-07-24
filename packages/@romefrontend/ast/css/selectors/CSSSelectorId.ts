import {CSSIdentifierType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// #foo
export interface CSSSelectorId extends NodeBaseWithComments {
	type: "CSSSelectorId";
	id: CSSIdentifierType;
}

export const cssSelectorId = createBuilder<CSSSelectorId>(
	"CSSSelectorId",
	{
		bindingKeys: {},
		visitorKeys: {
			id: true,
		},
	},
);
