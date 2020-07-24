import {AnyCSSRuleStatement, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// @document
export interface CSSDocumentAtStatement extends NodeBaseWithComments {
	type: "CSSDocumentAtStatement";
	body: Array<AnyCSSRuleStatement>;
	// TODO head properties
}

export const cssDocumentAtStatement = createBuilder<CSSDocumentAtStatement>(
	"CSSDocumentAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
		},
	},
);
