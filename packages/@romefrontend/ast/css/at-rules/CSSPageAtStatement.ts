import {CSSRuleDeclaration, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// @page
export type CSSPageAtStatement = NodeBaseWithComments & {
	type: "CSSPageAtStatement";
	// TODO pseudo property
	declarations: Array<CSSRuleDeclaration>;
	// TODO should we have a field for each known valid @page property?
};

export const cssPageAtStatement = createBuilder<CSSPageAtStatement>(
	"CSSPageAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			declarations: true,
		},
	},
);
