import {CSSIdentifierType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// .foo
export type CSSSelectorClass = NodeBaseWithComments & {
	type: "CSSSelectorClass";
	className: CSSIdentifierType;
};

export const cssSelectorClass = createBuilder<CSSSelectorClass>(
	"CSSSelectorClass",
	{
		bindingKeys: {},
		visitorKeys: {
			className: true,
		},
	},
);
