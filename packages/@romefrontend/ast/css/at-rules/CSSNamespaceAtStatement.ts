import {
	CSSIdentifierType,
	CSSStringType,
	CSSURLType,
	NodeBaseWithComments,
} from "../../index";
import {createBuilder} from "../../utils";

// @namespace url(http://www.w3.org/1999/xhtml);
// @namespace svg url(http://www.w3.org/2000/svg);
export interface CSSNamespaceAtStatement extends NodeBaseWithComments {
	type: "CSSNamespaceAtStatement";
	namespace?: CSSStringType | CSSURLType;
	prefix?: CSSIdentifierType;
}

export const cssNamespaceAtStatement = createBuilder<CSSNamespaceAtStatement>(
	"CSSNamespaceAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			namespace: true,
			prefix: true,
		},
	},
);
