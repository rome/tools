import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSNamespaceAtStatement = JSNodeBase & {
	type: "CSSNamespaceAtStatement";
};

export const cssNamespaceAtStatement = createBuilder<CSSNamespaceAtStatement>(
	"CSSNamespaceAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
