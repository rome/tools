import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSNamespaceAtStatement = NodeBaseWithComments & {
	type: "CSSNamespaceAtStatement";
};

export const cssNamespaceAtStatement = createBuilder<CSSNamespaceAtStatement>(
	"CSSNamespaceAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
