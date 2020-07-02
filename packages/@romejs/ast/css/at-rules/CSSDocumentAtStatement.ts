import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSDocumentAtStatement = NodeBaseWithComments & {
	type: "CSSDocumentAtStatement";
};

export const cssDocumentAtStatement = createBuilder<CSSDocumentAtStatement>(
	"CSSDocumentAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
