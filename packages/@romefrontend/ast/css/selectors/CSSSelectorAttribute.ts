import {CSSIdentifierType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// [foo]
// [foo=bar]
// [foo^=bar]
export interface CSSSelectorAttribute extends NodeBaseWithComments {
	type: "CSSSelectorAttribute";
	name: CSSIdentifierType;
	value: undefined | string;
	kind:
		| "exact"
		| "word"
		| "exact-or-hypen-prefix"
		| "prefix"
		| "suffix"
		| "contains";
	caseSensitive?: boolean;
}

export const cssSelectorAttribute = createBuilder<CSSSelectorAttribute>(
	"CSSSelectorAttribute",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
		},
	},
);
