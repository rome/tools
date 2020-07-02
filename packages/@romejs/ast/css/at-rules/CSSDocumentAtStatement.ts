import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSDocumentAtStatement = JSNodeBase & {
	type: "CSSDocumentAtStatement";
};

export const cssDocumentAtStatement = createBuilder<CSSDocumentAtStatement>(
	"CSSDocumentAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
